use std::collections::HashMap;
use std::path::PathBuf;

use tree_sitter as ts;

use super::parser_value::Value;
use super::type_parser::TypeParser;
use crate::parser::expr_parser::ExprParser;
use crate::parser::parser_value::ValueBody;
use crate::utils::TreeSitterUtils;
use crate::{bytecode as b, utils};

pub struct ModuleParser<'a> {
    pub types: TypeParser<'a>,
    pub globals: Vec<b::Global>,
    pub funcs: Vec<b::Func>,
    pub idents: HashMap<&'a str, Value>,
    src: &'a str,
    path: PathBuf,
}

impl<'a> ModuleParser<'a> {
    pub fn new(path: PathBuf, src: &'a str) -> Self {
        ModuleParser {
            src,
            path,
            types: TypeParser::new(src),
            globals: vec![],
            funcs: vec![],
            idents: HashMap::new(),
        }
    }

    pub fn finish(self) -> b::Module {
        b::Module {
            typedefs: self.types.typedefs,
            funcs: self.funcs,
            globals: self.globals,
            sources: vec![b::Source::new(self.path)],
        }
    }

    pub fn add_root(&mut self, node: ts::Node<'a>) {
        node.of_kind("root");

        for sym_node in node.iter_children() {
            let ident_node = sym_node.required_field("name").of_kind("ident");
            let ident = ident_node.get_text(self.src);

            match sym_node.kind() {
                "type_decl" => self.types.add_type(ident, sym_node),
                "func_decl" => self.add_func(ident, sym_node),
                "global_var_decl" => self.add_global(ident, sym_node),
                _ => panic!("Unexpected symbol kind: {}", sym_node.kind()),
            }
        }
    }

    pub fn add_func(&mut self, name: &'a str, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "func_decl");

        let (params, params_names): (Vec<_>, Vec<_>) = node
            .iter_field("params")
            .map(|param_node| {
                let param_name_node = param_node.required_field("pat").of_kind("ident");
                let param_name = param_name_node.get_text(self.src);

                let param_ty = match param_node.field("type") {
                    Some(ty_node) => self.types.parse_type(ty_node),
                    None => b::Type::unknown(None),
                };

                (
                    b::Param {
                        ty: param_ty,
                        loc: b::Loc::from_node(0, &param_node),
                    },
                    (param_name, b::Loc::from_node(0, &param_name_node)),
                )
            })
            .unzip();

        let ret_ty = match node.field("ret_type") {
            Some(ty_node) => self.types.parse_type(ty_node),
            None => b::Type::unknown(None),
        };

        let mut extn: Option<b::Extern> = None;
        for directive_node in node.iter_field("directives") {
            let args_nodes: Vec<_> = directive_node.iter_field("args").collect();
            match directive_node.required_field("name").get_text(self.src) {
                "extern" => {
                    // TODO: error handling
                    assert!(extn.is_none());
                    assert!(args_nodes.len() == 1);
                    assert!(args_nodes[0].kind() == "string_lit");
                    let symbol_name = utils::decode_string_lit(
                        args_nodes[0].required_field("content").get_text(self.src),
                    );
                    extn = Some(b::Extern { name: symbol_name });
                }
                _ => todo!(),
            }
        }

        let func_idx = self.funcs.len();

        self.funcs.push(b::Func {
            params,
            ret: ret_ty,
            extn,
            body: vec![],
            loc: b::Loc::from_node(0, &node),
        });
        self.idents.insert(
            name,
            Value::new(ValueBody::Func(func_idx), b::Loc::from_node(0, &node)),
        );

        if let Some(return_node) = node.field("return") {
            self.funcs[func_idx].body = utils::replace_with(self, |this| {
                let mut value_parser =
                    ExprParser::new(self.src, this, Some(func_idx), params_names);

                let value = value_parser.add_expr_node(return_node, true);
                value_parser.push_values([&value], true);

                value_parser.finish()
            });
        };
    }

    pub fn add_global(&mut self, name: &'a str, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "global_var_decl");

        let ty = match node.field("type") {
            Some(ty_node) => self.types.parse_type(ty_node),
            None => b::Type::unknown(None),
        };

        let global_idx = self.globals.len();

        self.globals.push(b::Global {
            ty,
            body: vec![],
            is_entry_point: name == "main",
            loc: b::Loc::from_node(0, &node),
        });
        self.idents.insert(
            name,
            Value::new(ValueBody::Global(global_idx), b::Loc::from_node(0, &node)),
        );

        self.globals[global_idx].body = utils::replace_with(self, |this| {
            let mut value_parser = ExprParser::new(self.src, this, None, []);

            let value = value_parser.add_expr_node(node.required_field("value"), true);
            value_parser.push_values([&value], true);
            value_parser.finish()
        });
    }
}
