mod binary;
mod traits;

use target_lexicon::Triple;
use traits::Codegen;

use crate::proto::lex;

pub fn compile_program(program: &lex::Module) {
    // TODO: get the target from some kind of configuration
    let triple = Triple::host();
    let mut codegen = binary::BinaryCodegen::new(triple, program.name.clone());

    for instr in program.body.iter() {
        if let Some(lex::instr::Instr::FnDecl(fn_decl)) = instr.instr.as_ref() {
            codegen.declare_function(fn_decl);
        };
    }

    for instr in program.body.iter() {
        if let Some(lex::instr::Instr::FnDecl(fn_decl)) = instr.instr.as_ref() {
            codegen.define_function(fn_decl);
        };
    }

    codegen.write_to_file(&program.name);

    println!("Compiled program to {}", &program.name);
}
