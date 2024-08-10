use std::path::PathBuf;
use std::{cmp, fmt};

use derive_more::Display;
use derive_new::new;
use tree_sitter as ts;

use super::instr::*;
use super::ty::*;
use crate::utils;
use crate::utils::SortedMap;

#[derive(Debug, Clone)]
pub struct Module {
    pub typedefs: Vec<TypeDef>,
    pub globals: Vec<Global>,
    pub funcs: Vec<Func>,
    pub sources: Vec<Source>,
}
impl Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module:")?;

        for (i, typedef) in self.typedefs.iter().enumerate() {
            write!(f, "type {i} {}:", typedef.loc)?;

            match &typedef.body {
                TypeDefBody::Record(v) => {
                    write!(f, " (record")?;
                    for (name, field) in &v.fields {
                        write!(f, "\n    {name}: {field}")?;
                    }
                    write!(f, ")")?;
                }
            }

            writeln!(f)?;
        }

        for (i, global) in self.globals.iter().enumerate() {
            writeln!(f, "global {i} {}: {}", global.loc, global.ty)?;
            write_body(f, &global.body, 4)?;
        }

        for (i, func) in self.funcs.iter().enumerate() {
            write!(f, "func {i} {}:", func.loc)?;

            if let Some(Extern { name }) = &func.extn {
                write!(f, " (extern {})", utils::encode_string_lit(name))?;
            }

            if func.params.len() > 0 {
                write!(f, " (params")?;
                for param in &func.params {
                    write!(f, " ({} {})", param.ty, param.loc)?;
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

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub body: TypeDefBody,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct Global {
    pub ty: Type,
    pub body: Vec<Instr>,
    pub is_entry_point: bool,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Vec<Instr>,
    pub extn: Option<Extern>,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub enum TypeDefBody {
    Record(RecordType),
}

#[derive(Debug, Clone)]
pub struct RecordType {
    pub fields: SortedMap<String, RecordField>,
}

#[derive(Debug, Clone, Display, new)]
#[display("{ty} {loc}")]
pub struct RecordField {
    pub name: RecordFieldName,
    pub ty: Type,
    pub loc: Loc,
}

#[derive(Debug, Clone, new)]
pub struct RecordFieldName {
    pub name: String,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ty: Type,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct Extern {
    pub name: String,
}

#[derive(Debug, Clone, new)]
pub struct Source {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display(":{start_line}:{start_col}-{end_line}:{end_col}")]
pub struct Loc {
    pub source: usize,
    pub start_line: usize,
    pub start_col: usize,
    pub start_byte: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub end_byte: usize,
}
impl Loc {
    pub fn from_node(source: usize, node: &ts::Node) -> Loc {
        let start_pos = node.start_position();
        let end_pos = node.end_position();
        Loc {
            source,
            start_line: start_pos.row + 1,
            start_col: start_pos.column + 1,
            start_byte: node.start_byte(),
            end_line: end_pos.row + 1,
            end_col: end_pos.column + 1,
            end_byte: node.end_byte(),
        }
    }
    pub fn merge(&self, other: &Loc) -> Loc {
        assert!(self.source == other.source);
        Loc {
            source: self.source,
            start_byte: cmp::min(self.start_byte, other.start_byte),
            start_line: cmp::min(self.start_line, other.start_line),
            start_col: cmp::min(self.start_col, other.start_col),
            end_byte: cmp::max(self.end_byte, other.end_byte),
            end_line: cmp::max(self.end_line, other.end_line),
            end_col: cmp::max(self.end_col, other.end_col),
        }
    }
}

fn write_body(
    f: &mut fmt::Formatter<'_>,
    body: &[Instr],
    mut indent: usize,
) -> fmt::Result {
    for instr in body {
        if matches!(&instr.body, InstrBody::Else | InstrBody::End) {
            indent -= 4;
        }

        writeln!(f, "{}{instr}", " ".repeat(indent))?;

        if matches!(
            &instr.body,
            InstrBody::If(..) | InstrBody::Else | InstrBody::Loop(..)
        ) {
            indent += 4;
        }
    }
    Ok(())
}
