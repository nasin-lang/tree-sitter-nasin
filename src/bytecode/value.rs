use std::fmt;
use std::fmt::Display;

use itertools::Itertools;

use crate::utils;

pub type GlobalIdx = u32;
pub type FuncIdx = u32;
pub type RelativeValue = u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveValue {
    Bool(bool),
    Number(String),
}

impl Display for PrimitiveValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PrimitiveValue::Bool(b) => {
                write!(f, "{}", b)?;
            }
            PrimitiveValue::Number(n) => {
                write!(f, "{}", n)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstValue {
    Primitive(PrimitiveValue),
    String(String),
    Array(Vec<PrimitiveValue>),
    Record(Vec<(String, PrimitiveValue)>),
}

impl ConstValue {
    pub fn is_primitive(&self) -> bool {
        matches!(self, ConstValue::Primitive(_))
    }

    pub fn is_composite(&self) -> bool {
        !self.is_primitive()
    }
}

impl Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ConstValue::Primitive(v) => write!(f, "{v}")?,
            ConstValue::String(s) => {
                write!(f, "{}", utils::encode_string_lit(s))?;
            }
            ConstValue::Array(items) => {
                write!(f, "[{}]", items.iter().join(", "))?;
            }
            ConstValue::Record(fields) => {
                write!(f, "{{")?;
                for (name, field) in fields {
                    write!(f, "  {name} = {field}")?;
                }
                write!(f, "\n}}")?;
            }
        }
        Ok(())
    }
}
