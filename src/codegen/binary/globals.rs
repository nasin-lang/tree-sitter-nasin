use std::borrow::Cow;
use std::collections::HashMap;

use cl::Module;
use cranelift_shim::{self as cl};
use derive_new::new;
use itertools::repeat_n;

use super::func::FuncCodegen;
use super::types;
use crate::bytecode as b;

#[derive(Debug, Clone)]
pub struct GlobalBinding<'a> {
    pub symbol_name: String,
    pub value: types::RuntimeValue<'a>,
    pub ty: &'a b::Type,
    pub init: Option<&'a [b::Instr]>,
}

/// Describe all static data that is present in the module and which values they represent
#[derive(Debug, new)]
pub struct Globals<'a> {
    #[new(default)]
    strings: HashMap<&'a str, cl::DataId>,
    #[new(default)]
    tuples: HashMap<Vec<types::RuntimeValue<'a>>, cl::DataId>,
    #[new(default)]
    globals: Vec<GlobalBinding<'a>>,
}
impl<'a> Globals<'a> {
    pub fn get_global(&self, idx: usize) -> Option<&GlobalBinding<'a>> {
        self.globals.get(idx)
    }

    pub fn insert_global<'s>(
        &'s mut self,
        idx: usize,
        global: &'a b::Global,
        module: &'s mut cl::ObjectModule,
        typedefs: &'s [&'a b::TypeDef],
    ) where
        'a: 's,
    {
        assert!(idx == self.globals.len());

        // TODO: improve name mangling
        let symbol_name = format!("$global{idx}");

        let (value, is_const) = 'parse_static_body: {
            let mut codegen = FuncCodegen::new(None, module, self, &[], typedefs);

            for instr in &global.body {
                if let Some(value) = codegen.value_from_instr(instr) {
                    codegen.stack.push(value);
                } else {
                    let data_id = codegen
                        .globals
                        .create_writable_for_type(&global.ty, typedefs, module);
                    let value = types::RuntimeValue::new(
                        Cow::Borrowed(&global.ty),
                        types::ValueSource::Data(data_id),
                    );
                    break 'parse_static_body (value, false);
                }
            }

            assert!(codegen.stack.len() == 1);
            (codegen.stack.pop(), true)
        };

        self.globals.push(GlobalBinding {
            symbol_name,
            value,
            ty: &global.ty,
            init: if is_const { None } else { Some(&global.body) },
        });
    }

    pub fn data_for_string(
        &mut self,
        value: &'a str,
        module: &mut cl::ObjectModule,
    ) -> cl::DataId {
        if let Some(id) = self.strings.get(value) {
            return *id;
        }

        let data_id = module.declare_anonymous_data(false, false).unwrap();
        let mut desc = cl::DataDescription::new();

        let mut bytes = value.as_bytes().to_vec();
        // Append a null terminator to avoid problems if used as a C string
        bytes.extend([0]);

        desc.define(bytes.into());
        module.define_data(data_id, &desc).unwrap();

        self.strings.insert(value, data_id);
        data_id
    }

    pub fn data_for_tuple(
        &mut self,
        values: Vec<types::RuntimeValue<'a>>,
        module: &mut cl::ObjectModule,
    ) -> Option<cl::DataId> {
        if let Some(id) = self.tuples.get(&values) {
            return Some(*id);
        }

        let data_id = module.declare_anonymous_data(false, false).unwrap();
        let mut desc = cl::DataDescription::new();

        let mut bytes = vec![];
        let mut included_datas = HashMap::new();

        for item in &values {
            if let types::ValueSource::Data(field_data_id) = item.src {
                let offset = bytes.len();
                bytes.extend(repeat_n(0u8, module.isa().pointer_bytes() as usize));

                let field_gv = included_datas.entry(field_data_id).or_insert_with(|| {
                    module.declare_data_in_data(field_data_id, &mut desc)
                });
                desc.write_data_addr(offset as u32, field_gv.clone(), 0);
            } else {
                if let Err(()) = item.serialize(&mut bytes, module.isa().endianness()) {
                    return None;
                }
            }
        }

        desc.define(bytes.into());
        module.define_data(data_id, &desc).unwrap();

        self.tuples.insert(values, data_id);
        Some(data_id)
    }

    pub fn create_writable_for_type(
        &mut self,
        ty: &b::Type,
        typedefs: &[&b::TypeDef],
        module: &mut cl::ObjectModule,
    ) -> cl::DataId {
        let ptr = module.isa().pointer_bytes() as usize;

        let size = match ty {
            b::Type::String(s) => s.len.map_or(ptr, |len| len + 1),
            b::Type::Array(a) => a.len.map_or(ptr, |len| {
                len * types::get_type(&a.item, typedefs, module).bytes() as usize
            }),
            b::Type::TypeRef(i) => match &typedefs[*i as usize].body {
                b::TypeDefBody::Record(rec) => rec
                    .fields
                    .values()
                    .map(|field| {
                        types::get_type(&field.ty, typedefs, module).bytes() as usize
                    })
                    .sum(),
            },
            b::Type::Bool
            | b::Type::I8
            | b::Type::U8
            | b::Type::I16
            | b::Type::U16
            | b::Type::I32
            | b::Type::U32
            | b::Type::I64
            | b::Type::U64
            | b::Type::USize
            | b::Type::F32
            | b::Type::F64 => types::get_type(ty, typedefs, module).bytes() as usize,
            b::Type::AnyNumber
            | b::Type::AnySignedNumber
            | b::Type::AnyFloat
            | b::Type::Infer(_) => unreachable!(),
        };

        let data_id = module.declare_anonymous_data(false, false).unwrap();
        let mut desc = cl::DataDescription::new();
        desc.define_zeroinit(size);
        module.define_data(data_id, &desc).unwrap();

        data_id
    }
}
