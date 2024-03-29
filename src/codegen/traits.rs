use crate::proto::m_ir;

pub trait Codegen {
    fn declare_function(&mut self, idx: usize, decl: &m_ir::FnDecl);
    fn build_function(&mut self, idx: usize, decl: &m_ir::FnDecl);
    fn declare_global(&mut self, idx: usize, decl: &m_ir::DataDecl);
    fn build_module_init(&mut self, init: &m_ir::ModuleInit);
    fn write_to_file(self, file: &str);
}
