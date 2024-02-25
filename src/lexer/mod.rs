mod registry;

use self::registry::Registry;
use crate::proto::{ast, lex};

impl From<&ast::Module> for lex::Module {
    fn from(value: &ast::Module) -> Self {
        let mut lexer = Lexer::new(value.name.clone());

        for stmt in value.body.iter() {
            lexer.add_symbol(&stmt);
        }

        lexer.finish()
    }
}

pub struct Lexer {
    pub name: String,
    registry: Registry,
    symbols: Vec<lex::Symbol>,
}

impl Lexer {
    pub fn new(name: String) -> Self {
        Lexer {
            name,
            symbols: Vec::new(),
            registry: Registry::new(),
        }
    }

    pub fn add_symbol(&mut self, node: &ast::Stmt) {
        match &node.stmt {
            Some(ast::stmt::Stmt::Fn(func)) => {
                let name = self.registry.use_name(Some(&func.name));

                let mut builder = InstrBuilder::new(&mut self.registry);

                let mut args = Vec::new();
                let mut args_ty = Vec::new();
                for arg in func.args.iter() {
                    let ast::pat::Pat::Name(name_pat) = arg.pat.pat.as_ref().unwrap();
                    args.push(builder.registry.use_name(Some(name_pat)).to_string());

                    // FIXME: hardcoded type
                    let ty = Registry::num_type("0");

                    args_ty.push(ty.clone());
                    builder.registry.set_value_type(name_pat.clone(), ty);
                }

                let ty = lex::Type {
                    r#type: Some(lex::r#type::Type::Fn(lex::FnType {
                        ret: vec![lex::Type {
                            r#type: Some(lex::r#type::Type::Unknown(true)),
                        }],
                        args: args_ty.clone(),
                    })),
                };
                self.registry.set_value_type(name.to_string(), ty.clone());
                builder.registry.set_value_type(name.to_string(), ty);

                let ret_ty: Vec<lex::Type> = if let Some(ret) = &func.ret {
                    let (ret, ret_ty) = builder.add_expr(&ret, None);
                    builder.body.push(lex::Instr {
                        instr: Some(lex::instr::Instr::FnReturn(ret)),
                    });
                    vec![ret_ty]
                } else {
                    vec![]
                };

                let fn_ty = lex::FnType {
                    ret: ret_ty,
                    args: args_ty,
                };

                let ty = lex::Type {
                    r#type: Some(lex::r#type::Type::Fn(fn_ty.clone())),
                };
                self.registry.set_value_type(name.to_string(), ty.clone());
                builder.registry.set_value_type(name.to_string(), ty);

                self.symbols.push(lex::Symbol {
                    symbol: Some(lex::symbol::Symbol::FnDecl(lex::FnDecl {
                        name,
                        r#type: fn_ty,
                        args,
                        body: builder.finish(),
                    })),
                });
            }
            Some(ast::stmt::Stmt::Var(var)) => {
                let ast::pat::Pat::Name(original_name) = var.pat.pat.as_ref().unwrap();
                let name = self.registry.use_name(Some(original_name));

                let mut builder = InstrBuilder::new(&mut self.registry);

                let (value, ty) = builder.add_expr(&var.value, None);
                builder.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::BodyReturn(value)),
                });

                builder.registry.set_value_type(name.clone(), ty.clone());
                self.registry.set_value_type(name.clone(), ty.clone());

                // Names can't repeat between data implementations, because they will probably be
                // all implemented in the same scope
                self.registry.append_names(&builder.registry);

                self.symbols.push(lex::Symbol {
                    symbol: Some(lex::symbol::Symbol::DataDecl(lex::DataDecl {
                        name,
                        r#type: ty,
                        body: builder.finish(),
                    })),
                });
            }
            _ => {
                // FIXME: melhorar tratamento de erro
                unreachable!()
            }
        };
    }

    pub fn finish(self) -> lex::Module {
        lex::Module {
            name: self.name,
            symbols: self.symbols,
        }
    }
}

struct InstrBuilder {
    registry: Registry,
    body: Vec<lex::Instr>,
}

impl InstrBuilder {
    pub fn new(parent_registry: &Registry) -> Self {
        InstrBuilder {
            registry: Registry::with_parent(parent_registry),
            body: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<lex::Instr> {
        self.body
    }

    pub fn add_stmt(&mut self, node: &ast::Stmt) -> (lex::Value, lex::Type) {
        match &node.stmt {
            Some(ast::stmt::Stmt::Var(var)) => {
                let ast::pat::Pat::Name(name_pat) = var.pat.pat.as_ref().unwrap();
                let name = self.registry.use_name(Some(name_pat));

                let (value, ty) = self.add_expr(&var.value, Some(&name));

                self.registry.set_value_type(name, ty.clone());

                (value, ty)
            }
            Some(ast::stmt::Stmt::Fn(_)) => {
                todo!()
            }
            None => {
                // FIXME: melhorar tratamento de erro
                unreachable!()
            }
        }
    }

    pub fn add_expr(&mut self, expr: &ast::Expr, name: Option<&str>) -> (lex::Value, lex::Type) {
        match expr.expr.as_ref().unwrap() {
            ast::expr::Expr::Num(num) => {
                let mut value = lex::Value {
                    value: Some(lex::value::Value::Num(num.clone())),
                };
                // TODO: improve type handling
                let ty = self.registry.value_type(&value).unwrap();

                if let Some(name) = name {
                    value = self.assign(name, value, ty.clone());
                }

                return (value, ty);
            }
            ast::expr::Expr::BinOp(op) => {
                let (left, left_ty) = self.add_expr(&op.left, None);
                let (right, right_ty) = self.add_expr(&op.right, None);

                let ty = self.registry.merge_types(&left_ty, &right_ty);

                let name = name.map_or_else(|| self.registry.use_name(None), |v| v.to_string());
                self.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::BinOp(lex::BinOp {
                        name: name.clone(),
                        r#type: ty.clone(),
                        op: match op.op() {
                            ast::BinOpType::Add => lex::BinOpType::Add,
                            ast::BinOpType::Sub => lex::BinOpType::Sub,
                            ast::BinOpType::Mod => lex::BinOpType::Mod,
                            ast::BinOpType::Mul => lex::BinOpType::Mul,
                            ast::BinOpType::Div => lex::BinOpType::Div,
                            ast::BinOpType::Pow => lex::BinOpType::Pow,
                        }
                        .into(),
                        left,
                        right,
                    })),
                });

                let value = lex::Value {
                    value: Some(lex::value::Value::Ident(name)),
                };

                return (value, ty);
            }
            ast::expr::Expr::Ident(ident) => {
                let internal_name = self.registry.get_internal_name(ident).unwrap();

                let mut value = lex::Value {
                    value: Some(lex::value::Value::Ident(internal_name.to_string())),
                };

                let ty = self.registry.value_type(&value).unwrap();

                if let Some(name) = name {
                    value = self.assign(name, value, ty.clone());
                }

                return (value, ty);
            }
            ast::expr::Expr::FnCall(call) => {
                let mut args = Vec::new();
                for arg in call.args.iter() {
                    let (value, _) = self.add_expr(arg, None);
                    args.push(value);
                }

                let (callee, callee_ty) = self.add_expr(&call.callee, None);
                let callee_name = match callee.value {
                    Some(lex::value::Value::Ident(name)) => {
                        self.registry.get_internal_name(&name).unwrap().to_string()
                    }
                    _ => {
                        // TODO: improve error handling
                        unreachable!()
                    }
                };

                let ret_ty = if let Some(lex::r#type::Type::Fn(fn_ty)) = &callee_ty.r#type.as_ref()
                {
                    // TODO: many return values
                    &fn_ty.ret[0] as &lex::Type
                } else {
                    // TODO: improve error handling
                    unreachable!()
                };

                let name = name.map_or_else(|| self.registry.use_name(None), |v| v.to_string());

                self.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::FnCall(lex::FnCall {
                        name: name.clone(),
                        r#type: ret_ty.clone(),
                        callee: callee_name,
                        args,
                    })),
                });

                let value = lex::Value {
                    value: Some(lex::value::Value::Ident(name)),
                };

                return (value, ret_ty.clone());
            }
            ast::expr::Expr::Block(block) => {
                for stmt in block.body.iter() {
                    self.add_stmt(stmt);
                }

                return self.add_expr(&block.ret, name);
            }
            ast::expr::Expr::FnExpr(_) => {
                todo!()
            }
        };
    }

    pub fn assign(&mut self, name: &str, value: lex::Value, ty: lex::Type) -> lex::Value {
        self.body.push(lex::Instr {
            instr: Some(lex::instr::Instr::Assign(lex::Assign {
                name: name.to_string(),
                r#type: ty.clone(),
                value,
            })),
        });

        self.registry.set_value_type(name.to_string(), ty.clone());

        lex::Value {
            value: Some(lex::value::Value::Ident(name.to_string())),
        }
    }
}
