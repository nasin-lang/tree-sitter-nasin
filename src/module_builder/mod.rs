mod instr_builder;
mod registry;
mod types;

use tree_sitter as ts;

use self::instr_builder::InstrBuilder;
use self::registry::ModuleRegistry;
use self::types::fn_type;
use crate::module_builder::registry::{FuncRegistry, Registry, ValueRef};
use crate::module_builder::types::unknown_type;
use crate::proto::m_ir;
use crate::tree_sitter_utils::TreeSitterUtils;

pub struct ModuleBuilder<'a> {
    pub name: String,
    pub source: &'a str,
    registry: ModuleRegistry,
    data: Vec<m_ir::DataDecl>,
    funcs: Vec<m_ir::FnDecl>,
    init_body: Vec<m_ir::Instr>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(name: String, source: &'a str) -> Self {
        ModuleBuilder {
            name,
            source,
            data: Vec::new(),
            funcs: Vec::new(),
            registry: ModuleRegistry::new(),
            init_body: Vec::new(),
        }
    }

    pub fn parse(name: String, source: &'a str, node: &'a ts::Node<'a>) -> m_ir::Module {
        node.of_kind("root");

        let mut this = ModuleBuilder::new(name, source);

        for sym_node in node.iter_children() {
            let ident_node = sym_node.required_field("name").of_kind("ident");
            let ident = ident_node.get_text(this.source).to_string();

            match sym_node.kind() {
                "fn_decl" => this.add_func(ident, sym_node),
                "global_var_decl" => this.add_global(ident, sym_node),
                _ => panic!("Unexpected symbol kind: {}", sym_node.kind()),
            }
        }

        this.finish()
    }

    pub fn add_func(&mut self, name: String, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "fn_decl");

        let mut local_registry = FuncRegistry::new(&mut self.registry);
        let mut local_builder = InstrBuilder::new(&mut local_registry, self.source);

        for param_node in node.iter_field("params") {
            let param_name_node = param_node.required_field("pat").of_kind("ident");
            let param_name = param_name_node.get_text(self.source);

            let ty = param_node
                .field("type")
                .as_ref()
                .map_or(unknown_type(), |ty_node| local_builder.parse_type(ty_node));

            local_builder.registry.register_param(param_name, ty);
        }

        let func_idx = local_builder
            .registry
            .module_registry
            .register_func(&name, local_builder.registry.get_params().map(|p| p.r#type));
        let func_ref = ValueRef::Func(func_idx);

        let ret = node
            .field("ret_type")
            .as_ref()
            .map_or(unknown_type(), |ty_node| local_builder.parse_type(ty_node));

        let (ret_value, _) = local_builder.add_expr(&node.required_field("return"));
        let ret_ref: ValueRef = ret_value.clone().into();

        local_builder.body.push(m_ir::Instr {
            instr: Some(m_ir::instr::Instr::FnReturn(ret_value.clone())),
        });

        local_builder
            .registry
            .set_value_type(ret_ref.clone(), ret, None);

        let locals: Vec<_> = local_builder.registry.get_locals().collect();
        let params: Vec<_> = local_builder.registry.get_params().collect();
        let ret = local_builder.registry.value_type(&ret_ref).unwrap();

        let ty = fn_type(params.iter().map(|p| p.r#type.clone()), [ret.clone()]);

        local_builder
            .registry
            .module_registry
            .set_value_type(func_ref.clone(), ty.clone(), None);

        let body = local_builder.finish();

        self.funcs.push(m_ir::FnDecl {
            export: Some(m_ir::Export { name }),
            locals,
            params,
            ret: vec![ret],
            body,
        });
    }

    pub fn add_global(&mut self, name: String, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "global_var_decl");

        let mut local_builder = InstrBuilder::new(&mut self.registry, self.source);

        let ty = node
            .field("type")
            .map_or(unknown_type(), |ty| local_builder.parse_type(&ty));

        let global_idx = local_builder.registry.register_global(&name, ty.clone());
        let global_ref = ValueRef::Global(global_idx);

        let (value, _) = local_builder.add_expr(&node.required_field("value"));
        let value_ref: ValueRef = value.clone().into();

        // Shadow the global value with the local result so next time we use the global we
        // get the local value instead of loading the global again.
        local_builder
            .registry
            .idents_mut()
            .insert(&name, value_ref.clone());

        local_builder.body.push(m_ir::Instr {
            instr: Some(m_ir::instr::Instr::StoreGlobal(m_ir::StoreGlobal {
                global_idx,
                value,
            })),
        });

        local_builder
            .registry
            .set_value_type(value_ref.clone(), ty, Some(global_ref.clone()));

        let ty = local_builder.registry.value_type(&value_ref).unwrap();

        self.data.push(m_ir::DataDecl {
            // FIXME: read export info from the source
            export: if name == "main" {
                Some(m_ir::Export { name })
            } else {
                None
            },
            r#type: ty,
        });

        self.init_body.extend(local_builder.finish());
    }

    pub fn finish(self) -> m_ir::Module {
        m_ir::Module {
            name: self.name,
            data: self.data,
            funcs: self.funcs,
            init: if self.init_body.len() > 0 {
                Some(m_ir::ModuleInit {
                    locals: self.registry.get_locals().collect(),
                    body: self.init_body,
                })
            } else {
                None
            },
        }
    }
}
