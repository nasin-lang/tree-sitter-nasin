use std::fmt;
use std::fmt::Display;

use super::instr::*;
use super::ty::*;
use crate::utils::SortedMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub typedefs: Vec<TypeDef>,
    pub globals: Vec<Global>,
    pub funcs: Vec<Func>,
}
impl Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module:")?;

        for (i, typedef) in self.typedefs.iter().enumerate() {
            write!(f, "type {}:", i)?;

            match &typedef.body {
                TypeDefBody::Record(v) => {
                    write!(f, " (record")?;
                    for (name, field) in &v.fields {
                        write!(f, "\n    {}: (field {})", name, &field.ty)?;
                    }
                    write!(f, ")")?;
                }
            }

            writeln!(f)?;
        }

        for (i, global) in self.globals.iter().enumerate() {
            writeln!(f, "global {}: {}", i, global.ty)?;
            write_body(f, &global.body, 4)?;
        }

        for (i, func) in self.funcs.iter().enumerate() {
            write!(f, "func {}:", i)?;

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

            writeln!(f, " (returns {})", &func.ret)?;

            write_body(f, &func.body, 4)?;
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
    pub entry: bool,
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

fn write_body(
    f: &mut fmt::Formatter<'_>,
    body: &[Instr],
    mut indent: usize,
) -> fmt::Result {
    let mut buffer = vec![];
    for instr in body {
        if matches!(
            instr,
            Instr::Dup(_)
                | Instr::GetGlobal(_)
                | Instr::CreateNumber(_, _)
                | Instr::CreateBool(_)
                | Instr::CompileError
        ) {
            buffer.push(instr);
        } else if buffer.is_empty()
            || matches!(instr, Instr::CreateString(_) | Instr::Else | Instr::End)
        {
            for instr in buffer.drain(..) {
                writeln!(f, "{}{instr}", " ".repeat(indent))?;
            }

            if matches!(instr, Instr::Else | Instr::End) {
                indent -= 4;
            }

            writeln!(f, "{}{instr}", " ".repeat(indent))?;
        } else {
            write!(f, "{}", " ".repeat(indent))?;
            for instr in buffer.drain(..) {
                write!(f, "({instr}) ")?;
            }
            write!(f, "{instr}")?;
            writeln!(f)?;
        }

        if matches!(instr, Instr::If(..) | Instr::Else | Instr::Loop(..)) {
            indent += 4;
        }
    }
    for instr in buffer.drain(..) {
        writeln!(f, "{}{instr}", " ".repeat(indent))?;
    }
    Ok(())
}
