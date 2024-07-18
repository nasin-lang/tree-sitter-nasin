use std::fmt;
use std::fmt::Display;

use super::instr::*;
use super::ty::*;
use crate::utils;
use crate::utils::SortedMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub typedefs: Vec<TypeDef>,
    pub globals: Vec<Global>,
    pub funcs: Vec<Func>,
}

impl Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module:")?;

        for (i, typedef) in self.typedefs.iter().enumerate() {
            write!(f, "\ntype {}:", i)?;

            match &typedef.body {
                TypeDefBody::Record(v) => {
                    write!(f, " (record")?;
                    for (name, field) in &v.fields {
                        write!(f, "\n    {}: (field {})", name, &field.ty)?;
                    }
                    write!(f, ")")?;
                }
            }
        }

        for (i, global) in self.globals.iter().enumerate() {
            write!(f, "\nglobal {}:", i)?;

            write!(f, " {}", global.ty)?;

            if global.body.len() > 0 {
                write!(f, "\n{}", utils::indented(4, &global.body))?;
            }
        }

        for (i, func) in self.funcs.iter().enumerate() {
            write!(f, "\nfunc {}:", i)?;

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

            write!(f, " (returns {})", &func.ret)?;

            if func.body.len() > 0 {
                write!(f, "\n{}", utils::indented(4, &func.body))?;
            }
        }

        write!(f, "\n")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeDef {
    pub body: TypeDefBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Global {
    pub ty: Type,
    pub body: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Func {
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Vec<Instr>,
    pub extn: Option<Extern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeDefBody {
    Record(RecordType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordType {
    pub fields: SortedMap<String, RecordTypeField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordTypeField {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extern {
    pub name: String,
}
