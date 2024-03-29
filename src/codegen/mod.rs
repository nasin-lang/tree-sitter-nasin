mod binary;
mod traits;

use target_lexicon::Triple;
use traits::Codegen;

use crate::proto::m_ir;

pub fn compile_program(program: &m_ir::Module) {
    // TODO: get the target from some kind of configuration
    let triple = Triple::host();
    let mut codegen = binary::BinaryCodegen::new(triple, program.name.clone());

    for (i, data) in program.data.iter().enumerate() {
        codegen.declare_global(i, data);
    }

    for (i, func) in program.funcs.iter().enumerate() {
        codegen.declare_function(i, func);
    }

    for (i, func) in program.funcs.iter().enumerate() {
        codegen.build_function(i, func);
    }

    if let Some(init) = &program.init {
        codegen.build_module_init(init);
    }

    codegen.write_to_file(&program.name);

    println!("Compiled program to {}", &program.name);
}
