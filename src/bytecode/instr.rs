use std::fmt;
use std::fmt::Display;

use super::value::*;
use crate::utils;

pub type InstrBlock = Vec<Instr>;
pub type JumpCount = u8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr {
    Pull(RelativeValue),
    Dup(RelativeValue),
    Drop(RelativeValue),

    GetGlobal(GlobalIdx),
    GetField(String),
    CreateValue(ConstValue),
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

    If(InstrBlock, InstrBlock),
    Loop(InstrBlock),
    Continue(JumpCount),
}

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instr::Pull(v) => write!(f, "pull {v}")?,
            Instr::Dup(v) => write!(f, "dup {v}")?,
            Instr::Drop(v) => write!(f, "drop {v}")?,
            Instr::GetGlobal(idx) => write!(f, "get_global {idx}")?,
            Instr::GetField(field_name) => write!(f, "get_field {field_name}")?,
            Instr::CreateValue(v) => write!(f, "create_value {v}")?,
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
            Instr::If(then_bl, else_bl) => {
                write!(f, "if:")?;
                if then_bl.len() > 0 {
                    write!(f, "\n{}", utils::indented(4, then_bl))?;
                }
                if else_bl.len() > 0 {
                    write!(f, "\nelse:\n{}", utils::indented(4, else_bl))?;
                }
            }
            Instr::Loop(bl) => {
                write!(f, "loop:")?;
                if bl.len() > 0 {
                    write!(f, "\n{}", utils::indented(4, bl))?;
                }
            }
            Instr::Continue(c) => write!(f, "continue {c}")?,
        }
        Ok(())
    }
}
