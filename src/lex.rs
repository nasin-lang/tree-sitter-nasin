use std::collections::HashMap;
use std::rc::Rc;

use crate::proto::ast;
use crate::proto::lex;

impl From<&ast::Module> for lex::Module {
    fn from(value: &ast::Module) -> Self {
        let mut builder = InstrBuilder::new();

        for stmt in value.body.iter() {
            builder.add_stmt(&stmt);
        }

        lex::Module {
            name: value.name.clone(),
            body: builder.finish(),
        }
    }
}

#[derive(Default)]
struct InstrBuilder<'a> {
    parent: Option<&'a InstrBuilder<'a>>,
    names: Vec<Rc<str>>,
    ident_map: HashMap<String, Vec<Rc<str>>>,
    body: Vec<lex::Instr>,
}

impl<'a> InstrBuilder<'a> {
    pub fn new() -> Self {
        InstrBuilder::default()
    }

    pub fn new_with_parent(parent: &'a InstrBuilder) -> Self {
        InstrBuilder {
            parent: Some(parent),
            ..InstrBuilder::default()
        }
    }

    pub fn finish(self) -> Vec<lex::Instr> {
        self.body
    }

    pub fn add_name(&mut self, original_name: Option<&str>) -> String {
        let ident = original_name.unwrap_or("");
        let count = self.count_ident(ident);

        let name: Rc<str> = if let Some(original_name) = original_name {
            if count > 0 {
                format!("{}_{}", original_name, count).into()
            } else {
                original_name.into()
            }
        } else {
            format!("v{}", count + 1).into()
        };

        if !self.ident_map.contains_key(ident) {
            self.ident_map.insert(ident.to_string(), Vec::new());
        }
        let ident_names = self.ident_map.get_mut(ident).unwrap();

        ident_names.push(name.clone());
        self.names.push(name.clone());

        name.to_string()
    }

    fn count_ident(&self, ident: &str) -> usize {
        let mut count = self
            .ident_map
            .get(ident)
            .map(|names| names.len())
            .unwrap_or(0);

        if let Some(parent) = self.parent {
            count += parent.count_ident(ident);
        }

        count
    }

    pub fn add_stmt(&mut self, node: &ast::Stmt) -> lex::Value {
        match &node.stmt {
            Some(ast::stmt::Stmt::Var(var)) => {
                let ast::pat::Pat::Name(name_pat) = var.pat.pat.as_ref().unwrap();
                let name = self.add_name(Some(&name_pat.name));
                self.add_expr(&var.value, Some(&name))
            }
            Some(ast::stmt::Stmt::Fn(func)) => {
                let name = self.add_name(Some(&func.name));

                let mut fn_builder = InstrBuilder::new_with_parent(self);
                let args = func
                    .args
                    .iter()
                    .map(|arg| {
                        let ast::pat::Pat::Name(name_pat) = arg.pat.pat.as_ref().unwrap();
                        fn_builder.add_name(Some(&name_pat.name)).to_string()
                    })
                    .collect();

                if let Some(ret) = &func.ret {
                    let ret = lex::FnReturn {
                        value: fn_builder.add_expr(&ret, None),
                    };
                    fn_builder.body.push(lex::Instr {
                        instr: Some(lex::instr::Instr::FnReturn(ret)),
                    });
                }

                self.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::FnDecl(lex::FnDecl {
                        name: name.to_string(),
                        args,
                        body: fn_builder.finish(),
                    })),
                });

                lex::Value {
                    value: Some(lex::value::Value::Ident(name.to_string())),
                }
            }
            _ => {
                // FIXME: melhorar tratamento de erro
                unreachable!()
            }
        }
    }

    pub fn add_expr(&mut self, expr: &ast::Expr, name: Option<&str>) -> lex::Value {
        match expr.expr.as_ref().unwrap() {
            ast::expr::Expr::Num(num) => {
                let value = lex::Value {
                    value: Some(lex::value::Value::Num(lex::NumLit {
                        value: num.value.clone(),
                    })),
                };
                if let Some(name) = name {
                    self.body.push(lex::Instr {
                        instr: Some(lex::instr::Instr::Assign(lex::Assign {
                            name: name.to_string(),
                            value,
                        })),
                    });
                    return lex::Value {
                        value: Some(lex::value::Value::Ident(name.to_string())),
                    };
                }
                return value;
            }
            ast::expr::Expr::BinOp(op) => {
                let left = self.add_expr(&op.left, None);
                let right = self.add_expr(&op.right, None);
                let name = name.map_or_else(|| self.add_name(None), |v| v.to_string());
                self.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::BinOp(lex::BinOp {
                        name: name.clone(),
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
                return lex::Value {
                    value: Some(lex::value::Value::Ident(name)),
                };
            }
            ast::expr::Expr::Ident(ident) => {
                let value = lex::Value {
                    value: Some(lex::value::Value::Ident(ident.name.clone())),
                };
                if let Some(name) = name {
                    self.body.push(lex::Instr {
                        instr: Some(lex::instr::Instr::Assign(lex::Assign {
                            name: name.to_string(),
                            value,
                        })),
                    });
                    return lex::Value {
                        value: Some(lex::value::Value::Ident(name.to_string())),
                    };
                }
                return value;
            }
            ast::expr::Expr::FnCall(call) => {
                let mut args = Vec::new();
                for arg in call.args.iter() {
                    args.push(self.add_expr(arg, None));
                }

                let callee = match self.add_expr(&call.callee, None).value {
                    Some(lex::value::Value::Ident(name)) => name,
                    _ => {
                        // TODO: improve error handling
                        unreachable!()
                    }
                };

                let name = name.map_or_else(|| self.add_name(None), |v| v.to_string());

                self.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::FnCall(lex::FnCall {
                        name: name.clone(),
                        callee,
                        args,
                    })),
                });

                return lex::Value {
                    value: Some(lex::value::Value::Ident(name)),
                };
            }
            ast::expr::Expr::Block(block) => {
                for stmt in block.body.iter() {
                    self.add_stmt(stmt);
                }

                return self.add_expr(&block.ret, name);
            }
            _ => {
                // FIXME: melhorar tratamento de erro
                unreachable!()
            }
        };
    }
}
