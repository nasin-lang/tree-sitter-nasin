mod expr_parser;
mod module_parser;
mod parser_value;
mod type_parser;

use tree_sitter as ts;

use self::module_parser::ModuleParser;
use crate::bytecode as b;
use crate::sources::Sources;

pub fn parse_module(src: &Sources, root_node: ts::Node) -> b::Module {
    let mut module_parser = ModuleParser::new(src);
    module_parser.add_root(root_node);
    module_parser.finish()
}
