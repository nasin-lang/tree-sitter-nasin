use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use torvo::{build_file, config::BuildConfig, get_module_name, parse_mir, parse_tree};

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
        /// Whether to dump the MIR of the source file
        dump_mir: bool,
        #[arg(long)]
        /// Whether to dump the CLIF of the source file, if using Cranelift
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
    Mir,
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
            dump_mir,
            dump_clif,
        } => {
            let src = fs::read_to_string(&file).expect("failed to read file");
            let name = get_module_name(&file);

            let cfg = BuildConfig {
                out: out.unwrap_or(name.clone().into()),
                silent,
                dump_ast,
                dump_mir,
                dump_clif,
            };

            build_file(&name, &src, &cfg);
        }
        CliCommand::Dump { target, file } => {
            let src = fs::read_to_string(&file).expect("failed to read file");
            let name = get_module_name(&file);

            match target {
                DumpTarget::Ast => {
                    let tree = parse_tree(&src);
                    println!("{}", tree.root_node().to_sexp());
                }
                DumpTarget::Mir => {
                    let module = parse_mir(&name, &src, &BuildConfig::default());
                    println!("{}", module);
                }
            }
        }
    }
}
