use std::fmt;
use std::fmt::Display;

use crate::utils;

pub type RelativeValue = u16;
pub type GlobalIdx = u32;
pub type FuncIdx = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr {
    Dup(RelativeValue),

    GetGlobal(GlobalIdx),
    GetField(String),
    CreateBool(bool),
    CreateNumber(String),
    CreateString(String),
    CreateArray(u32),
    CreateRecord(Vec<String>),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,

    Call(FuncIdx),

    If,
    Else,
    Loop(u8),
    End,
    Continue,

    CompileError,
}

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instr::Dup(v) => write!(f, "dup {v}")?,
            Instr::GetGlobal(idx) => write!(f, "get_global {idx}")?,
            Instr::GetField(field_name) => write!(f, "get_field {field_name}")?,
            Instr::CreateBool(v) => write!(f, "create_bool {v}")?,
            Instr::CreateNumber(v) => write!(f, "create_number {v}")?,
            Instr::CreateString(v) => {
                write!(f, "create_string {}", utils::encode_string_lit(v))?
            }
            Instr::CreateArray(len) => write!(f, "create_array {len}")?,
            Instr::CreateRecord(fields) => {
                write!(f, "create_record")?;
                for name in fields {
                    write!(f, " {name}")?;
                }
            }
            Instr::Add => write!(f, "add")?,
            Instr::Sub => write!(f, "sub")?,
            Instr::Mul => write!(f, "mul")?,
            Instr::Div => write!(f, "div")?,
            Instr::Mod => write!(f, "mod")?,
            Instr::Pow => write!(f, "pow")?,
            Instr::Eq => write!(f, "eq")?,
            Instr::Neq => write!(f, "neq")?,
            Instr::Gt => write!(f, "gt")?,
            Instr::Gte => write!(f, "gte")?,
            Instr::Lt => write!(f, "lt")?,
            Instr::Lte => write!(f, "lte")?,
            Instr::Call(idx) => write!(f, "call {idx}")?,
            Instr::If => writeln!(f, "if")?,
            Instr::Else => writeln!(f, "else")?,
            Instr::Loop(n) => writeln!(f, "loop {n}")?,
            Instr::End => writeln!(f, "end")?,
            Instr::Continue => writeln!(f, "continue")?,
            Instr::CompileError => writeln!(f, "compile_error")?,
        }
        Ok(())
    }
}
