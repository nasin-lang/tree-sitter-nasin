use cranelift_codegen::ir::types;
use cranelift_module::Module;

use crate::mir;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn get_type(&self, ty: &mir::Type) -> types::Type {
        match &ty {
            mir::Type::I8 => types::I8,
            mir::Type::I16 => types::I16,
            mir::Type::I32 => types::I32,
            mir::Type::I64 => types::I64,
            mir::Type::U8 => types::I8,
            mir::Type::U16 => types::I16,
            mir::Type::U32 => types::I32,
            mir::Type::U64 => types::I64,
            mir::Type::USize => self.poiter_type(),
            mir::Type::F32 => types::F32,
            mir::Type::F64 => types::F64,
            mir::Type::Ambig(_) | mir::Type::Unknown => {
                panic!("Type must be resolved before codegen")
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl<T: Module> TypeGen for T {
    fn poiter_type(&self) -> types::Type {
        self.target_config().pointer_type()
    }
}
