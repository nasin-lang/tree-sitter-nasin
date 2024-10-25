use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;

use cranelift_shim::{self as cl, InstBuilder};
use derive_more::From;
use derive_new::new;

use crate::bytecode as b;

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct RuntimeValue<'a> {
    pub ty: Cow<'a, b::Type>,
    pub src: ValueSource,
}
impl RuntimeValue<'_> {
    pub fn native_type(
        &self,
        modules: &[b::Module],
        obj_module: &impl cl::Module,
    ) -> cl::Type {
        get_type(&self.ty, modules, obj_module)
    }
    pub fn serialize(
        &self,
        bytes: &mut Vec<u8>,
        endianess: cl::Endianness,
    ) -> Result<(), ()> {
        self.src.serialize(bytes, endianess)
    }
    pub fn add_to_func(
        &self,
        obj_module: &impl cl::Module,
        func: &mut cl::FunctionBuilder,
    ) -> cl::Value {
        match self.src {
            ValueSource::Value(v) => v,
            ValueSource::I8(n) => func.ins().iconst(cl::types::I8, n as i64),
            ValueSource::I16(n) => func.ins().iconst(cl::types::I16, n as i64),
            ValueSource::I32(n) => func.ins().iconst(cl::types::I32, n as i64),
            ValueSource::I64(n) => func.ins().iconst(cl::types::I64, unsafe {
                mem::transmute_copy::<u64, i64>(&n)
            }),
            ValueSource::F32(n) => func.ins().f32const(n.to_float()),
            ValueSource::F64(n) => func.ins().f64const(n.to_float()),
            ValueSource::Data(data_id) => {
                let field_gv = obj_module.declare_data_in_func(data_id, &mut func.func);
                func.ins()
                    .global_value(obj_module.isa().pointer_type(), field_gv.clone())
            }
            ValueSource::StackSlot(ss) => {
                func.ins()
                    .stack_addr(obj_module.isa().pointer_type(), ss, 0)
            }
            ValueSource::AppliedMethod(..) => todo!("function references"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub enum ValueSource {
    I8(u8),
    I16(u16),
    I32(u32),
    I64(u64),
    F32(F32Bits),
    F64(F64Bits),
    Value(cl::Value),
    Data(cl::DataId),
    StackSlot(cl::StackSlot),
    AppliedMethod(cl::Value, (usize, usize)),
}
impl ValueSource {
    pub fn serialize(
        &self,
        bytes: &mut Vec<u8>,
        endianess: cl::Endianness,
    ) -> Result<(), ()> {
        macro_rules! serialize_number {
            ($n:expr) => {
                match endianess {
                    cl::Endianness::Little => bytes.extend(($n).to_le_bytes()),
                    cl::Endianness::Big => bytes.extend(($n).to_be_bytes()),
                }
            };
        }

        match self {
            ValueSource::I8(n) => bytes.push(*n),
            ValueSource::I16(n) => serialize_number!(n),
            ValueSource::I32(n) => serialize_number!(n),
            ValueSource::I64(n) => serialize_number!(n),
            ValueSource::F32(n) => serialize_number!(n.to_float()),
            ValueSource::F64(n) => serialize_number!(n.to_float()),
            ValueSource::Value(..)
            | ValueSource::Data(..)
            | ValueSource::StackSlot(..)
            | ValueSource::AppliedMethod(..) => return Err(()),
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct F32Bits(u32);
impl F32Bits {
    pub fn to_float(&self) -> f32 {
        f32::from_bits(self.0)
    }
}
impl From<f32> for F32Bits {
    fn from(value: f32) -> Self {
        Self(value.to_bits())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct F64Bits(u64);
impl F64Bits {
    pub fn to_float(&self) -> f64 {
        f64::from_bits(self.0)
    }
}
impl From<f64> for F64Bits {
    fn from(value: f64) -> Self {
        Self(value.to_bits())
    }
}

pub fn tuple_from_record<'a>(
    fields: impl Iterator<Item = (&'a String, RuntimeValue<'a>)> + 'a,
    ty: &b::Type,
    modules: &[b::Module],
) -> Vec<RuntimeValue<'a>> {
    let fields: HashMap<_, _> = fields.collect();

    match &ty.body {
        b::TypeBody::TypeRef(i, j) => match &modules[*i].typedefs[*j].body {
            b::TypeDefBody::Record(rec) => rec
                .fields
                .keys()
                .map(|key| {
                    fields
                        .get(key)
                        .expect(&format!("missing field: {key}"))
                        .clone()
                })
                .collect(),
        },
        _ => panic!("type is not a record type"),
    }
}

pub fn get_type(
    ty: &b::Type,
    modules: &[b::Module],
    obj_module: &impl cl::Module,
) -> cl::Type {
    match &ty.body {
        b::TypeBody::Bool => cl::types::I8,
        b::TypeBody::I8 => cl::types::I8,
        b::TypeBody::I16 => cl::types::I16,
        b::TypeBody::I32 => cl::types::I32,
        b::TypeBody::I64 => cl::types::I64,
        b::TypeBody::U8 => cl::types::I8,
        b::TypeBody::U16 => cl::types::I16,
        b::TypeBody::U32 => cl::types::I32,
        b::TypeBody::U64 => cl::types::I64,
        b::TypeBody::F32 => cl::types::F32,
        b::TypeBody::F64 => cl::types::F64,
        b::TypeBody::USize
        | b::TypeBody::String(_)
        | b::TypeBody::Array(_)
        | b::TypeBody::Ptr(_) => obj_module.isa().pointer_type(),
        b::TypeBody::TypeRef(i, j) => match &modules[*i].typedefs[*j].body {
            b::TypeDefBody::Record(_) => obj_module.isa().pointer_type(),
        },
        b::TypeBody::AnyNumber
        | b::TypeBody::AnySignedNumber
        | b::TypeBody::AnyFloat
        | b::TypeBody::Inferred(_) => panic!("Type must be resolved before codegen"),
        b::TypeBody::AnyOpaque => panic!("anyopaque cannot be used directly"),
        b::TypeBody::Func(_) => todo!("first-class functions are not supported yet"),
    }
}

pub fn get_size(
    ty: &b::Type,
    modules: &[b::Module],
    obj_module: &impl cl::Module,
) -> usize {
    let ptr = obj_module.isa().pointer_bytes() as usize;

    match &ty.body {
        b::TypeBody::String(s) => s.len.map_or(ptr, |len| len + 1),
        b::TypeBody::Array(a) => a.len.map_or(ptr, |len| {
            len * get_type(&a.item, modules, obj_module).bytes() as usize
        }),
        b::TypeBody::TypeRef(i, j) => match &modules[*i].typedefs[*j].body {
            b::TypeDefBody::Record(rec) => rec
                .fields
                .values()
                .map(|field| get_type(&field.ty, modules, obj_module).bytes() as usize)
                .sum(),
        },
        b::TypeBody::Bool
        | b::TypeBody::I8
        | b::TypeBody::U8
        | b::TypeBody::I16
        | b::TypeBody::U16
        | b::TypeBody::I32
        | b::TypeBody::U32
        | b::TypeBody::I64
        | b::TypeBody::U64
        | b::TypeBody::USize
        | b::TypeBody::F32
        | b::TypeBody::F64
        | b::TypeBody::Ptr(_) => get_type(ty, modules, obj_module).bytes() as usize,
        b::TypeBody::AnyNumber
        | b::TypeBody::AnySignedNumber
        | b::TypeBody::AnyFloat
        | b::TypeBody::Inferred(_) => panic!("Type must be resolved before codegen"),
        b::TypeBody::AnyOpaque => panic!("anyopaque cannot be used directly"),
        b::TypeBody::Func(_) => todo!("first-class functions are not supported yet"),
    }
}
