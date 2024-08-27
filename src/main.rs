use std::env;
use std::path::PathBuf;
use std::process::exit;

use clap::{Parser, Subcommand};
use nasin::config::BuildConfig;
use nasin::context;

#[derive(Parser, Debug)]
#[command(name = "Nasin Language")]
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
            let mut ctx = context::BuildContext::new(BuildConfig {
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
            });

            let src_idx = ctx.preload(file).expect("file not found");

            ctx.parse(src_idx);

            {
                let errors = ctx.errors.lock().unwrap();
                if errors.len() > 0 {
                    for err in errors.iter() {
                        eprintln!("{err}");
                    }
                    exit(1);
                }
            }

            ctx.compile();
        }
    }
}
