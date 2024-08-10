use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

use clap::{Parser, Subcommand, ValueEnum};
use torvo::config::BuildConfig;
use torvo::{codegen, parser, typecheck};
use tree_sitter as ts;

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
    /// Build a source file
    Build {
        /// Path to the file to compile
        file: PathBuf,
        #[arg(long, short)]
        /// Path where to place the output file
        out: Option<PathBuf>,
        #[arg(long, short)]
        /// Omit all messages
        silent: bool,
        #[arg(long)]
        /// Whether to dump the AST of the source file
        dump_ast: bool,
        #[arg(long)]
        /// Whether to dump the parsed bytecode of the source file
        dump_bytecode: bool,
        #[arg(long)]
        /// Whether to dump the parsed CLIF of the source file, if using Cranelift
        dump_clif: bool,
    },
    /// Dump artifacts of compilation
    Dump {
        target: DumpTarget,
        /// Path to the file to show the artifacts of
        file: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum DumpTarget {
    Ast,
    Bytecode,
}

fn main() {
    unsafe { compact_debug::enable(true) };

    let cli = Cli::parse();

    match cli.cmd {
        CliCommand::Build {
            file,
            out,
            silent,
            dump_ast,
            dump_bytecode,
            dump_clif,
        } => {
            let src = fs::read_to_string(&file).expect("failed to read file");
            let cfg = BuildConfig {
                out: out.unwrap_or_else(|| {
                    env::current_dir()
                        .unwrap()
                        .to_owned()
                        .join(file.file_stem().unwrap())
                }),
                silent,
                dump_ast,
                dump_bytecode,
                dump_clif,
            };

            let mut ts_parser = ts::Parser::new();
            ts_parser
                .set_language(tree_sitter_torvo::language())
                .unwrap();
            let tree = ts_parser
                .parse(&src, None)
                .expect("Could not parse this file");
            let root_node = tree.root_node();

            if dump_ast {
                println!("{}", root_node.to_sexp());
            }

            let module = parser::parse_module(file, &src, root_node);
            //eprintln!("{}", module);

            let (module, errors) = typecheck::check_module(module);

            if dump_bytecode {
                println!("{}", module);
            }

            if errors.len() > 0 {
                for err in errors {
                    eprintln!("{err}");
                }
                exit(1);
            }

            codegen::compile_program(&module, &cfg);
        }
        CliCommand::Dump { target, file } => {
            //let src = fs::read_to_string(&file).expect("failed to read file");
            //let name = get_module_name(&file);
            //
            //match target {
            //    DumpTarget::Ast => {
            //        let tree = parse_tree(&src);
            //        println!("{}", tree.root_node().to_sexp());
            //    }
            //    DumpTarget::Mir => {
            //        let module = parse_mir(&name, &src, &BuildConfig::default());
            //        println!("{}", module);
            //    }
            //}
        }
    }
}
