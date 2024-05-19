use std::fmt;
use std::fmt::Display;

use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Local(u32),
    Param(u32),
}

impl Value {
    /// If the value is a param, replace if with the corresponding value in the specified
    /// list
    pub fn replace_params(&mut self, values: &[Value]) {
        if let Value::Param(idx) = self {
            *self = values[*idx as usize].clone();
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Value::Local(idx) => {
                write!(f, "%{}", idx)?;
            }
            Value::Param(idx) => {
                write!(f, "<param {}>", idx)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstValue {
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<ConstValue>),
}

impl Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ConstValue::Bool(b) => {
                write!(f, "{}", b)?;
            }
            ConstValue::Number(n) => {
                write!(f, "{}", n)?;
            }
            ConstValue::String(s) => {
                write!(f, "\"{}\"", utils::encode_string_lit(s))?;
            }
            ConstValue::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}
