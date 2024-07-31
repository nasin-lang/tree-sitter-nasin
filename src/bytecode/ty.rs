use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::zip;

use derive_new::new;

use super::TypeDef;
use crate::utils::{self, SortedMap};

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
    Infer(InferType),
    String(StringType),
    Array(ArrayType),
    TypeRef(u16),
}

impl Type {
    pub fn unknown() -> Self {
        Type::Infer(InferType {
            properties: SortedMap::new(),
        })
    }

    pub fn is_unknown(&self) -> bool {
        if let Type::Infer(i) = self {
            return i.properties.is_empty();
        }
        false
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Type::Infer(_))
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

    pub fn property(&self, name: &str, typedefs: &[TypeDef]) -> Option<Type> {
        match self {
            Type::Infer(v) => v.properties.get(&name.to_string()).cloned(),
            Type::TypeRef(_) => todo!(),
            _ => None,
        }
    }

    pub fn intersection(&self, other: &Type, typedefs: &[TypeDef]) -> Option<Type> {
        if self.extends(other, typedefs) {
            return Some(self.clone());
        }
        if other.extends(self, typedefs) {
            return Some(other.clone());
        }

        if let (Type::Infer(a), Type::Infer(b)) = (self, other) {
            let mut properties = a.properties.clone();
            for (name, b_ty) in &b.properties {
                match properties.get_mut(name) {
                    Some(a_ty) => *a_ty = a_ty.intersection(b_ty, typedefs)?,
                    None => return None,
                }
            }
            return Some(Type::Infer(InferType { properties }));
        }

        None
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
            (Type::Infer(a), Type::Infer(b)) => {
                let mut props = SortedMap::new();
                for ((a_key, a_value), (b_key, b_value)) in
                    zip(&a.properties, &b.properties)
                {
                    if a_key != b_key {
                        todo!()
                    }
                    props.insert(
                        a_key.to_string(),
                        a_value.common_type(b_value, typedefs)?,
                    );
                }
                Some(Type::Infer(InferType { properties: props }))
            }
            (a, b) => a.intersection(b, typedefs),
        }
    }

    pub fn extends(&self, other: &Type, typedefs: &[TypeDef]) -> bool {
        if other.is_unknown() {
            return true;
        }
        match (self, other) {
            (Type::U8, Type::AnyNumber)
            | (Type::U16, Type::AnyNumber)
            | (Type::U32, Type::AnyNumber)
            | (Type::U64, Type::AnyNumber)
            | (Type::USize, Type::AnyNumber)
            | (Type::I8, Type::AnyNumber)
            | (Type::I16, Type::AnyNumber)
            | (Type::I32, Type::AnyNumber)
            | (Type::I64, Type::AnyNumber)
            | (Type::F32, Type::AnyNumber)
            | (Type::F64, Type::AnyNumber)
            | (Type::I8, Type::AnySignedNumber)
            | (Type::I16, Type::AnySignedNumber)
            | (Type::I32, Type::AnySignedNumber)
            | (Type::I64, Type::AnySignedNumber)
            | (Type::F32, Type::AnySignedNumber)
            | (Type::F64, Type::AnySignedNumber)
            | (Type::F32, Type::AnyFloat)
            | (Type::F64, Type::AnyFloat) => true,
            (Type::String(a), Type::String(b)) => a.len == b.len || b.len.is_none(),
            (Type::Array(a), Type::Array(b)) => {
                a.item.extends(&b.item, typedefs) && (a.len == b.len || b.len.is_none())
            }
            (a, Type::Infer(b)) => {
                b.properties
                    .iter()
                    .all(|(name, b_ty)| match a.property(name, typedefs) {
                        Some(a_ty) => a_ty.extends(b_ty, typedefs),
                        None => false,
                    })
            }
            (a, b) => a == b,
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
            Type::Infer(v) => {
                write!(f, "(infer")?;
                for (name, t) in &v.properties {
                    write!(f, "\n  (field {} {})", utils::encode_string_lit(name), t)?;
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
    pub properties: SortedMap<String, Type>,
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
