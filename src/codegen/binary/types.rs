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
        module: &b::Module,
        obj_module: &impl cl::Module,
    ) -> cl::Type {
        get_type(&self.ty, module, obj_module)
    }
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

        match self.src {
            ValueSource::I8(n) => bytes.push(n),
            ValueSource::I16(n) => serialize_number!(n),
            ValueSource::I32(n) => serialize_number!(n),
            ValueSource::I64(n) => serialize_number!(n),
            ValueSource::F32(n) => serialize_number!(n.to_float()),
            ValueSource::F64(n) => serialize_number!(n.to_float()),
            ValueSource::Value(_) | ValueSource::Data(_) | ValueSource::StackSlot(_) => {
                return Err(())
            }
        }

        Ok(())
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
    module: &b::Module,
) -> Vec<RuntimeValue<'a>> {
    let fields: HashMap<_, _> = fields.collect();

    match ty {
        b::Type::TypeRef(i) => match &module.typedefs[*i].body {
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
        b::Type::AnyNumber
        | b::Type::AnySignedNumber
        | b::Type::AnyFloat
        | b::Type::Bool
        | b::Type::I8
        | b::Type::I16
        | b::Type::I32
        | b::Type::I64
        | b::Type::U8
        | b::Type::U16
        | b::Type::U32
        | b::Type::U64
        | b::Type::USize
        | b::Type::F32
        | b::Type::F64
        | b::Type::String(_)
        | b::Type::Array(_)
        | b::Type::Inferred(_) => panic!("type is not a record type"),
    }
}

pub fn get_type(
    ty: &b::Type,
    module: &b::Module,
    obj_module: &impl cl::Module,
) -> cl::Type {
    match &ty {
        b::Type::Bool => cl::types::I8,
        b::Type::I8 => cl::types::I8,
        b::Type::I16 => cl::types::I16,
        b::Type::I32 => cl::types::I32,
        b::Type::I64 => cl::types::I64,
        b::Type::U8 => cl::types::I8,
        b::Type::U16 => cl::types::I16,
        b::Type::U32 => cl::types::I32,
        b::Type::U64 => cl::types::I64,
        b::Type::F32 => cl::types::F32,
        b::Type::F64 => cl::types::F64,
        b::Type::USize | b::Type::String(_) | b::Type::Array(_) => {
            obj_module.isa().pointer_type()
        }
        b::Type::TypeRef(i) => match &module.typedefs[*i].body {
            b::TypeDefBody::Record(_) => obj_module.isa().pointer_type(),
        },
        b::Type::AnyNumber
        | b::Type::AnySignedNumber
        | b::Type::AnyFloat
        | b::Type::Inferred(_) => panic!("Type must be resolved before codegen"),
    }
}
