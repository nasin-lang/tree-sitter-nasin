use std::fmt;
use std::fmt::Display;

use super::Type;
use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr {
    Dup(usize),

    GetGlobal(usize),
    GetField(String),
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

    Call(usize),

    If(Type),
    Else,
    Loop(Type, usize),
    End,
    Continue,

    CompileError,
}
impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instr::Dup(v) => write!(f, "dup ^{v}")?,
            Instr::GetGlobal(idx) => write!(f, "get_global {idx}")?,
            Instr::GetField(field) => write!(f, "get_field .{field}")?,
            Instr::CreateBool(v) => write!(f, "create_bool {v}")?,
            Instr::CreateNumber(ty, v) => write!(f, "create_number {ty} {v}")?,
            Instr::CreateString(v) => {
                write!(f, "create_string {}", utils::encode_string_lit(v))?
            }
            Instr::CreateArray(ty, len) => write!(f, "create_array {ty} {len}")?,
            Instr::CreateRecord(ty, fields) => {
                write!(f, "create_record {ty}")?;
                for field in fields {
                    write!(f, " .{field}")?;
                }
            }
            Instr::Add => write!(f, "add")?,
            Instr::Sub => write!(f, "sub")?,
            Instr::Mul => write!(f, "mul")?,
            Instr::Div => write!(f, "div")?,
            Instr::Mod => write!(f, "mod")?,
            Instr::Eq => write!(f, "eq")?,
            Instr::Neq => write!(f, "neq")?,
            Instr::Gt => write!(f, "gt")?,
            Instr::Gte => write!(f, "gte")?,
            Instr::Lt => write!(f, "lt")?,
            Instr::Lte => write!(f, "lte")?,
            Instr::Call(idx) => write!(f, "call {idx}")?,
            Instr::If(ty) => write!(f, "if {ty}")?,
            Instr::Else => write!(f, "else")?,
            Instr::Loop(ty, n) => write!(f, "loop {ty} {n}")?,
            Instr::End => write!(f, "end")?,
            Instr::Continue => write!(f, "continue")?,
            Instr::CompileError => write!(f, "compile_error")?,
        }
        Ok(())
    }
}
