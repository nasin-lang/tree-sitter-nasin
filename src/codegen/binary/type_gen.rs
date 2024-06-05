use cranelift_codegen::ir::{types, Endianness};
use cranelift_module::{DataDescription, DataId, Linkage, Module};
use itertools::izip;

use crate::mir;

pub trait TypeGen {
    fn poiter_type(&self) -> types::Type;
    fn endianness(&self) -> Endianness;
    fn create_global_data(
        &mut self,
        symbol: Option<&str>,
        value: Option<&mir::ConstValue>,
        ty: &mir::Type,
        typedefs: &[mir::TypeDef],
    ) -> (DataId, DataDescription);

    fn is_pointer_type(&self, ty: &mir::Type, typedefs: &[mir::TypeDef]) -> bool {
        match ty {
            mir::Type::TypeRef(v) => {
                let typedef = typedefs.get(*v as usize).expect("Type ref out of bounds");
                match &typedef.body {
                    mir::TypeDefBody::Record(_) => true,
                }
            }
            _ => ty.is_composite(),
        }
    }

    fn get_type(&self, ty: &mir::Type, typedefs: &[mir::TypeDef]) -> types::Type {
        if ty.is_infer() {
            panic!("Type must be resolved before codegen")
        }

        if self.is_pointer_type(ty, typedefs) {
            return self.poiter_type();
        }

        match &ty {
            mir::Type::Bool => return types::I8,
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

        panic!("Type {} is not implemented", &ty);
    }

    fn get_size(
        &self,
        value: Option<&mir::ConstValue>,
        ty: &mir::Type,
        typedefs: &[mir::TypeDef],
    ) -> usize {
        if ty.is_infer() {
            panic!("Type must be resolved before codegen")
        }

        if ty.is_primitive() {
            return self.get_type(ty, typedefs).bytes() as usize;
        }

        match &ty {
            mir::Type::String(string_ty) => {
                // Append a null terminator to avoid problems if used as a C string
                1 + match (string_ty.len, value) {
                    (Some(len), Some(mir::ConstValue::String(v))) => {
                        assert_eq!(len, v.len());
                        len
                    }
                    (Some(len), None) => len,
                    (None, Some(mir::ConstValue::String(v))) => v.len(),
                    _ => panic!("Expected string value"),
                }
            }
            mir::Type::Array(array_ty) => {
                let item_size = self.get_type(&array_ty.item, typedefs).bytes() as usize;
                let len = match (array_ty.len, value) {
                    (Some(len), Some(mir::ConstValue::Array(v))) => {
                        assert_eq!(len, v.len());
                        len
                    }
                    (Some(len), None) => len,
                    (None, Some(mir::ConstValue::Array(v))) => v.len(),
                    _ => panic!("Array size mismatch"),
                };
                item_size * len
            }
            mir::Type::TypeRef(v) => {
                let typedef = typedefs.get(*v as usize).expect("Type ref out of bounds");
                match &typedef.body {
                    mir::TypeDefBody::Record(record_ty) => record_ty
                        .fields
                        .iter()
                        .map(|f| self.get_type(&f.ty, typedefs).bytes() as usize)
                        .sum(),
                }
            }
            _ => panic!("Type {} is not implemented", &ty),
        }
    }

    fn serialize_value(&self, value: &mir::ConstValue, ty: &mir::Type) -> Vec<u8> {
        match value {
            mir::ConstValue::Bool(b) => {
                vec![*b as u8]
            }
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
                        _ => panic!(
                            "Pointer type {} is not allowed",
                            self.poiter_type()
                        ),
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
            mir::ConstValue::Array(_) | mir::ConstValue::Record(_) => panic!("serialize_value doesn't handle Arrays and Records. Use create_global_data")
        }
    }
}

impl<T: Module> TypeGen for T {
    fn poiter_type(&self) -> types::Type {
        self.isa().pointer_type()
    }

    fn endianness(&self) -> Endianness {
        self.isa().endianness()
    }

    fn create_global_data(
        &mut self,
        symbol: Option<&str>,
        value: Option<&mir::ConstValue>,
        ty: &mir::Type,
        typedefs: &[mir::TypeDef],
    ) -> (DataId, DataDescription) {
        let size = self.get_size(value.clone(), ty, typedefs);

        let data_id = match symbol {
            Some(v) => self.declare_data(v, Linkage::Local, true, false),
            None => self.declare_anonymous_data(true, false),
        }
        .unwrap();

        let mut data = DataDescription::new();
        let mut data_relocs = Vec::new();

        match &value {
            Some(v) => {
                let mut bytes = Vec::with_capacity(size);

                match v {
                    mir::ConstValue::Bool(_)
                    | mir::ConstValue::Number(_)
                    | mir::ConstValue::String(_) => {
                        bytes.extend(self.serialize_value(v, ty))
                    }
                    mir::ConstValue::Array(values) => {
                        let mir::Type::Array(array_type) = ty else {
                            panic!("Type must be an array");
                        };

                        for value in values {
                            if value.is_primitive() {
                                bytes.extend(
                                    self.serialize_value(value, &array_type.item),
                                );
                            } else {
                                let (value_data_id, _) = self.create_global_data(
                                    None,
                                    Some(value),
                                    &array_type.item,
                                    typedefs,
                                );

                                let value_gv =
                                    self.declare_data_in_data(value_data_id, &mut data);

                                data_relocs.push((bytes.len() as u32, value_gv));
                                bytes.extend(
                                    [0u8].repeat(self.poiter_type().bytes() as usize),
                                );
                            }
                        }
                    }
                    mir::ConstValue::Record(values) => {
                        let mir::Type::TypeRef(type_ref) = ty else {
                            panic!("Type must be a record");
                        };
                        let mir::TypeDefBody::Record(record_type) = &typedefs
                            .get(*type_ref as usize)
                            .expect("TypeRef out of bounds")
                            .body
                        else {
                            panic!("Type must be a record");
                        };

                        for (value, field) in izip!(values, &record_type.fields) {
                            if value.is_primitive() {
                                bytes.extend(self.serialize_value(value, &field.ty));
                            } else {
                                let (value_data_id, _) = self.create_global_data(
                                    None,
                                    Some(value),
                                    &field.ty,
                                    typedefs,
                                );

                                let value_gv =
                                    self.declare_data_in_data(value_data_id, &mut data);

                                data_relocs.push((bytes.len() as u32, value_gv));
                                bytes.extend(
                                    [0u8].repeat(self.poiter_type().bytes() as usize),
                                );
                            }
                        }
                    }
                }

                assert_eq!(bytes.len(), size);

                data.define(bytes.into());
            }
            _ => {
                data.define_zeroinit(size);
            }
        }

        for (offset, gv) in data_relocs {
            data.write_data_addr(offset, gv, 0);
        }

        self.define_data(data_id, &data).unwrap();

        (data_id, data)
    }
}
