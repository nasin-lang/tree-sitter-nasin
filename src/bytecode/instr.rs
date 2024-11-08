use std::fmt;
use std::fmt::Display;

use derive_new::new;
use itertools::Itertools;

use super::{Loc, Type, ValueIdx};
use crate::utils;

#[derive(Debug, Clone)]
pub enum InstrBody {
    Dup(usize),

    GetGlobal(usize, usize),
    GetProperty(String),
    GetField(String),
    GetMethod(String),
    CreateBool(bool),
    CreateNumber(String),
    CreateString(String),
    CreateArray(usize),
    CreateRecord(Vec<String>),

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Not,

    Call(usize, usize),
    IndirectCall(usize),

    If(ValueIdx),
    Else,
    Loop(usize),
    End,
    Continue,

    ArrayLen,
    ArrayPtr(u64),
    StrLen,
    StrPtr(u64),

    Type(Type),

    CompileError,
}
impl Display for InstrBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrBody::Dup(v) => write!(f, "dup ^{v}")?,
            InstrBody::GetGlobal(mod_idx, global_idx) => {
                write!(f, "get_global {mod_idx}-{global_idx}")?
            }
            InstrBody::GetProperty(prop) => write!(f, "get_property .{prop}")?,
            InstrBody::GetField(field) => write!(f, "get_field .{field}")?,
            InstrBody::GetMethod(field) => write!(f, "get_method .{field}")?,
            InstrBody::CreateBool(v) => write!(f, "create_bool {v}")?,
            InstrBody::CreateNumber(v) => write!(f, "create_number {v}")?,
            InstrBody::CreateString(v) => {
                write!(f, "create_string {}", utils::encode_string_lit(v))?
            }
            InstrBody::CreateArray(len) => write!(f, "create_array {len}")?,
            InstrBody::CreateRecord(fields) => {
                write!(f, "create_record")?;
                for field in fields {
                    write!(f, " .{field}")?;
                }
            }
            InstrBody::Add => write!(f, "add")?,
            InstrBody::Sub => write!(f, "sub")?,
            InstrBody::Mul => write!(f, "mul")?,
            InstrBody::Div => write!(f, "div")?,
            InstrBody::Mod => write!(f, "mod")?,
            InstrBody::Eq => write!(f, "eq")?,
            InstrBody::Neq => write!(f, "neq")?,
            InstrBody::Gt => write!(f, "gt")?,
            InstrBody::Gte => write!(f, "gte")?,
            InstrBody::Lt => write!(f, "lt")?,
            InstrBody::Lte => write!(f, "lte")?,
            InstrBody::Not => write!(f, "not")?,
            InstrBody::Call(mod_idx, func_idx) => write!(f, "call {mod_idx}-{func_idx}")?,
            InstrBody::IndirectCall(n) => write!(f, "indirect_call {n}")?,
            InstrBody::If(v) => write!(f, "if -> v{v}")?,
            InstrBody::Else => write!(f, "else")?,
            InstrBody::Loop(n) => write!(f, "loop {n}")?,
            InstrBody::End => write!(f, "end")?,
            InstrBody::Continue => write!(f, "continue")?,
            InstrBody::ArrayLen => write!(f, "array_len")?,
            InstrBody::ArrayPtr(idx) => write!(f, "array_ptr {idx}")?,
            InstrBody::StrLen => write!(f, "str_len")?,
            InstrBody::StrPtr(idx) => write!(f, "str_ptr {idx}")?,
            InstrBody::Type(ty) => write!(f, "type {ty}")?,
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
