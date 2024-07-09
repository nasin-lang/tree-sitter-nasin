use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

use crate::utils;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Bool,
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
        Type::Infer(InferType { properties: vec![] })
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
        self.is_signed_int() || self.is_unsigned_int() || self.is_float()
    }

    pub fn is_signed_int(&self) -> bool {
        matches!(self, Type::I8 | Type::I16 | Type::I32 | Type::I64)
    }

    pub fn is_unsigned_int(&self) -> bool {
        matches!(
            self,
            Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Type::Bool => write!(f, "bool"),
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

#[derive(Debug, Clone, Eq, Hash)]
pub struct InferType {
    pub properties: Vec<(String, Type)>,
}

impl InferType {
    pub fn new(properties: impl IntoIterator<Item = (String, Type)>) -> Self {
        Self {
            properties: properties.into_iter().collect(),
        }
    }
}

impl PartialEq for InferType {
    fn eq(&self, other: &Self) -> bool {
        if self.properties.len() != other.properties.len() {
            return false;
        }

        let a_set: HashSet<_> = self.properties.iter().collect();
        let b_set: HashSet<_> = other.properties.iter().collect();

        a_set == b_set
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringType {
    pub len: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub item: Box<Type>,
    pub len: Option<usize>,
}

impl ArrayType {
    pub fn new(item: Type, len: Option<usize>) -> Self {
        Self {
            item: Box::new(item),
            len,
        }
    }
}
