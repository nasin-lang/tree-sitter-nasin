mod entry;
mod module_checker;

use self::module_checker::TypeChecker;
use crate::sources::Sources;
use crate::{bytecode as b, errors};

pub fn check_module<'a>(
    module: b::Module,
    src: &'a Sources<'a>,
) -> (b::Module, Vec<errors::Error<'a>>) {
    TypeChecker::new(src).check_module(module)
}
