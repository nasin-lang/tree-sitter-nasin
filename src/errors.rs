use std::fmt;

use derive_more::{Display, From};
use derive_new::new;
use thiserror::Error;

use crate::{bytecode as b, context, utils};

#[derive(Debug, Clone, Error, new)]
pub struct Error<'a> {
    detail: ErrorDetail,
    ctx: &'a context::BuildContext<'a>,
    loc: b::Loc,
}
impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = self.loc.source_idx;
        let src = self.ctx.source(idx);

        let line = self.loc.start_line;
        let col = self.loc.start_col;
        writeln!(f, "{}:{line}:{col}", src.path.display())?;

        let num = format!("{line}");
        let line_content = src.content().line(line).expect("line should be valid");
        writeln!(f, "{} |", " ".repeat(num.len()))?;
        writeln!(f, "{num} | {line_content}",)?;
        writeln!(f, "{} | {}^", " ".repeat(num.len()), " ".repeat(col - 1))?;
        writeln!(f, "error: {}", self.detail)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Display, From)]
pub enum ErrorDetail {
    UnexpectedType(UnexpectedType),
    TypeMisatch(TypeMisatch),
}

#[derive(Debug, Clone, Display, new)]
#[display("Expected type {}, found {}", &expected.body, &actual.body)]
pub struct UnexpectedType {
    pub expected: b::Type,
    pub actual: b::Type,
}

#[derive(Debug, Clone, Display, new)]
#[display(
    "All results of the expression should have the same type\n{}",
    utils::indented(2, types.iter().map(|t| format!("- found {t}"))),
)]
pub struct TypeMisatch {
    pub types: Vec<b::Type>,
}
