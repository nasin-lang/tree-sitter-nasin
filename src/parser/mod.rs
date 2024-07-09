mod module_parser;
mod parser_value;
mod type_parser;
mod value_parser;

use tree_sitter as ts;

use self::module_parser::ModuleParser;
use crate::bytecode as b;

pub fn parse_module(src: &str, root_node: ts::Node) -> b::Module {
    let mut module_parser = ModuleParser::new(src);
    module_parser.add_root(root_node);
    module_parser.finish()
}
