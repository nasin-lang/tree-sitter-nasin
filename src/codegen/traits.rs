use crate::proto::lex;

pub trait Codegen {
    fn declare_function(&mut self, decl: &lex::FnDecl);
    fn define_function(&mut self, decl: &lex::FnDecl);
    fn write_to_file(self, file: &str);
}
