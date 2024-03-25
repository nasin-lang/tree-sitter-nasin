mod instr_builder;
mod registry;
mod types;

use tree_sitter as ts;

use self::instr_builder::InstrBuilder;
use self::registry::Registry;
use self::registry::ValueTypeDeps;
use self::types::fn_type;
use crate::proto::m_ir;
use crate::tree_sitter_utils::TreeSitterUtils;

pub struct ModuleBuilder<'a> {
    pub name: String,
    pub source: &'a str,
    registry: Registry,
    symbols: Vec<m_ir::Symbol>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(name: String, source: &'a str) -> Self {
        ModuleBuilder {
            name,
            source,
            symbols: Vec::new(),
            registry: Registry::new(),
        }
    }

    pub fn parse(name: String, source: &'a str, node: &'a ts::Node<'a>) -> m_ir::Module {
        node.of_kind("root");

        let mut this = ModuleBuilder::new(name, source);

        let registered_syms: Vec<_> = node
            .iter_children()
            .map(|sym_node| {
                let ident_node = sym_node.required_field("name").of_kind("ident");
                let ident = ident_node.get_text(this.source);
                let name = this.registry.use_name(Some(&ident));

                (name, sym_node)
            })
            .collect();

        for (name, sym_node) in registered_syms {
            this.add_symbol(name, sym_node);
        }

        this.finish()
    }

    pub fn add_symbol(&mut self, name: String, node: ts::Node<'a>) {
        match node.kind() {
            "fn_decl" => {
                let mut local_registry = self.registry.clone();
                let mut local_builder = InstrBuilder::new(&mut local_registry, self.source);

                let mut params = Vec::new();
                let mut params_ty = Vec::new();
                for param_node in node.iter_field("params") {
                    let param_name_node = param_node.required_field("pat").of_kind("ident");
                    let param_name = param_name_node.get_text(self.source);
                    params.push(
                        local_builder
                            .registry
                            .use_name(Some(param_name))
                            .to_string(),
                    );

                    let ty = param_node.field("type").as_ref().map_or(
                        m_ir::Type {
                            r#type: Some(m_ir::r#type::Type::Unknown(true)),
                        },
                        |ty_node| local_builder.parse_type(ty_node),
                    );

                    params_ty.push(ty.clone());
                    local_builder.registry.insert_value_type(
                        &param_name,
                        ty.clone(),
                        ValueTypeDeps::default(),
                    );
                }

                let mut ret_ty = node.field("ret_type").as_ref().map_or(
                    m_ir::Type {
                        r#type: Some(m_ir::r#type::Type::Unknown(true)),
                    },
                    |ty_node| local_builder.parse_type(ty_node),
                );

                let ty = fn_type(params_ty.clone(), [ret_ty.clone()]);

                self.registry
                    .insert_value_type(&name, ty.clone(), ValueTypeDeps::default());
                local_builder.registry.insert_value_type(
                    &name,
                    ty.clone(),
                    ValueTypeDeps::default(),
                );

                let (ret, _) = local_builder.add_expr(&node.required_field("return"), None);
                local_builder.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::FnReturn(ret.clone())),
                });

                if let m_ir::value::Value::Ident(ident) = ret.value.as_ref().unwrap() {
                    local_builder
                        .registry
                        .set_value_type(&ident, ret_ty, Some(&name));
                }

                ret_ty = local_builder.registry.value_type(&ret);

                for (i, arg) in params.iter().enumerate() {
                    let arg_value = m_ir::Value {
                        value: Some(m_ir::value::Value::Ident(arg.to_string())),
                    };
                    let parsed_ty = local_builder.registry.value_type(&arg_value);
                    params_ty[i] = parsed_ty;
                }

                let ty = fn_type(params_ty.clone(), [ret_ty.clone()]);

                self.registry.set_value_type(&name, ty.clone(), None);
                local_builder.registry.set_value_type(&name, ty, None);

                self.symbols.push(m_ir::Symbol {
                    symbol: Some(m_ir::symbol::Symbol::FnDecl(m_ir::FnDecl {
                        name,
                        r#type: m_ir::FnType {
                            ret: vec![ret_ty],
                            args: params_ty,
                        },
                        args: params,
                        body: local_builder.finish(),
                    })),
                });
            }
            "global_var_decl" => {
                let mut local_builder = InstrBuilder::new(&mut self.registry, self.source);

                let ty = node.field("type").map_or(
                    m_ir::Type {
                        r#type: Some(m_ir::r#type::Type::Unknown(true)),
                    },
                    |ty| local_builder.parse_type(&ty),
                );

                local_builder.registry.insert_value_type(
                    &name,
                    ty.clone(),
                    ValueTypeDeps::default(),
                );

                let (value, _) = local_builder.add_expr(&node.required_field("value"), None);
                local_builder.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::BodyReturn(value.clone())),
                });

                if let m_ir::value::Value::Ident(ident) = value.value.as_ref().unwrap() {
                    local_builder
                        .registry
                        .set_value_type(&ident, ty, Some(&name));
                }

                let ty = local_builder.registry.value_type(&value);

                local_builder
                    .registry
                    .set_value_type(&name, ty.clone(), None);

                self.symbols.push(m_ir::Symbol {
                    symbol: Some(m_ir::symbol::Symbol::DataDecl(m_ir::DataDecl {
                        name,
                        r#type: ty,
                        body: local_builder.finish(),
                    })),
                });
            }
            k => panic!("Found unexpected symbol `{}`", k),
        };
    }

    pub fn finish(self) -> m_ir::Module {
        m_ir::Module {
            name: self.name,
            symbols: self.symbols,
        }
    }
}
