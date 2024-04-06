use cranelift_codegen::ir::types;
use cranelift_module::Module;

use crate::mir;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn get_type(&self, ty: &mir::Type) -> types::Type {
        match &ty {
            mir::Type::Primitive(prim) => match mir::PrimType::try_from(prim.clone()).unwrap() {
                mir::PrimType::I8 => types::I8,
                mir::PrimType::I16 => types::I16,
                mir::PrimType::I32 => types::I32,
                mir::PrimType::I64 => types::I64,
                mir::PrimType::U8 => types::I8,
                mir::PrimType::U16 => types::I16,
                mir::PrimType::U32 => types::I32,
                mir::PrimType::U64 => types::I64,
                mir::PrimType::USize => self.poiter_type(),
                mir::PrimType::F32 => types::F32,
                mir::PrimType::F64 => types::F64,
                mir::PrimType::Bool => types::I8,
                mir::PrimType::Char => types::I8,
            },
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
