pub mod codegen;
pub mod lexer;
pub mod proto;
pub mod tree_sitter_test;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, io};

use clap::{Parser, Subcommand, ValueEnum};
use codegen::compile_program;
use prost::Message;

#[derive(Parser, Debug)]
#[command(name = "Torvo Language")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
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
        #[arg(short, long)]
        /// Print the artifacts in a text format
        text: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ShowTarget {
    Ast,
    Lex,
}

fn main() {
    unsafe { compact_debug::enable(true) };

    tree_sitter_test::toy();

    let cli = Cli::parse();

    match cli.cmd {
        CliCommand::Build { file } => {
            let parser_path = env::current_exe()
                .expect("failed to get execution path")
                .parent()
                .unwrap()
                .join("torvo-parser");
            let output = Command::new(parser_path)
                .arg(&file)
                .arg(&file.file_stem().expect("failed to read module name"))
                .stdin(File::open(file).expect("failed to open file"))
                .output()
                .expect("failed to parse file");

            if !output.status.success() {
                // TODO: better handling of errors
                io::stderr().write_all(&output.stderr).unwrap();
                exit(output.status.code().unwrap_or(1));
            }

            let ast = proto::ast::Module::decode(output.stdout.as_slice()).unwrap();
            let lex = proto::lex::Module::from(&ast);

            println!("{}", lex);

            compile_program(&lex);
        }
        CliCommand::Show { target, file, text } => {
            let parser_path = env::current_exe()
                .expect("failed to get execution path")
                .parent()
                .unwrap()
                .join("torvo-parser");
            let output = Command::new(parser_path)
                .arg(&file)
                .arg(&file.file_stem().expect("failed to read module name"))
                .stdin(File::open(file).expect("failed to open file"))
                .output()
                .expect("failed to parse file");

            if !output.status.success() {
                // TODO: better handling of errors
                io::stderr().write_all(&output.stderr).unwrap();
                exit(output.status.code().unwrap_or(1));
            }

            if let (&false, &ShowTarget::Ast) = (&text, &target) {
                io::stdout().write_all(&output.stdout).unwrap();
                return;
            }

            let ast = proto::ast::Module::decode(output.stdout.as_slice()).unwrap();

            match target {
                ShowTarget::Ast => {
                    println!("{:?}", ast);
                }
                ShowTarget::Lex => {
                    let lex = proto::lex::Module::from(&ast);
                    println!("{}", lex);
                }
            }
        }
    }
}
