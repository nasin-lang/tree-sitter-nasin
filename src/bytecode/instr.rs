use std::fmt;
use std::fmt::Display;

use derive_new::new;

use super::{Loc, Type};
use crate::utils;

#[derive(Debug, Clone)]
pub enum InstrBody {
    Dup(usize),

    GetGlobal(usize, usize),
    GetProperty(String),
    GetField(String),
    GetMethod(String),
    CreateBool(bool),
    CreateNumber(Type, String),
    CreateString(String),
    CreateArray(Type, usize),
    CreateRecord(Type, Vec<String>),

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

    If(Type),
    Else,
    Loop(Type, usize),
    End,
    Continue,

    ArrayLen,
    ArrayPtr(u64),
    StrLen,
    StrPtr(u64),

    Type(Type),

    CompileError,
}

#[derive(Debug, Clone, new)]
pub struct Instr {
    pub body: InstrBody,
    pub loc: Loc,
}
impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.body {
            InstrBody::Dup(v) => write!(f, "dup ^{v}")?,
            InstrBody::GetGlobal(mod_idx, global_idx) => {
                write!(f, "get_global {mod_idx}-{global_idx}")?
            }
            InstrBody::GetProperty(prop) => write!(f, "get_property .{prop}")?,
            InstrBody::GetField(field) => write!(f, "get_field .{field}")?,
            InstrBody::GetMethod(field) => write!(f, "get_method .{field}")?,
            InstrBody::CreateBool(v) => write!(f, "create_bool {v}")?,
            InstrBody::CreateNumber(ty, v) => write!(f, "create_number {ty} {v}")?,
            InstrBody::CreateString(v) => {
                write!(f, "create_string {}", utils::encode_string_lit(v))?
            }
            InstrBody::CreateArray(ty, len) => write!(f, "create_array {ty} {len}")?,
            InstrBody::CreateRecord(ty, fields) => {
                write!(f, "create_record {ty}")?;
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
            InstrBody::If(ty) => write!(f, "if {ty}")?,
            InstrBody::Else => write!(f, "else")?,
            InstrBody::Loop(ty, n) => write!(f, "loop {ty} {n}")?,
            InstrBody::End => write!(f, "end")?,
            InstrBody::Continue => write!(f, "continue")?,
            InstrBody::ArrayLen => write!(f, "array_len")?,
            InstrBody::ArrayPtr(idx) => write!(f, "array_ptr {idx}")?,
            InstrBody::StrLen => write!(f, "str_len")?,
            InstrBody::StrPtr(idx) => write!(f, "str_ptr {idx}")?,
            InstrBody::Type(ty) => write!(f, "type {ty}")?,
            InstrBody::CompileError => write!(f, "compile_error")?,
        }
        write!(f, " {}", &self.loc)?;
        Ok(())
    }
}
