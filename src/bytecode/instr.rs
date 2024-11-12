use std::fmt;
use std::fmt::Display;

use derive_new::new;
use itertools::Itertools;

use super::{Loc, Type, ValueIdx};
use crate::utils;

#[derive(Debug, Clone)]
pub enum InstrBody {
    GetGlobal(usize, usize),
    GetProperty(ValueIdx, String),
    GetField(ValueIdx, String),
    GetMethod(ValueIdx, String),
    CreateBool(bool),
    CreateNumber(String),
    CreateString(String),
    CreateArray(Vec<ValueIdx>),
    CreateRecord(utils::SortedMap<String, ValueIdx>),

    Add(ValueIdx, ValueIdx),
    Sub(ValueIdx, ValueIdx),
    Mul(ValueIdx, ValueIdx),
    Div(ValueIdx, ValueIdx),
    Mod(ValueIdx, ValueIdx),

    Eq(ValueIdx, ValueIdx),
    Neq(ValueIdx, ValueIdx),
    Gt(ValueIdx, ValueIdx),
    Gte(ValueIdx, ValueIdx),
    Lt(ValueIdx, ValueIdx),
    Lte(ValueIdx, ValueIdx),
    Not(ValueIdx),

    Call(usize, usize, Vec<ValueIdx>),
    IndirectCall(ValueIdx, Vec<ValueIdx>),

    If(ValueIdx, ValueIdx),
    Else,
    Loop(Vec<ValueIdx>, ValueIdx),
    End,
    Continue(Vec<ValueIdx>),

    ArrayLen(ValueIdx),
    ArrayPtr(ValueIdx, u64),
    StrLen(ValueIdx),
    StrPtr(ValueIdx, u64),

    Type(ValueIdx, Type),

    CompileError,
}
impl Display for InstrBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrBody::GetGlobal(mod_idx, global_idx) => {
                write!(f, "get_global {mod_idx}-{global_idx}")?
            }
            InstrBody::GetProperty(v, prop) => write!(f, "get_property v{v} .{prop}")?,
            InstrBody::GetField(v, field) => write!(f, "get_field v{v} .{field}")?,
            InstrBody::GetMethod(v, field) => write!(f, "get_method v{v} .{field}")?,
            InstrBody::CreateBool(v) => write!(f, "create_bool {v}")?,
            InstrBody::CreateNumber(v) => write!(f, "create_number {v}")?,
            InstrBody::CreateString(v) => {
                write!(f, "create_string {}", utils::encode_string_lit(v))?
            }
            InstrBody::CreateArray(vs) => {
                write!(f, "create_array")?;
                for v in vs {
                    write!(f, " v{v}")?;
                }
            }
            InstrBody::CreateRecord(fields) => {
                write!(f, "create_record")?;
                for (name, v) in fields {
                    write!(f, " .{name}=v{v}")?;
                }
            }
            InstrBody::Add(a, b) => write!(f, "add {a} {b}")?,
            InstrBody::Sub(a, b) => write!(f, "sub {a} {b}")?,
            InstrBody::Mul(a, b) => write!(f, "mul {a} {b}")?,
            InstrBody::Div(a, b) => write!(f, "div {a} {b}")?,
            InstrBody::Mod(a, b) => write!(f, "mod {a} {b}")?,
            InstrBody::Eq(a, b) => write!(f, "eq {a} {b}")?,
            InstrBody::Neq(a, b) => write!(f, "neq {a} {b}")?,
            InstrBody::Gt(a, b) => write!(f, "gt {a} {b}")?,
            InstrBody::Gte(a, b) => write!(f, "gte {a} {b}")?,
            InstrBody::Lt(a, b) => write!(f, "lt {a} {b}")?,
            InstrBody::Lte(a, b) => write!(f, "lte {a} {b}")?,
            InstrBody::Not(v) => write!(f, "not {v}")?,
            InstrBody::Call(mod_idx, func_idx, args) => {
                write!(f, "call {mod_idx}-{func_idx}")?;
                for arg in args {
                    write!(f, " v{arg}")?;
                }
            }
            InstrBody::IndirectCall(v, args) => {
                write!(f, "indirect_call v{v}")?;
                for arg in args {
                    write!(f, " v{arg}")?;
                }
            }
            InstrBody::If(v, target) => write!(f, "if v{v} -> v{target}")?,
            InstrBody::Else => write!(f, "else")?,
            InstrBody::Loop(vs, target) => {
                write!(f, "loop")?;
                for v in vs {
                    write!(f, " v{v}")?;
                }
                write!(f, " -> v{target}")?;
            }
            InstrBody::End => write!(f, "end")?,
            InstrBody::Continue(vs) => {
                write!(f, "continue")?;
                for v in vs {
                    write!(f, " v{v}")?;
                }
            }
            InstrBody::ArrayLen(v) => write!(f, "array_len v{v}")?,
            InstrBody::ArrayPtr(v, idx) => write!(f, "array_ptr v{v} {idx}")?,
            InstrBody::StrLen(v) => write!(f, "str_len v{v}")?,
            InstrBody::StrPtr(v, idx) => write!(f, "str_ptr v{v} {idx}")?,
            InstrBody::Type(v, ty) => write!(f, "type v{v} {ty}")?,
            InstrBody::CompileError => write!(f, "compile_error")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct Instr {
    pub body: InstrBody,
    pub loc: Loc,
    #[new(default)]
    pub results: Vec<ValueIdx>,
}
impl Instr {
    pub fn with_results(mut self, results: impl IntoIterator<Item = ValueIdx>) -> Self {
        self.results = results.into_iter().collect();
        self
    }
}
impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.results.len() > 0 {
            write!(
                f,
                "{} = ",
                self.results.iter().map(|n| format!("v{n}")).join(", ")
            )?;
        }
        write!(f, "{} {}", &self.body, &self.loc)?;
        Ok(())
    }
}
