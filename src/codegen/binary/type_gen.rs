use cranelift_codegen::ir::types;
use cranelift_module::Module;

use crate::proto::lex;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn get_type(&self, ty: &lex::Type) -> types::Type {
        match ty.r#type.as_ref().unwrap() {
            lex::r#type::Type::Primitive(prim) => {
                match lex::PrimType::try_from(prim.clone()).unwrap() {
                    lex::PrimType::I8 => types::I8,
                    lex::PrimType::I16 => types::I16,
                    lex::PrimType::I32 => types::I32,
                    lex::PrimType::I64 => types::I64,
                    lex::PrimType::U8 => types::I8,
                    lex::PrimType::U16 => types::I16,
                    lex::PrimType::U32 => types::I32,
                    lex::PrimType::U64 => types::I64,
                    lex::PrimType::USize => self.poiter_type(),
                    lex::PrimType::F32 => types::F32,
                    lex::PrimType::F64 => types::F64,
                    lex::PrimType::Bool => types::I8,
                    lex::PrimType::Char => types::I8,
                }
            }
            lex::r#type::Type::Ambig(_) | lex::r#type::Type::Unknown(_) => {
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
