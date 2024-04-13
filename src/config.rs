use std::path::PathBuf;

/// Stores the configurations for a compilation
#[derive(Debug, Clone, Default)]
pub struct BuildConfig {
    /// Path where to place the output file
    pub out: PathBuf,
    /// Omit all messages
    pub silent: bool,
    /// Whether to dump the AST of the source file
    pub dump_ast: bool,
    /// Whether to dump the MIR of the source file
    pub dump_mir: bool,
    /// Whether to dump the CLIF of the source file, if using Cranelift
    pub dump_clif: bool,
}
