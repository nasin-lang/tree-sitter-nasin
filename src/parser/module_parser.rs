use std::collections::HashMap;

use derive_new::new;
use tree_sitter as ts;

use super::parser_value::Value;
use super::type_parser::TypeParser;
use crate::parser::expr_parser::ExprParser;
use crate::parser::parser_value::ValueBody;
use crate::utils::TreeSitterUtils;
use crate::{bytecode as b, context, utils};

#[derive(new)]
pub struct ModuleParser<'a, 't> {
    #[new(value = "TypeParser::new(ctx, src_idx, mod_idx)")]
    pub types: TypeParser<'a>,
    #[new(default)]
    pub globals: Vec<DeclaredGlobal<'t>>,
    #[new(default)]
    pub funcs: Vec<DeclaredFunc<'t>>,
    #[new(default)]
    pub idents: HashMap<String, Value>,
    ctx: &'a context::BuildContext<'a>,
    src_idx: usize,
    mod_idx: usize,
}

impl<'a, 't> ModuleParser<'a, 't> {
    pub fn finish(mut self) {
        let src_idx = self.src_idx;
        let mod_idx = self.mod_idx;

        for i in 0..self.globals.len() {
            let value_node = self.globals[i].value_node.clone();

            let mut value_parser =
                ExprParser::new(self.ctx, self, src_idx, mod_idx, None, []);

            let value = value_parser.add_expr_node(value_node, true);
            value_parser.push_values([&value], true);

            (self, self.globals[i].global.body) = value_parser.finish();
        }

        for i in 0..self.funcs.len() {
            let Some(value_node) = self.funcs[i].value_node else {
                continue;
            };
            let params_names = self.funcs[i].params_names.clone();

            let mut value_parser =
                ExprParser::new(self.ctx, self, src_idx, mod_idx, Some(i), params_names);

            let value = value_parser.add_expr_node(value_node, true);
            value_parser.push_values([&value], true);

            (self, self.funcs[i].func.body) = value_parser.finish();
        }

        let module = &mut self.ctx.lock_modules_mut()[self.mod_idx];
        module.typedefs = self.types.typedefs;
        module.globals = self.globals.into_iter().map(|x| x.global).collect();
        module.funcs = self.funcs.into_iter().map(|x| x.func).collect();
    }
    pub fn add_root(&mut self, node: ts::Node<'t>) {
        node.of_kind("root");

        for sym_node in node.iter_children() {
            let ident_node = sym_node.required_field("name").of_kind("ident");
            let ident =
                ident_node.get_text(&self.ctx.source(self.src_idx).content().text);

            match sym_node.kind() {
                "type_decl" => self.types.add_type(ident, sym_node),
                "func_decl" => self.add_func(ident, sym_node),
                "global_var_decl" => self.add_global(ident, sym_node),
                _ => panic!("Unexpected symbol kind: {}", sym_node.kind()),
            }
        }
    }

    fn add_func(&mut self, name: &'a str, node: ts::Node<'t>) {
        assert_eq!(node.kind(), "func_decl");

        let (params, params_names): (Vec<_>, Vec<_>) = node
            .iter_field("params")
            .map(|param_node| {
                let param_name_node = param_node.required_field("pat").of_kind("ident");
                let param_name = param_name_node
                    .get_text(&self.ctx.source(self.src_idx).content().text);

                let param_ty = match param_node.field("type") {
                    Some(ty_node) => self.types.parse_type(ty_node),
                    None => b::Type::unknown(None),
                };

                (
                    b::Param {
                        ty: param_ty,
                        loc: b::Loc::from_node(self.src_idx, &param_node),
                    },
                    (
                        param_name.to_string(),
                        b::Loc::from_node(self.src_idx, &param_name_node),
                    ),
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
                .get_text(&self.ctx.source(self.src_idx).content().text)
            {
                "extern" => {
                    // TODO: error handling
                    assert!(extn.is_none());
                    assert!(args_nodes.len() == 1);
                    assert!(args_nodes[0].kind() == "string_lit");
                    let symbol_name = utils::decode_string_lit(
                        args_nodes[0]
                            .required_field("content")
                            .get_text(&self.ctx.source(self.src_idx).content().text),
                    );
                    extn = Some(b::Extern { name: symbol_name });
                }
                _ => todo!(),
            }
        }

        let func = b::Func {
            name: name.to_string(),
            params,
            ret: ret_ty,
            extn,
            body: vec![],
            loc: b::Loc::from_node(self.src_idx, &node),
        };
        self.idents.insert(
            func.name.clone(),
            Value::new(
                ValueBody::Func(self.mod_idx, self.funcs.len()),
                b::Loc::from_node(self.src_idx, &node),
            ),
        );
        self.funcs.push(DeclaredFunc {
            func,
            value_node: node.field("return"),
            params_names,
        });
    }
    fn add_global(&mut self, name: &'a str, node: ts::Node<'t>) {
        assert_eq!(node.kind(), "global_var_decl");

        let ty = match node.field("type") {
            Some(ty_node) => self.types.parse_type(ty_node),
            None => b::Type::unknown(None),
        };

        let global = b::Global {
            name: name.to_string(),
            ty,
            body: vec![],
            is_entry_point: name == "main",
            loc: b::Loc::from_node(self.src_idx, &node),
        };
        self.idents.insert(
            global.name.clone(),
            Value::new(
                ValueBody::Global(self.mod_idx, self.globals.len()),
                b::Loc::from_node(self.src_idx, &node),
            ),
        );
        self.globals.push(DeclaredGlobal {
            global,
            value_node: node.required_field("value"),
        });
    }
}

pub struct DeclaredFunc<'t> {
    pub func: b::Func,
    value_node: Option<ts::Node<'t>>,
    params_names: Vec<(String, b::Loc)>,
}

pub struct DeclaredGlobal<'t> {
    pub global: b::Global,
    value_node: ts::Node<'t>,
}
