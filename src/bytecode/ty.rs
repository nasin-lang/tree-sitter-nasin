use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

use derive_new::new;

use super::{TypeDef, TypeDefBody};
use crate::utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Bool,
    // FIXME: use interface/trait for this
    AnyNumber,
    // FIXME: use interface/trait for this
    AnySignedNumber,
    // FIXME: use interface/trait for this
    AnyFloat,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
    Inferred(InferType),
    String(StringType),
    Array(ArrayType),
    TypeRef(usize),
}

impl Type {
    pub fn unknown() -> Self {
        Type::Inferred(InferType {
            properties: utils::SortedMap::new(),
        })
    }

    pub fn is_unknown(&self) -> bool {
        if let Type::Inferred(i) = self {
            return i.properties.is_empty();
        }
        false
    }

    pub fn is_inferred(&self) -> bool {
        matches!(self, Type::Inferred(_))
    }

    pub fn is_composite(&self) -> bool {
        matches!(self, Type::String(_) | Type::Array(_))
    }

    pub fn is_primitive(&self) -> bool {
        self.is_bool() || self.is_number()
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Type::Bool)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Type::AnyNumber | Type::AnySignedNumber)
            || self.is_sint()
            || self.is_uint()
            || self.is_float()
    }

    pub fn is_int(&self) -> bool {
        self.is_sint() || self.is_uint()
    }

    pub fn is_sint(&self) -> bool {
        matches!(self, Type::I8 | Type::I16 | Type::I32 | Type::I64)
    }

    pub fn is_uint(&self) -> bool {
        matches!(
            self,
            Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::AnyFloat | Type::F32 | Type::F64)
    }

    pub fn property<'a>(
        &'a self,
        name: &str,
        typedefs: &'a [TypeDef],
    ) -> Option<&'a Type> {
        match self {
            Type::Inferred(v) => v.properties.get(name),
            Type::TypeRef(idx) => match &typedefs.get(*idx)?.body {
                TypeDefBody::Record(rec) => Some(&rec.fields.get(name)?.ty),
            },
            _ => None,
        }
    }

    pub fn intersection(&self, other: &Type, typedefs: &[TypeDef]) -> Option<Type> {
        macro_rules! unordered {
            ($a:pat, $b:pat) => {
                ($a, $b) | ($b, $a)
            };
        }
        macro_rules! number {
            ($var:ident $( , $gen:ident)*) => {
                unordered!(Type::$var, Type::AnyNumber $( | Type::$gen )*)
            };
        }
        match (self, other) {
            number!(U8) => Some(Type::U8),
            number!(U16) => Some(Type::U16),
            number!(U32) => Some(Type::U32),
            number!(U64) => Some(Type::U64),
            number!(USize) => Some(Type::USize),
            number!(I8, AnySignedNumber) => Some(Type::I8),
            number!(I16, AnySignedNumber) => Some(Type::I16),
            number!(I32, AnySignedNumber) => Some(Type::I32),
            number!(I64, AnySignedNumber) => Some(Type::I64),
            number!(F32, AnySignedNumber, AnyFloat) => Some(Type::F32),
            number!(F64, AnySignedNumber, AnyFloat) => Some(Type::F64),
            (Type::String(a), Type::String(b)) => {
                let len = match (&a.len, &b.len) {
                    (a_len, b_len) if a_len == b_len => a_len.clone(),
                    (Some(len), None) | (None, Some(len)) => Some(*len),
                    _ => return None,
                };
                Some(Type::String(StringType { len }))
            }
            (Type::Array(a), Type::Array(b)) => {
                let len = match (&a.len, &b.len) {
                    (a_len, b_len) if a_len == b_len => a_len.clone(),
                    (Some(len), None) | (None, Some(len)) => Some(*len),
                    _ => return None,
                };
                Some(Type::Array(ArrayType {
                    len,
                    item: a.item.intersection(&b.item, typedefs)?.into(),
                }))
            }
            (Type::Inferred(a), Type::Inferred(b)) => {
                let mut props = utils::SortedMap::new();
                let prop_names: HashSet<_> =
                    a.properties.keys().chain(b.properties.keys()).collect();
                for prop_name in prop_names {
                    let a_prop_ty = a.properties.get(prop_name);
                    let b_prop_ty = b.properties.get(prop_name);
                    let ty = match (a_prop_ty, b_prop_ty) {
                        (Some(a_prop), Some(b_prop)) => {
                            a_prop.intersection(b_prop, typedefs)?
                        }
                        (Some(prop), None) | (None, Some(prop)) => prop.clone(),
                        (None, None) => return None, // this should never happen
                    };
                    props.insert(prop_name.to_string(), ty);
                }
                Some(Type::Inferred(InferType { properties: props }))
            }
            unordered!(Type::Inferred(a), b) => {
                let has_all_properties = a.properties.iter().all(|(name, a_ty)| {
                    b.property(name, typedefs)
                        .is_some_and(|b_ty| a_ty.intersection(b_ty, typedefs).is_some())
                });
                if has_all_properties {
                    Some(b.clone())
                } else {
                    None
                }
            }
            (a, b) if a == b => Some(a.clone()),
            _ => None,
        }
    }

    pub fn common_type(&self, other: &Type, typedefs: &[TypeDef]) -> Option<Type> {
        if self == other {
            return Some(self.clone());
        }

        match (self, other) {
            (Type::String(a), Type::String(b)) => Some(Type::String(StringType {
                len: if a.len == b.len { a.len.clone() } else { None },
            })),
            (Type::Array(a), Type::Array(b)) => Some(Type::Array(ArrayType {
                item: a.item.common_type(&b.item, typedefs)?.into(),
                len: if a.len == b.len { a.len.clone() } else { None },
            })),
            (Type::Inferred(a), Type::Inferred(b)) => {
                let mut props = utils::SortedMap::new();
                let prop_names: HashSet<_> =
                    a.properties.keys().chain(b.properties.keys()).collect();
                for prop_name in prop_names {
                    let a_prop_ty = a.properties.get(prop_name);
                    let b_prop_ty = b.properties.get(prop_name);
                    let ty = match (a_prop_ty, b_prop_ty) {
                        (Some(a_prop), Some(b_prop)) => {
                            a_prop.common_type(b_prop, typedefs)?
                        }
                        (Some(prop), None) | (None, Some(prop)) => prop.clone(),
                        (None, None) => return None, // this should never happen
                    };
                    props.insert(prop_name.to_string(), ty);
                }
                Some(Type::Inferred(InferType { properties: props }))
            }
            (Type::Inferred(a), b) | (b, Type::Inferred(a)) => {
                for (prop_name, prop_ty) in &a.properties {
                    if prop_ty
                        .common_type(b.property(prop_name, typedefs)?, typedefs)
                        .is_none()
                    {
                        return None;
                    }
                }
                Some(b.clone())
            }
            (a, b) => a.intersection(b, typedefs),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Type::Bool => write!(f, "bool"),
            Type::AnyNumber => write!(f, "AnyNumber"),
            Type::AnySignedNumber => write!(f, "AnySignedNumber"),
            Type::AnyFloat => write!(f, "AnyFloat"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::USize => write!(f, "usize"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::Inferred(v) => {
                write!(f, "(infered")?;
                for (name, t) in &v.properties {
                    write!(f, " .{}: {}", name, t)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Type::String(v) => {
                if let Some(len) = v.len {
                    write!(f, "(string {})", len)?;
                } else {
                    write!(f, "string")?;
                }
                Ok(())
            }
            Type::Array(v) => {
                write!(f, "(array {}", v.item)?;
                if let Some(len) = v.len {
                    write!(f, " {}", len)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Type::TypeRef(i) => write!(f, "(type {i})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InferType {
    pub properties: utils::SortedMap<String, Type>,
}

impl InferType {
    pub fn new(properties: impl IntoIterator<Item = (String, Type)>) -> Self {
        Self {
            properties: properties.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct StringType {
    pub len: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
pub struct ArrayType {
    pub item: Box<Type>,
    pub len: Option<usize>,
}
