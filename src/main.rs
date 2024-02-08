pub mod proto;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, io};

use clap::{Parser, Subcommand, ValueEnum};
use protobuf::Message;

#[derive(Parser, Debug)]
#[command(name = "Torvo Language")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    /// Show artifacts of compilation
    Show {
        target: ShowTarget,
        /// Path to the file to show the artifacts of
        file: PathBuf,
        #[arg(short, long)]
        /// Print the artifacts in a pretty format
        pretty: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ShowTarget {
    Ast,
    Ir,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        CliCommand::Show {
            target,
            file,
            pretty,
        } => {
            let parser = env::current_exe()
                .expect("failed to get execution path")
                .parent()
                .unwrap()
                .join("torvo-parser");
            let output = Command::new(dbg!(parser))
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

            if let (&false, &ShowTarget::Ast) = (&pretty, &target) {
                io::stdout().write_all(&output.stdout).unwrap();
                return;
            }

            let ast = proto::ast::Module::parse_from_bytes(output.stdout.as_slice()).unwrap();

            match target {
                ShowTarget::Ast => {
                    println!("{}", protobuf::text_format::print_to_string_pretty(&ast));
                }
                ShowTarget::Ir => {
                    println!("{}", protobuf::text_format::print_to_string_pretty(&ast));
                }
            }
        }
    }
}
