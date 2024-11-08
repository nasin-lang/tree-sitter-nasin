use std::borrow::Cow;
use std::collections::HashMap;

use cranelift_shim::{self as cl};
use derive_new::new;
use itertools::repeat_n;

use super::func::FuncCodegen;
use super::types;
use crate::{bytecode as b, utils};

#[derive(Debug, Clone)]
pub struct GlobalBinding<'a> {
    pub symbol_name: String,
    pub value: types::RuntimeValue<'a>,
    pub is_const: bool,
    pub is_entry_point: bool,
}

/// Describe all static data that is present in the module and which values they represent
#[derive(new)]
pub struct Globals<'a> {
    pub modules: &'a [b::Module],
    #[new(default)]
    pub data: HashMap<cl::DataId, cl::DataDescription>,
    #[new(default)]
    strings: HashMap<String, cl::DataId>,
    #[new(default)]
    tuples: HashMap<Vec<types::RuntimeValue<'a>>, cl::DataId>,
    #[new(default)]
    pub globals: HashMap<(usize, usize), GlobalBinding<'a>>,
}
impl<'a> Globals<'a> {
    pub fn get_global(&self, mod_idx: usize, idx: usize) -> Option<&GlobalBinding<'a>> {
        self.globals.get(&(mod_idx, idx))
    }

    pub fn insert_global<M: cl::Module>(
        &mut self,
        mod_idx: usize,
        idx: usize,
        obj_module: M,
    ) -> M {
        let global = &self.modules[mod_idx].globals[idx];

        // TODO: improve name mangling
        let symbol_name = format!("$global_{mod_idx}_{idx}");

        let (value, is_const, module) = utils::replace_with(self, |s| {
            let mut codegen =
                FuncCodegen::new(self.modules, None, obj_module, s, HashMap::new());

            for instr in &global.body {
                if let Some(value) = codegen.value_from_instr(instr, mod_idx) {
                    codegen.stack.push(value);
                } else {
                    let (data_id, module) = codegen
                        .globals
                        .create_writable_for_type(&global.ty, codegen.obj_module);
                    let value = types::RuntimeValue::new(
                        Cow::Borrowed(&global.ty),
                        data_id.into(),
                    );
                    return (codegen.globals, (value, false, module));
                }
            }

            assert!(codegen.stack.len() >= 1);
            (
                codegen.globals,
                (codegen.stack.pop(), true, codegen.obj_module),
            )
        });

        self.globals.insert(
            (mod_idx, idx),
            GlobalBinding {
                symbol_name,
                value,
                is_const,
                is_entry_point: global.is_entry_point,
            },
        );

        module
    }

    pub fn data_for_string<M: cl::Module>(
        &mut self,
        value: &str,
        mut obj_module: M,
    ) -> (cl::DataId, M) {
        if let Some(id) = self.strings.get(value) {
            return (*id, obj_module);
        }

        let data_id = obj_module.declare_anonymous_data(false, false).unwrap();
        let mut desc = cl::DataDescription::new();

        let mut bytes = vec![];

        match obj_module.isa().pointer_bytes() {
            1 => types::ValueSource::I8(value.len() as u8),
            2 => types::ValueSource::I16(value.len() as u16),
            4 => types::ValueSource::I32(value.len() as u32),
            8 => types::ValueSource::I64(value.len() as u64),
            _ => panic!("how many bytes?"),
        }
        .serialize(&mut bytes, obj_module.isa().endianness())
        .unwrap();

        bytes.extend(value.as_bytes());
        // Append a null terminator to avoid problems if used as a C string
        bytes.extend([0]);

        desc.define(bytes.into());
        obj_module.define_data(data_id, &desc).unwrap();

        self.data.insert(data_id, desc);
        self.strings.insert(value.to_string(), data_id);
        (data_id, obj_module)
    }

    pub fn data_for_array<M: cl::Module>(
        &mut self,
        mut values: Vec<types::RuntimeValue<'a>>,
        obj_module: M,
    ) -> (Option<cl::DataId>, M) {
        let len = match obj_module.isa().pointer_bytes() {
            1 => types::ValueSource::I8(values.len() as u8),
            2 => types::ValueSource::I16(values.len() as u16),
            4 => types::ValueSource::I32(values.len() as u32),
            8 => types::ValueSource::I64(values.len() as u64),
            _ => panic!("how many bytes?"),
        };
        values.insert(
            0,
            types::RuntimeValue::new(
                Cow::Owned(b::Type::new(b::TypeBody::USize, None)),
                len,
            ),
        );
        self.data_for_tuple(values, obj_module)
    }

    pub fn data_for_tuple<M: cl::Module>(
        &mut self,
        values: Vec<types::RuntimeValue<'a>>,
        mut obj_module: M,
    ) -> (Option<cl::DataId>, M) {
        if let Some(id) = self.tuples.get(&values) {
            return (Some(*id), obj_module);
        }

        let data_id = obj_module.declare_anonymous_data(false, false).unwrap();
        let mut desc = cl::DataDescription::new();

        let mut bytes = vec![];
        let mut included_datas = HashMap::new();

        for item in &values {
            if let types::ValueSource::Data(field_data_id) = item.src {
                let offset = bytes.len();
                bytes.extend(repeat_n(0u8, obj_module.isa().pointer_bytes() as usize));

                let field_gv = included_datas.entry(field_data_id).or_insert_with(|| {
                    obj_module.declare_data_in_data(field_data_id, &mut desc)
                });
                desc.write_data_addr(offset as u32, field_gv.clone(), 0);
            } else {
                if let Err(()) = item.serialize(&mut bytes, obj_module.isa().endianness())
                {
                    return (None, obj_module);
                }
            }
        }

        desc.define(bytes.into());
        obj_module.define_data(data_id, &desc).unwrap();

        self.data.insert(data_id, desc);
        self.tuples.insert(values, data_id);
        (Some(data_id), obj_module)
    }

    pub fn create_writable_for_type<M: cl::Module>(
        &mut self,
        ty: &b::Type,
        mut obj_module: M,
    ) -> (cl::DataId, M) {
        let data_id = obj_module.declare_anonymous_data(false, false).unwrap();
        let mut desc = cl::DataDescription::new();
        desc.define_zeroinit(types::get_size(ty, self.modules, &obj_module));
        obj_module.define_data(data_id, &desc).unwrap();

        self.data.insert(data_id, desc);
        (data_id, obj_module)
    }
}
