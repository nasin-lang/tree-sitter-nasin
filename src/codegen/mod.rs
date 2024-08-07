mod binary;

use std::fs;

use crate::{bytecode as b, config};

pub fn compile_program(program: &b::Module, cfg: &config::BuildConfig) {
    fs::create_dir_all(cfg.out.parent().unwrap()).unwrap();

    let codegen = binary::BinaryCodegen::new(program, cfg);
    codegen.write();

    if !cfg.silent {
        println!("Compiled program to {}", &cfg.out.to_string_lossy());
    }
}
