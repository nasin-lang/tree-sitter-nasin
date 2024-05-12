mod binary;
mod traits;

use target_lexicon::Triple;
use traits::Codegen;

use crate::config::BuildConfig;
use crate::mir;

pub fn compile_program(program: &mir::Module, cfg: &BuildConfig) {
    // TODO: get the target from some kind of configuration
    let triple = Triple::host();
    let mut codegen = binary::BinaryCodegen::new(triple, program.name.clone(), cfg);

    for (i, global) in program.globals.iter().enumerate() {
        codegen.declare_global(i, global);
    }

    for (i, func) in program.funcs.iter().enumerate() {
        codegen.declare_function(i, func);
    }

    for (i, func) in program.funcs.iter().enumerate() {
        if func.body.is_empty() {
            continue;
        }
        codegen.build_function(i, func);
    }

    if let Some(init) = &program.init {
        codegen.build_module_init(init);
    }

    codegen.write_to_file(&cfg.out);

    if !cfg.silent {
        println!("Compiled program to {}", &program.name);
    }
}
