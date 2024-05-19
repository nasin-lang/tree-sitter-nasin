use std::fmt;
use std::fmt::Display;

use super::instr::*;
use super::ty::*;
use super::value::*;
use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub name: String,
    pub globals: Vec<Global>,
    pub funcs: Vec<Func>,
    pub init: Option<ModuleInit>,
}

impl Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module \"{}\":", self.name,)?;

        for (i, global) in self.globals.iter().enumerate() {
            write!(f, "\n  global {}:", i)?;

            if let Some(Export { name }) = &global.export {
                write!(f, " (export \"{}\")", name)?;
            }

            write!(f, " {}", global.ty)?;

            if let Some(value) = &global.value {
                write!(f, " {}", value)?;
            }
        }

        for (i, func) in self.funcs.iter().enumerate() {
            write!(f, "\n  func {}:", i)?;

            if let Some(Export { name }) = &func.export {
                write!(f, " (export \"{}\")", name)?;
            }

            if let Some(Extern { name }) = &func.extn {
                write!(f, " (extern \"{}\")", name)?;
            }

            if func.params.len() > 0 {
                write!(f, " (params")?;
                for param in &func.params {
                    write!(f, " {}", param.ty)?;
                }
                write!(f, ")")?;
            }

            if func.ret.len() > 0 {
                write!(f, " (returns")?;
                for ret in &func.ret {
                    write!(f, " {}", ret)?;
                }
                write!(f, ")")?;
            }

            for (i, local) in func.locals.iter().enumerate() {
                write!(f, "\n       %{}: {}", i, local.ty)?;
            }

            if func.body.len() > 0 {
                write!(f, "\n{}", utils::indented(4, &func.body))?;
            }
        }

        if let Some(init) = &self.init {
            write!(f, "\n  init: ")?;

            for (i, local) in init.locals.iter().enumerate() {
                if i > 0 {
                    write!(f, "\n        ")?;
                }
                write!(f, "%{}: {}", i, local.ty)?;
            }

            write!(f, "\n{}", utils::indented(4, &init.body))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Global {
    pub ty: Type,
    pub value: Option<ConstValue>,
    pub export: Option<Export>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Func {
    pub params: Vec<Param>,
    pub ret: Vec<Type>,
    pub locals: Vec<Local>,
    pub body: Vec<Instr>,
    pub export: Option<Export>,
    pub extn: Option<Extern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleInit {
    pub locals: Vec<Local>,
    pub body: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Local {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Export {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extern {
    pub name: String,
}
