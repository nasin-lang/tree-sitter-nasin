use crate::proto::m_ir;

pub trait Codegen {
    fn declare_function(&mut self, decl: &m_ir::FnDecl);
    fn build_function(&mut self, decl: &m_ir::FnDecl);
    fn declare_data(&mut self, decl: &m_ir::DataDecl);
    fn write_to_file(self, file: &str);
}
