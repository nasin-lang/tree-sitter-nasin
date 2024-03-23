pub mod codegen;
pub mod module_builder;
pub mod proto;
pub mod tree_sitter_utils;

use std::fs;
use std::path::PathBuf;

use crate::codegen::compile_program;
use clap::{Parser, Subcommand, ValueEnum};
use tree_sitter as ts;
use tree_sitter_torvo::language;

#[derive(Parser, Debug)]
#[command(name = "Torvo Language")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    #[clap(alias = "b")]
    /// Build file to a binary
    Build {
        /// Path to the file to compile
        file: PathBuf,
    },
    /// Show artifacts of compilation
    Show {
        target: ShowTarget,
        /// Path to the file to show the artifacts of
        file: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ShowTarget {
    Ast,
    Lex,
}

fn main() {
    unsafe { compact_debug::enable(true) };

    let cli = Cli::parse();

    match cli.cmd {
        CliCommand::Build { file } => {
            let name = file.file_stem().expect("failed to read module name");
            let src = fs::read_to_string(&file).expect("failed to read file");

            let mut parser = ts::Parser::new();
            parser.set_language(language()).unwrap();
            let tree = parser.parse(&src, None).expect("Could not parse this f");

            println!("{}\n", tree.root_node().to_sexp());

            let module = module_builder::ModuleBuilder::parse(
                name.to_str().expect("What even is this name").to_string(),
                &src,
                &tree.root_node(),
            );

            println!("{}", module);

            compile_program(&module);
        }
        CliCommand::Show { target, file } => {
            let name = file.file_stem().expect("failed to read module name");
            let src = fs::read_to_string(&file).expect("failed to read file");

            let mut parser = ts::Parser::new();
            parser.set_language(language()).unwrap();
            let tree = parser.parse(&src, None).expect("Could not parse this f");

            match target {
                ShowTarget::Ast => {
                    println!("{}", tree.root_node().to_sexp());
                }
                ShowTarget::Lex => {
                    let module = module_builder::ModuleBuilder::parse(
                        name.to_str().expect("What even is this name").to_string(),
                        &src,
                        &tree.root_node(),
                    );
                    println!("{}", module);
                }
            }
        }
    }
}
