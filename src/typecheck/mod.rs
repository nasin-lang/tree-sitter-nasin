mod entry;
mod module_checker;

use thiserror::Error;

use self::module_checker::TypeChecker;
use crate::{bytecode as b, utils};

#[derive(Debug, Clone, Error)]
pub enum TypeError {
    #[error("Expected type {expected}, found {actual}")]
    UnexpectedType { expected: b::Type, actual: b::Type },
    #[error(
        "All results of the expression should have the same type\n{}",
        utils::indented(2, .0.iter().map(|t| format!("- found {t}"))),
    )]
    TypeMisatch(Vec<b::Type>),
}

pub fn check_module(module: b::Module) -> (b::Module, Vec<TypeError>) {
    TypeChecker::new().check_module(module)
}
