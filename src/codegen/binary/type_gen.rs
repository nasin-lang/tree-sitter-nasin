use cranelift_codegen::ir::{types, Endianness};
use cranelift_module::Module;

use crate::mir;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn endianness(&self) -> Endianness;

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
            mir::Type::String(string_ty) => {
                // Append a null terminator to avoid problems if used as a C string
                return string_ty.len.expect("Type does not have a known length") + 1;
            }
            mir::Type::Array(array_ty) => {
                let item_size = self.get_size(&array_ty.item);
                let len = array_ty.len.expect("Type does not have a known length");
                return item_size * len;
            }
            _ => {}
        }

        panic!("Type {} is not implemented", &ty);
    }

    fn serialize(&self, ty: &mir::Type, value: &mir::ConstValue) -> Vec<u8> {
        let size = self.get_size(ty);

        let bytes = match value {
            mir::ConstValue::Number(n) => {
                macro_rules! serialize_number {
                    ($ty:ty) => {{
                        let n = n.parse::<$ty>().expect(&format!(
                            "{} cannot be cast to {}",
                            n,
                            stringify!($ty)
                        ));

                        match self.endianness() {
                            Endianness::Little => n.to_le_bytes().to_vec(),
                            Endianness::Big => n.to_be_bytes().to_vec(),
                        }
                    }};
                }

                match ty {
                    mir::Type::I8 => serialize_number!(i8),
                    mir::Type::I16 => serialize_number!(i16),
                    mir::Type::I32 => serialize_number!(i32),
                    mir::Type::I64 => serialize_number!(i64),
                    mir::Type::U8 => serialize_number!(u8),
                    mir::Type::U16 => serialize_number!(u16),
                    mir::Type::U32 => serialize_number!(u32),
                    mir::Type::U64 => serialize_number!(u64),
                    mir::Type::USize => match self.poiter_type() {
                        types::I8 => serialize_number!(u8),
                        types::I16 => serialize_number!(u16),
                        types::I32 => serialize_number!(u32),
                        types::I64 => serialize_number!(u64),
                        _ => panic!("Pointer type {} is not allowed", self.poiter_type()),
                    },
                    mir::Type::F32 => serialize_number!(f32),
                    mir::Type::F64 => serialize_number!(f64),
                    _ => panic!("Number of type {} is not allowed", ty),
                }
            }
            mir::ConstValue::String(s) => {
                let mut bytes = s.as_bytes().to_vec();
                // Append a null terminator to avoid problems if used as a C string
                bytes.extend([0]);
                bytes
            }
            mir::ConstValue::Array(values) => {
                let mut bytes = Vec::with_capacity(size);

                let mir::Type::Array(array_type) = ty else {
                    panic!("Type must be an array");
                };

                for value in values {
                    bytes.extend(self.serialize(&array_type.item, value));
                }

                bytes
            }
        };

        assert_eq!(bytes.len(), size);

        bytes
    }
}

impl<T: Module> TypeGen for T {
    fn poiter_type(&self) -> types::Type {
        self.isa().pointer_type()
    }

    fn endianness(&self) -> Endianness {
        self.isa().endianness()
    }
}
