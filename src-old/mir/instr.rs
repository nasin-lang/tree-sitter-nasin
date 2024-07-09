use std::fmt;
use std::fmt::Display;

use super::value::*;
use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr {
    Bind(BindInstr),
    LoadGlobal(LoadGlobalInstr),
    StoreGlobal(StoreGlobalInstr),
    LoadField(LoadFieldInstr),
    CreateBool(CreateBoolInstr),
    CreateNumber(CreateNumberInstr),
    CreateString(CreateStringInstr),
    CreateData(CreateDataInstr),
    Add(BinOpInstr),
    Sub(BinOpInstr),
    Mul(BinOpInstr),
    Div(BinOpInstr),
    Mod(BinOpInstr),
    Pow(BinOpInstr),
    Eq(BinOpInstr),
    Neq(BinOpInstr),
    Gt(BinOpInstr),
    Lt(BinOpInstr),
    Gte(BinOpInstr),
    Lte(BinOpInstr),
    Call(CallInstr),
    If(IfInstr),
    Loop(LoopInstr),
    Return(ReturnInstr),
    Break(BreakInstr),
    Continue(ContinueInstr),
}

impl Instr {
    /// Returns true if the instruction unconditionally returns the current function
    pub fn returns(&self) -> bool {
        match self {
            Instr::Return(..) => true,
            Instr::If(v) => {
                let then_returns = v.then_body.iter().any(Instr::returns);
                let else_returns = v.else_body.iter().any(Instr::returns);
                if then_returns && else_returns {
                    return true;
                }
                if then_returns {
                    return v.else_body.iter().any(Instr::jumps);
                }
                if else_returns {
                    return v.then_body.iter().any(Instr::jumps);
                }
                return false;
            }
            _ => false,
        }
    }

    /// Returns true if the instruction unconditionally returns the current function, or
    /// continue or break an if or loop
    pub fn jumps(&self) -> bool {
        match self {
            Instr::Break(..) => true,
            Instr::Continue(..) => true,
            ins => ins.returns(),
        }
    }

    /// Replaces all references to the params with the specified values
    pub fn replace_params(&mut self, values: &[Value]) {
        match self {
            Instr::Bind(v) => v.value.replace_params(values),
            Instr::StoreGlobal(v) => v.value.replace_params(values),
            Instr::CreateData(v) => {
                for value in &mut v.items {
                    value.replace_params(values);
                }
            }
            Instr::Add(v)
            | Instr::Sub(v)
            | Instr::Mul(v)
            | Instr::Div(v)
            | Instr::Mod(v)
            | Instr::Pow(v)
            | Instr::Eq(v)
            | Instr::Neq(v)
            | Instr::Gt(v)
            | Instr::Lt(v)
            | Instr::Gte(v)
            | Instr::Lte(v) => {
                v.left.replace_params(values);
                v.right.replace_params(values);
            }
            Instr::Call(v) => {
                for value in &mut v.args {
                    value.replace_params(values);
                }
            }
            Instr::If(v) => {
                v.cond.replace_params(values);
                for instr in &mut v.then_body {
                    instr.replace_params(values);
                }
                for instr in &mut v.else_body {
                    instr.replace_params(values);
                }
            }
            Instr::Loop(v) => {
                for instr in &mut v.body {
                    instr.replace_params(values);
                }
            }
            Instr::Return(v) => {
                if let Some(value) = &mut v.value {
                    value.replace_params(values);
                }
            }
            Instr::Break(v) => {
                for value in &mut v.values {
                    value.replace_params(values);
                }
            }
            Instr::Continue(v) => {
                for value in &mut v.values {
                    value.replace_params(values);
                }
            }
            _ => {}
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instr::Bind(v) => {
                write!(f, "%{} = {}", v.target_idx, v.value)?;
            }
            Instr::LoadGlobal(v) => {
                write!(
                    f,
                    "%{} = load_global <global {}>",
                    v.target_idx, v.global_idx
                )?;
            }
            Instr::StoreGlobal(v) => {
                write!(f, "store_global <global {}>", v.global_idx)?;
                if let Some(field_idx) = v.field_idx {
                    write!(f, ".{}", field_idx)?;
                }
                write!(f, ", {}", v.value)?;
            }
            Instr::LoadField(v) => {
                write!(
                    f,
                    "%{} = load_field {}.{}",
                    v.target_idx, v.source, v.field_idx
                )?;
            }
            Instr::CreateBool(v) => {
                write!(f, "%{} = create_bool {}", v.target_idx, &v.value)?;
            }
            Instr::CreateNumber(v) => {
                write!(f, "%{} = create_number {}", v.target_idx, &v.value)?;
            }
            Instr::CreateString(v) => {
                write!(
                    f,
                    "%{} = create_string \"{}\"",
                    v.target_idx,
                    &v.value
                        .replace("\"", "\\\"")
                        .replace("\n", "\\n")
                        .replace("\r", "\\r")
                        .replace("\t", "\\t")
                        .replace("\\", "\\\\")
                )?;
            }
            Instr::CreateData(v) => {
                write!(f, "%{} = create_data", v.target_idx)?;
                for (i, item) in v.items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, " {}", item)?;
                }
            }
            Instr::Add(v) => {
                write!(f, "%{} = add {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Sub(v) => {
                write!(f, "%{} = sub {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Mul(v) => {
                write!(f, "%{} = mul {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Div(v) => {
                write!(f, "%{} = div {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Mod(v) => {
                write!(f, "%{} = mod {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Pow(v) => {
                write!(f, "%{} = pow {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Eq(v) => {
                write!(f, "%{} = eq {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Neq(v) => {
                write!(f, "%{} = neq {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Gt(v) => {
                write!(f, "%{} = gt {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Lt(v) => {
                write!(f, "%{} = lt {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Lte(v) => {
                write!(f, "%{} = lte {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Gte(v) => {
                write!(f, "%{} = gte {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Call(v) => {
                write!(f, "%{} = call <func {}>", v.target_idx, v.func_idx,)?;
                for arg in &v.args {
                    write!(f, ", {}", arg)?;
                }
            }
            Instr::If(v) => {
                if v.target_idx_list.len() > 0 {
                    for (i, idx) in v.target_idx_list.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "%{}", idx)?;
                    }
                    write!(f, " = ")?;
                }

                write!(
                    f,
                    "(if {}\n  (then\n{})\n  (else\n{}))",
                    v.cond,
                    utils::indented(4, &v.then_body),
                    utils::indented(4, &v.else_body)
                )?;
            }
            Instr::Loop(v) => {
                if v.target_idx_list.len() > 0 {
                    for (i, idx) in v.target_idx_list.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "%{}", idx)?;
                    }
                    write!(f, " = ")?;
                }

                write!(f, "(loop")?;

                if v.updating_idx_list.len() > 0 {
                    write!(f, " (updates")?;
                    for (i, idx) in v.updating_idx_list.iter().enumerate() {
                        if i > 0 {
                            write!(f, ",")?;
                        }
                        write!(f, " %{idx}")?;
                    }
                    write!(f, ")")?;
                }

                write!(f, "\n{})", utils::indented(2, &v.body))?;
            }
            Instr::Return(v) => {
                write!(f, "return")?;
                if let Some(value) = &v.value {
                    write!(f, " {value}")?;
                }
            }
            Instr::Break(v) => {
                write!(f, "break {}", v.count)?;
                for (i, value) in v.values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, " {value}")?;
                }
            }
            Instr::Continue(v) => {
                write!(f, "continue {}", v.count)?;
                for (i, value) in v.values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, " {value}")?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindInstr {
    pub target_idx: u32,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadGlobalInstr {
    pub target_idx: u32,
    pub global_idx: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreGlobalInstr {
    pub global_idx: u32,
    pub field_idx: Option<u32>,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadFieldInstr {
    pub target_idx: u32,
    pub field_idx: u32,
    pub source: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateBoolInstr {
    pub target_idx: u32,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateNumberInstr {
    pub target_idx: u32,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateStringInstr {
    pub target_idx: u32,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDataInstr {
    pub target_idx: u32,
    pub items: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinOpInstr {
    pub target_idx: u32,
    pub left: Value,
    pub right: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallInstr {
    pub target_idx: u32,
    pub func_idx: u32,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnInstr {
    pub value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakInstr {
    pub count: u16,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueInstr {
    pub count: u16,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfInstr {
    pub target_idx_list: Vec<u32>,
    pub cond: Value,
    pub then_body: Vec<Instr>,
    pub else_body: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopInstr {
    pub target_idx_list: Vec<u32>,
    pub updating_idx_list: Vec<u32>,
    pub body: Vec<Instr>,
}
