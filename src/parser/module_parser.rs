use std::collections::HashMap;

use tree_sitter as ts;

use super::parser_value::Value;
use super::type_parser::TypeParser;
use crate::parser::expr_parser::ExprParser;
use crate::parser::parser_value::ValueBody;
use crate::sources::Sources;
use crate::utils::TreeSitterUtils;
use crate::{bytecode as b, utils};

pub struct ModuleParser<'a> {
    pub types: TypeParser<'a>,
    pub globals: Vec<DeclaredGlobal<'a>>,
    pub funcs: Vec<DeclaredFunc<'a>>,
    pub idents: HashMap<&'a str, Value>,
    src: &'a Sources<'a>,
}

impl<'a> ModuleParser<'a> {
    pub fn new(src: &'a Sources<'a>) -> Self {
        ModuleParser {
            src,
            types: TypeParser::new(src),
            globals: vec![],
            funcs: vec![],
            idents: HashMap::new(),
        }
    }

    pub fn finish(mut self) -> b::Module {
        for i in 0..self.globals.len() {
            let value_node = self.globals[i].value_node.clone();

            let mut value_parser = ExprParser::new(self.src, self, None, []);

            let value = value_parser.add_expr_node(value_node, true);
            value_parser.push_values([&value], true);

            (self, self.globals[i].global.body) = value_parser.finish();
        }

        for i in 0..self.funcs.len() {
            let Some(value_node) = self.funcs[i].value_node else {
                continue;
            };
            let params_names = self.funcs[i].params_names.clone();

            let mut value_parser = ExprParser::new(self.src, self, Some(i), params_names);

            let value = value_parser.add_expr_node(value_node, true);
            value_parser.push_values([&value], true);

            (self, self.funcs[i].func.body) = value_parser.finish();
        }

        b::Module {
            typedefs: self.types.typedefs,
            globals: self.globals.into_iter().map(|x| x.global).collect(),
            funcs: self.funcs.into_iter().map(|x| x.func).collect(),
            sources: self
                .src
                .sources
                .iter()
                .map(|s| b::Source::new(s.path.to_owned()))
                .collect(),
        }
    }

    pub fn add_root(&mut self, node: ts::Node<'a>) {
        node.of_kind("root");

        for sym_node in node.iter_children() {
            let ident_node = sym_node.required_field("name").of_kind("ident");
            let ident = ident_node.get_text(self.src.content(0));

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
                let param_name = param_name_node.get_text(self.src.content(0));

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
            match directive_node
                .required_field("name")
                .get_text(self.src.content(0))
            {
                "extern" => {
                    // TODO: error handling
                    assert!(extn.is_none());
                    assert!(args_nodes.len() == 1);
                    assert!(args_nodes[0].kind() == "string_lit");
                    let symbol_name = utils::decode_string_lit(
                        args_nodes[0]
                            .required_field("content")
                            .get_text(self.src.content(0)),
                    );
                    extn = Some(b::Extern { name: symbol_name });
                }
                _ => todo!(),
            }
        }

        let func_idx = self.funcs.len();

        self.funcs.push(DeclaredFunc {
            func: b::Func {
                params,
                ret: ret_ty,
                extn,
                body: vec![],
                loc: b::Loc::from_node(0, &node),
            },
            value_node: node.field("return"),
            params_names,
        });
        self.idents.insert(
            name,
            Value::new(ValueBody::Func(func_idx), b::Loc::from_node(0, &node)),
        );
    }

    pub fn add_global(&mut self, name: &'a str, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "global_var_decl");

        let ty = match node.field("type") {
            Some(ty_node) => self.types.parse_type(ty_node),
            None => b::Type::unknown(None),
        };

        let global_idx = self.globals.len();

        self.globals.push(DeclaredGlobal {
            global: b::Global {
                ty,
                body: vec![],
                is_entry_point: name == "main",
                loc: b::Loc::from_node(0, &node),
            },
            value_node: node.required_field("value"),
        });
        self.idents.insert(
            name,
            Value::new(ValueBody::Global(global_idx), b::Loc::from_node(0, &node)),
        );
    }
}

pub struct DeclaredFunc<'a> {
    pub func: b::Func,
    value_node: Option<ts::Node<'a>>,
    params_names: Vec<(&'a str, b::Loc)>,
}

pub struct DeclaredGlobal<'a> {
    pub global: b::Global,
    value_node: ts::Node<'a>,
}
