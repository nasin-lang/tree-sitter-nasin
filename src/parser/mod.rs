mod expr_parser;
mod module_parser;
mod parser_value;
mod type_parser;

use std::path::PathBuf;

use tree_sitter as ts;

use self::module_parser::ModuleParser;
use crate::bytecode as b;

pub fn parse_module(path: PathBuf, src: &str, root_node: ts::Node) -> b::Module {
    let mut module_parser = ModuleParser::new(path, src);
    module_parser.add_root(root_node);
    module_parser.finish()
}
