use cranelift_codegen::ir::{FuncRef, Function, InstBuilder, MemFlags, Value};
use cranelift_frontend::FunctionBuilder;
use cranelift_module::{DataId, FuncId, Module};

use crate::proto::m_ir;

use super::type_gen::TypeGen;

// Cranelift's variables are for mutable primitives, immutable primitive can just use values.
// Aggregate values can be either stack slots, if they have known length and are never moved, or
// heap allocated if else, regardless of mutability
/// Holds a Crenelift representation of a value, which can be different depending on type,
/// mutability, scope and sharing
#[derive(Debug, Clone)]
pub enum VariableRef {
    ImmPrimitive(Value),
    GlobalPrimitive(DataId, m_ir::Type),
    ExternalFunc(FuncId, Function, m_ir::FnType),
    GlobalFunc(FuncId, Function, m_ir::FnType),
    // TODO: mutable and aggregate types
}

impl VariableRef {
    pub fn get_value<M>(&self, module: &mut M, builder: &mut FunctionBuilder) -> Value
    where
        M: Module,
    {
        match self {
            Self::ImmPrimitive(value) => value.clone(),
            Self::GlobalPrimitive(data_id, ty) => {
                let ty = module.get_type(ty);

                let data = module.declare_data_in_func(data_id.clone(), &mut builder.func);
                let ptr = builder.ins().global_value(module.poiter_type(), data);

                // It would probably better to hold the pointer and deref it lazily instead
                builder.ins().load(ty, MemFlags::new(), ptr, 0)
            }
            Self::GlobalFunc(..) | Self::ExternalFunc(..) => {
                panic!("Cannot get value from a function")
            }
        }
    }

    pub fn get_ptr<M>(&self, module: &mut M, builder: &mut FunctionBuilder) -> Value
    where
        M: Module,
    {
        match self {
            Self::GlobalPrimitive(data_id, ..) => {
                let data = module.declare_data_in_func(data_id.clone(), &mut builder.func);
                builder.ins().global_value(module.poiter_type(), data)
            }
            Self::GlobalFunc(..) => todo!(),
            Self::ExternalFunc(..) => todo!(),
            Self::ImmPrimitive(..) => {
                panic!("Cannot get pointer for a local primitive that might only be in a register")
            }
        }
    }

    pub fn get_func_ref<M>(&self, module: &mut M, builder: &mut FunctionBuilder) -> FuncRef
    where
        M: Module,
    {
        match self {
            Self::GlobalFunc(func_id, ..) => {
                let func = &mut builder.func;
                module.declare_func_in_func(func_id.clone(), func)
            }
            Self::ExternalFunc(func_id, ..) => {
                let func = &mut builder.func;
                module.declare_func_in_func(func_id.clone(), func)
            }
            Self::ImmPrimitive(..) => {
                panic!("Variable is not a function",);
            }
            Self::GlobalPrimitive(..) => {
                panic!("Variable is not a function",);
            }
        }
    }
}
