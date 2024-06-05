#![allow(irrefutable_let_patterns)]

pub mod codegen;
pub mod config;
pub mod mir;
pub mod module_builder;
mod tree_sitter_utils;
pub mod utils;

use std::path::Path;

use config::BuildConfig;
use tree_sitter as ts;
use tree_sitter_torvo::language;

use crate::codegen::compile_program;

/// Build a source file
pub fn build_file(name: &str, src: &str, cfg: &BuildConfig) {
    let module = parse_mir(name, src, cfg);

    if cfg.dump_mir {
        println!("{}", module);
        println!();
    }

    compile_program(&module, cfg);
}

/// Parse the MIR of a source file
pub fn parse_mir(name: &str, src: &str, cfg: &BuildConfig) -> mir::Module {
    let tree = parse_tree(&src);

    if cfg.dump_ast {
        println!("{}", tree.root_node().to_sexp());
        println!();
    }

    let builder = module_builder::ModuleBuilder::new(name, src);
    builder.parse(&tree.root_node())
}

#[doc(hidden)]
pub fn parse_tree(src: &str) -> ts::Tree {
    let mut parser = ts::Parser::new();
    parser.set_language(language()).unwrap();
    parser.parse(src, None).expect("Could not parse this file")
}

/// Get the module name from a file path
pub fn get_module_name(file: &Path) -> String {
    file.file_stem()
        .expect("Failed to read module name")
        .to_str()
        .expect("What even is this name")
        .to_string()
}
