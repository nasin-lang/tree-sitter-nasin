use cranelift_codegen::ir::types;
use cranelift_module::Module;

use crate::mir;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn get_type(&self, ty: &mir::Type) -> types::Type {
        if ty.is_ambig() || ty.is_unknown() {
            panic!("Type must be resolved before codegen")
        }

        match &ty {
            mir::Type::I8 => return types::I8,
            mir::Type::I16 => return types::I16,
            mir::Type::I32 => return types::I32,
            mir::Type::I64 => return types::I64,
            mir::Type::U8 => return types::I8,
            mir::Type::U16 => return types::I16,
            mir::Type::U32 => return types::I32,
            mir::Type::U64 => return types::I64,
            mir::Type::USize => return self.poiter_type(),
            mir::Type::F32 => return types::F32,
            mir::Type::F64 => return types::F64,
            _ => {}
        }

        if ty.is_composite() {
            return self.poiter_type();
        }

        panic!("Type {} is not implemented", &ty);
    }
    fn get_size(&self, ty: &mir::Type) -> usize {
        if ty.is_ambig() || ty.is_unknown() {
            panic!("Type must be resolved before codegen")
        }

        if ty.is_primitive() {
            return self.get_type(ty).bytes() as usize;
        }

        match &ty {
            mir::Type::Array(array_ty) => {
                let item_size = self.get_size(&array_ty.item);
                let len = array_ty.len.expect("Type does not have a known length");
                return item_size * len;
            }
            _ => {}
        }

        panic!("Type {} is not implemented", &ty);
    }
}

impl<T: Module> TypeGen for T {
    fn poiter_type(&self) -> types::Type {
        self.target_config().pointer_type()
    }
}
