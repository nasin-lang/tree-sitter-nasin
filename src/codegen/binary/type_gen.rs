use cranelift_codegen::ir::types;
use cranelift_module::Module;

use crate::proto::m_ir;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn get_type(&self, ty: &m_ir::Type) -> types::Type {
        match ty.r#type.as_ref().unwrap() {
            m_ir::r#type::Type::Primitive(prim) => {
                match m_ir::PrimType::try_from(prim.clone()).unwrap() {
                    m_ir::PrimType::I8 => types::I8,
                    m_ir::PrimType::I16 => types::I16,
                    m_ir::PrimType::I32 => types::I32,
                    m_ir::PrimType::I64 => types::I64,
                    m_ir::PrimType::U8 => types::I8,
                    m_ir::PrimType::U16 => types::I16,
                    m_ir::PrimType::U32 => types::I32,
                    m_ir::PrimType::U64 => types::I64,
                    m_ir::PrimType::USize => self.poiter_type(),
                    m_ir::PrimType::F32 => types::F32,
                    m_ir::PrimType::F64 => types::F64,
                    m_ir::PrimType::Bool => types::I8,
                    m_ir::PrimType::Char => types::I8,
                }
            }
            m_ir::r#type::Type::Ambig(_) | m_ir::r#type::Type::Unknown(_) => {
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
