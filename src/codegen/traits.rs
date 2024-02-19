use crate::proto::lex;

pub trait Codegen {
    fn declare_function(&mut self, decl: &lex::FnDecl);
    fn build_function(&mut self, decl: &lex::FnDecl);
    fn declare_data(&mut self, decl: &lex::DataDecl);
    fn write_to_file(self, file: &str);
}
