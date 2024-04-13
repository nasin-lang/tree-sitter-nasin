use std::path::Path;

use crate::mir;

pub trait Codegen {
    fn declare_function(&mut self, idx: usize, decl: &mir::Func);
    fn build_function(&mut self, idx: usize, decl: &mir::Func);
    fn declare_global(&mut self, idx: usize, decl: &mir::Global);
    fn build_module_init(&mut self, init: &mir::ModuleInit);
    fn write_to_file(self, file: &Path);
}
