mod registry;
mod types;

use tree_sitter as ts;

use self::registry::ValueTypeDeps;
use self::types::{ambig, binop_sig, fn_type, merge_types, primitive, types_iter};
use self::{registry::Registry, types::num_type};
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

        for sym_node in node.iter_children() {
            this.add_symbol(sym_node);
        }

        this.finish()
    }

    pub fn add_symbol(&mut self, node: ts::Node<'a>) {
        match node.kind() {
            "fn_decl" => {
                let ident_node = node.required_field("name").of_kind("ident");
                let ident = ident_node.get_text(self.source);
                let name = self.registry.use_name(Some(&ident));

                let mut builder = InstrBuilder::new(&mut self.registry, self.source);

                let mut params = Vec::new();
                let mut params_ty = Vec::new();
                for param_node in node.iter_field("params") {
                    let param_name_node = param_node.required_field("pat").of_kind("ident");
                    let param_name = param_name_node.get_text(self.source);
                    params.push(builder.registry.use_name(Some(param_name)).to_string());

                    let ty = param_node.field("type").as_ref().map_or(
                        m_ir::Type {
                            r#type: Some(m_ir::r#type::Type::Unknown(true)),
                        },
                        |ty_node| builder.parse_type(ty_node),
                    );

                    params_ty.push(ty.clone());
                    builder.registry.insert_value_type(
                        &param_name,
                        ty.clone(),
                        ValueTypeDeps::default(),
                    );
                }

                let mut ret_ty = node.field("ret_type").as_ref().map_or(
                    m_ir::Type {
                        r#type: Some(m_ir::r#type::Type::Unknown(true)),
                    },
                    |ty_node| builder.parse_type(ty_node),
                );

                let ty = fn_type(params_ty.clone(), [ret_ty.clone()]);

                self.registry
                    .insert_value_type(&name, ty.clone(), ValueTypeDeps::default());
                builder
                    .registry
                    .insert_value_type(&name, ty.clone(), ValueTypeDeps::default());

                let (ret, _) = builder.add_expr(&node.required_field("return"), None);
                builder.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::FnReturn(ret.clone())),
                });

                if let m_ir::value::Value::Ident(ident) = ret.value.as_ref().unwrap() {
                    builder.registry.set_value_type(&ident, ret_ty, Some(&name));
                }

                ret_ty = builder.registry.value_type(&ret);

                for (i, arg) in params.iter().enumerate() {
                    let arg_value = m_ir::Value {
                        value: Some(m_ir::value::Value::Ident(arg.to_string())),
                    };
                    let parsed_ty = builder.registry.value_type(&arg_value);
                    params_ty[i] = parsed_ty;
                }

                let ty = fn_type(params_ty.clone(), [ret_ty.clone()]);

                self.registry.set_value_type(&name, ty.clone(), None);
                builder.registry.set_value_type(&name, ty, None);

                self.symbols.push(m_ir::Symbol {
                    symbol: Some(m_ir::symbol::Symbol::FnDecl(m_ir::FnDecl {
                        name,
                        r#type: m_ir::FnType {
                            ret: vec![ret_ty],
                            args: params_ty,
                        },
                        args: params,
                        body: builder.finish(),
                    })),
                });
            }
            "global_var_decl" => {
                let ident_node = node.required_field("name").of_kind("ident");
                let ident = ident_node.get_text(self.source);
                let name = self.registry.use_name(Some(ident));

                let mut builder = InstrBuilder::new(&mut self.registry, self.source);

                let ty = node.field("type").map_or(
                    m_ir::Type {
                        r#type: Some(m_ir::r#type::Type::Unknown(true)),
                    },
                    |ty| builder.parse_type(&ty),
                );

                builder
                    .registry
                    .insert_value_type(&name, ty.clone(), ValueTypeDeps::default());
                self.registry
                    .insert_value_type(&name, ty.clone(), ValueTypeDeps::default());

                let (value, _) = builder.add_expr(&node.required_field("value"), None);
                builder.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::BodyReturn(value.clone())),
                });

                if let m_ir::value::Value::Ident(ident) = value.value.as_ref().unwrap() {
                    builder.registry.set_value_type(&ident, ty, Some(&name));
                }

                let ty = builder.registry.value_type(&value);

                builder.registry.set_value_type(&name, ty.clone(), None);
                self.registry.set_value_type(&name, ty.clone(), None);

                // Names can't repeat between data implementations, because they will probably be
                // all implemented in the same scope
                self.registry.append_names(&builder.registry);

                self.symbols.push(m_ir::Symbol {
                    symbol: Some(m_ir::symbol::Symbol::DataDecl(m_ir::DataDecl {
                        name,
                        r#type: ty,
                        body: builder.finish(),
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

struct InstrBuilder<'a> {
    registry: Registry,
    body: Vec<m_ir::Instr>,
    source: &'a str,
}

impl<'a> InstrBuilder<'a> {
    pub fn new(parent_registry: &Registry, source: &'a str) -> Self {
        InstrBuilder {
            registry: Registry::with_parent(parent_registry),
            body: Vec::new(),
            source,
        }
    }

    pub fn finish(mut self) -> Vec<m_ir::Instr> {
        let name_value = |name: &str| m_ir::Value {
            value: Some(m_ir::value::Value::Ident(name.to_string())),
        };

        for inst in self.body.iter_mut() {
            // Instructions with a type should have their type loaded from the registry
            match inst.instr.as_mut() {
                Some(m_ir::instr::Instr::BinOp(binop)) => {
                    let value = name_value(&binop.name);
                    binop.r#type = self.registry.value_type(&value);
                }
                Some(m_ir::instr::Instr::FnCall(fncall)) => {
                    let value = name_value(&fncall.name);
                    fncall.r#type = self.registry.value_type(&value);
                }
                Some(m_ir::instr::Instr::Assign(assign)) => {
                    let value = name_value(&assign.name);
                    assign.r#type = self.registry.value_type(&value);
                }
                _ => {}
            }
        }

        self.body
    }

    pub fn add_stmt(&mut self, node: &ts::Node) -> (m_ir::Value, m_ir::Type) {
        match node.kind() {
            "var_decl" => {
                let var_name_node = node.required_field("pat").of_kind("ident");
                let var_name = var_name_node.get_text(self.source);
                let name = self.registry.use_name(Some(var_name));

                let (value, mut ty) = self.add_expr(&node.required_field("value"), Some(&name));

                if let Some(ty_node) = node.field("type") {
                    ty = self.parse_type(&ty_node);
                    self.registry.set_value_type(&name, ty.clone(), None);
                }

                (value, ty)
            }
            k => panic!("Found unexpected statement `{}`", k),
        }
    }

    pub fn add_expr(&mut self, node: &ts::Node, name: Option<&str>) -> (m_ir::Value, m_ir::Type) {
        match node.kind() {
            "number" => {
                let mut value = m_ir::Value {
                    value: Some(m_ir::value::Value::Num(
                        node.get_text(self.source).to_string(),
                    )),
                };
                // TODO: improve type handling
                let ty = self.registry.value_type(&value);

                if let Some(name) = name {
                    value = self.assign(name, value, ty.clone());
                }

                (value, ty)
            }
            "bin_op" => {
                let (left, left_ty) = self.add_expr(&node.required_field("left"), None);
                let (right, right_ty) = self.add_expr(&node.required_field("right"), None);

                // This will be implemented with typeclasses and generics
                // so + will be like `for T: Sum fn(T, T): T`
                // but none of this is implemented yet so we will use the number types instead, with
                // one signature for each type
                let num_ty = num_type("0");
                let ty = merge_types([&num_ty, &left_ty, &right_ty]).unwrap();
                let sigs = types_iter(&ty).map(binop_sig);

                let name = name.map_or_else(|| self.registry.use_name(None), |v| v.to_string());

                self.registry.insert_value_type(
                    &name,
                    ty.clone(),
                    ValueTypeDeps {
                        refs: vec![left.clone(), right.clone()],
                        sig: sigs.collect(),
                    },
                );

                let op_node = node.required_field("op");

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::BinOp(m_ir::BinOp {
                        name: name.clone(),
                        r#type: ty.clone(),
                        op: match op_node.get_text(self.source) {
                            "+" => m_ir::BinOpType::Add,
                            "-" => m_ir::BinOpType::Sub,
                            "%" => m_ir::BinOpType::Mod,
                            "*" => m_ir::BinOpType::Mul,
                            "/" => m_ir::BinOpType::Div,
                            "**" => m_ir::BinOpType::Pow,
                            _ => unreachable!(),
                        }
                        .into(),
                        left,
                        right,
                    })),
                });

                let value = m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(name)),
                };

                (value, ty)
            }
            "ident" => {
                let internal_name = self
                    .registry
                    .get_internal_name(node.get_text(self.source))
                    .unwrap();

                let mut value = m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(internal_name.to_string())),
                };

                let ty = self.registry.value_type(&value);

                if let Some(name) = name {
                    value = self.assign(name, value, ty.clone());
                }

                (value, ty)
            }
            "call" => {
                let mut args = Vec::new();
                for arg_node in node.iter_field("args") {
                    let (value, _) = self.add_expr(&arg_node, None);
                    args.push(value);
                }

                let (callee, callee_ty) = self.add_expr(&node.required_field("callee"), None);
                let callee_name = match callee.value {
                    Some(m_ir::value::Value::Ident(name)) => {
                        self.registry.get_internal_name(&name).unwrap().to_string()
                    }
                    _ => {
                        // TODO: improve error handling
                        unreachable!()
                    }
                };

                let fn_sigs: Vec<_> = types_iter(&callee_ty)
                    .filter_map(|ty| {
                        if let Some(m_ir::r#type::Type::Fn(fn_ty)) = ty.r#type.as_ref() {
                            Some(fn_ty.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                let ret_ty = ambig(
                    fn_sigs
                        .clone()
                        .into_iter()
                        // TODO: many return values
                        .map(|mut fn_ty| fn_ty.ret.remove(0)),
                );

                let name = name.map_or_else(|| self.registry.use_name(None), |v| v.to_string());

                self.registry.insert_value_type(
                    &name,
                    ret_ty.clone(),
                    ValueTypeDeps {
                        refs: args.clone(),
                        sig: fn_sigs,
                    },
                );

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::FnCall(m_ir::FnCall {
                        name: name.clone(),
                        r#type: ret_ty.clone(),
                        callee: callee_name,
                        args,
                    })),
                });

                let value = m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(name)),
                };

                (value, ret_ty.clone())
            }
            "block" => {
                for stmt_node in node.iter_field("body") {
                    self.add_stmt(&stmt_node);
                }

                self.add_expr(&node.required_field("value"), name)
            }
            k => panic!("Found unexpected expression `{}`", k),
        }
    }

    pub fn parse_type(&mut self, node: &ts::Node<'_>) -> m_ir::Type {
        match node.kind() {
            "ident" => {
                match node.get_text(self.source) {
                    "i8" => primitive!(I8),
                    "i16" => primitive!(I16),
                    "i32" => primitive!(I32),
                    "i64" => primitive!(I64),
                    "u8" => primitive!(U8),
                    "u16" => primitive!(U16),
                    "u32" => primitive!(U32),
                    "u64" => primitive!(U64),
                    "usize" => primitive!(USize),
                    "f32" => primitive!(F32),
                    "f64" => primitive!(F64),
                    _ => {
                        // TODO: improve error handling
                        panic!("{} is not a type, dummy", node.to_sexp());
                    }
                }
            }
            k => panic!("Found unexpected type `{}`", k),
        }
    }

    pub fn assign(&mut self, name: &str, value: m_ir::Value, ty: m_ir::Type) -> m_ir::Value {
        self.body.push(m_ir::Instr {
            instr: Some(m_ir::instr::Instr::Assign(m_ir::Assign {
                name: name.to_string(),
                r#type: ty.clone(),
                value,
            })),
        });

        self.registry
            .insert_value_type(&name, ty.clone(), ValueTypeDeps::default());

        m_ir::Value {
            value: Some(m_ir::value::Value::Ident(name.to_string())),
        }
    }
}
