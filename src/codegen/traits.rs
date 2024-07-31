use std::path::Path;

use crate::bytecode as b;

pub trait Codegen<'a> {
    fn declare_typedef(&mut self, idx: usize, decl: &'a b::TypeDef);
    fn declare_function(&mut self, idx: usize, decl: &'a b::Func);
    fn build_function(&mut self, idx: usize, decl: &'a b::Func);
    fn declare_global(&mut self, idx: usize, decl: &'a b::Global);
    fn write_to_file(self, file: &Path);
}
