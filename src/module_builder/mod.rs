mod instr_builder;
mod registry;

use tree_sitter as ts;

use self::instr_builder::InstrBuilder;
use self::registry::ModuleRegistry;
use crate::mir;
use crate::module_builder::registry::{FuncRegistry, Registry, ValueRef};
use crate::tree_sitter_utils::TreeSitterUtils;

pub struct ModuleBuilder<'a> {
    pub name: String,
    pub source: &'a str,
    registry: ModuleRegistry,
    globals: Vec<mir::Global>,
    funcs: Vec<mir::Func>,
    init_body: Vec<mir::Instr>,
}

impl<'a> ModuleBuilder<'a> {
    pub fn new(name: String, source: &'a str) -> Self {
        let mut registry = ModuleRegistry::new();

        // TODO: detect which intrinsic functions are needed and declare only those
        // TODO: use syscalls instead of libc functions, maybe we will have to implement a
        //       wrapper around syscall in C to be able to call it from Cranelift

        // void exit(int status);
        let exit_func = {
            let params = vec![mir::Type::I32];
            let ret = vec![];
            registry.register_func("exit", params.clone());
            registry.set_value_type(
                ValueRef::Func(0),
                mir::Type::func_type(params.clone(), ret.clone()),
                None,
            );
            mir::Func {
                extern_: Some(mir::Extern {
                    name: "exit".to_string(),
                }),
                params: params.into_iter().map(|ty| mir::Param { ty }).collect(),
                ret,
                ..Default::default()
            }
        };

        // ssize_t write(int fildes, const void *buf, size_t nbyte)
        let write_func = {
            let params = vec![
                mir::Type::I32,
                mir::Type::array_type(mir::Type::U8),
                mir::Type::USize,
            ];
            let ret = vec![mir::Type::USize];
            registry.register_func("write", params.clone());
            registry.set_value_type(
                ValueRef::Func(1),
                mir::Type::func_type(params.clone(), ret.clone()),
                None,
            );
            mir::Func {
                extern_: Some(mir::Extern {
                    name: "write".to_string(),
                }),
                params: params.into_iter().map(|ty| mir::Param { ty }).collect(),
                ret: vec![mir::Type::USize],
                ..Default::default()
            }
        };

        ModuleBuilder {
            name,
            source,
            globals: Vec::new(),
            funcs: vec![exit_func, write_func],
            registry,
            init_body: Vec::new(),
        }
    }

    pub fn parse(name: String, source: &'a str, node: &'a ts::Node<'a>) -> mir::Module {
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
                .map_or(mir::Type::Unknown, |ty_node| {
                    local_builder.parse_type(ty_node)
                });

            local_builder.registry.register_param(param_name, ty);
        }

        let func_idx = local_builder
            .registry
            .module_registry
            .register_func(&name, local_builder.registry.get_params().map(|p| p.ty));
        let func_ref = ValueRef::Func(func_idx);

        let ret = node
            .field("ret_type")
            .as_ref()
            .map_or(mir::Type::Unknown, |ty_node| {
                local_builder.parse_type(ty_node)
            });

        let (ret_value, _) = local_builder.add_expr(&node.required_field("return"));
        let ret_ref: ValueRef = ret_value.clone().into();

        local_builder
            .body
            .push(mir::Instr::Return(mir::ReturnInstr {
                value: Some(ret_value.clone()),
            }));

        local_builder
            .registry
            .set_value_type(ret_ref.clone(), ret, None);

        let locals: Vec<_> = local_builder.registry.get_locals().collect();
        let params: Vec<_> = local_builder.registry.get_params().collect();
        let ret = local_builder.registry.value_type(&ret_ref).unwrap();

        let ty = mir::Type::func_type(params.iter().map(|p| p.ty.clone()), [ret.clone()]);

        local_builder
            .registry
            .module_registry
            .set_value_type(func_ref.clone(), ty.clone(), None);

        let body = local_builder.finish();

        self.funcs.push(mir::Func {
            export: Some(mir::Export { name }),
            locals,
            params,
            ret: vec![ret],
            body,
            ..Default::default()
        });
    }

    pub fn add_global(&mut self, name: String, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "global_var_decl");

        let mut local_builder = InstrBuilder::new(&mut self.registry, self.source);

        let ty = node
            .field("type")
            .map_or(mir::Type::Unknown, |ty| local_builder.parse_type(&ty));

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

        local_builder
            .body
            .push(mir::Instr::StoreGlobal(mir::StoreGlobalInstr {
                global_idx,
                value,
            }));

        local_builder
            .registry
            .set_value_type(value_ref.clone(), ty, Some(global_ref.clone()));

        let ty = local_builder.registry.value_type(&value_ref).unwrap();

        self.globals.push(mir::Global {
            // FIXME: read export info from the source
            export: if name == "main" {
                Some(mir::Export { name })
            } else {
                None
            },
            ty,
        });

        self.init_body.extend(local_builder.finish());
    }

    pub fn finish(self) -> mir::Module {
        mir::Module {
            name: self.name,
            globals: self.globals,
            funcs: self.funcs,
            init: if self.init_body.len() > 0 {
                Some(mir::ModuleInit {
                    locals: self.registry.get_locals().collect(),
                    body: self.init_body,
                })
            } else {
                None
            },
        }
    }
}
