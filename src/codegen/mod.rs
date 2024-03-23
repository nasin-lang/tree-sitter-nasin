mod binary;
mod traits;

use target_lexicon::Triple;
use traits::Codegen;

use crate::proto::m_ir;

pub fn compile_program(program: &m_ir::Module) {
    // TODO: get the target from some kind of configuration
    let triple = Triple::host();
    let mut codegen = binary::BinaryCodegen::new(triple, program.name.clone());

    for symbol in program.symbols.iter() {
        match symbol.symbol.as_ref() {
            Some(m_ir::symbol::Symbol::FnDecl(fn_decl)) => {
                codegen.declare_function(fn_decl);
            }
            Some(m_ir::symbol::Symbol::DataDecl(data_decl)) => {
                codegen.declare_data(data_decl);
            }
            _ => {
                unreachable!();
            }
        }
    }

    for symbol in program.symbols.iter() {
        if let Some(m_ir::symbol::Symbol::FnDecl(fn_decl)) = symbol.symbol.as_ref() {
            codegen.build_function(fn_decl);
        }
    }

    codegen.write_to_file(&program.name);

    println!("Compiled program to {}", &program.name);
}
