use std::collections::HashMap;

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

pub struct Lexer<'a> {
    pub name: String,
    names: NameRegistry<'a>,
    symbols: Vec<lex::Symbol>,
}

impl<'a> Lexer<'a> {
    pub fn new(name: String) -> Self {
        Lexer {
            name,
            symbols: Vec::new(),
            names: NameRegistry::new(),
        }
    }

    pub fn add_symbol(&mut self, node: &ast::Stmt) {
        match &node.stmt {
            Some(ast::stmt::Stmt::Fn(func)) => {
                let name = self.names.use_name(Some(&func.name));

                let mut builder = InstrBuilder::new(&self.names);
                let args = func
                    .args
                    .iter()
                    .map(|arg| {
                        let ast::pat::Pat::Name(name_pat) = arg.pat.pat.as_ref().unwrap();
                        builder.names.use_name(Some(&name_pat.name)).to_string()
                    })
                    .collect();

                if let Some(ret) = &func.ret {
                    let ret = builder.add_expr(&ret, None);
                    builder.body.push(lex::Instr {
                        instr: Some(lex::instr::Instr::FnReturn(ret)),
                    });
                }

                self.symbols.push(lex::Symbol {
                    symbol: Some(lex::symbol::Symbol::FnDecl(lex::FnDecl {
                        name,
                        args,
                        body: builder.finish(),
                    })),
                });
            }
            Some(ast::stmt::Stmt::Var(var)) => {
                let ast::pat::Pat::Name(original_name) = var.pat.pat.as_ref().unwrap();
                let name = self.names.use_name(Some(&original_name.name));

                let mut builder = InstrBuilder::new(&self.names);

                let value = builder.add_expr(&var.value, None);
                builder.body.push(lex::Instr {
                    instr: Some(lex::instr::Instr::BodyReturn(value)),
                });

                let builder_name_map = builder.names.name_map.clone();

                self.symbols.push(lex::Symbol {
                    symbol: Some(lex::symbol::Symbol::DataDecl(lex::DataDecl {
                        name,
                        body: builder.finish(),
                    })),
                });

                // Names can't repeat between data implementations, because they will probably be
                // all implemented in the same scope
                for (key, mut value) in builder_name_map.into_iter() {
                    self.names
                        .name_map
                        .entry(key.clone())
                        .and_modify(|current| current.append(&mut value))
                        .or_insert(value);
                }
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

/// Handles name dedup, anonymous variable naming and non-target specific name mangling
struct NameRegistry<'a> {
    parent: Option<&'a NameRegistry<'a>>,
    /// Maps original names to the internal ones. Whenever a name is shadowed, it's added to the
    /// Vec at the same entry, and the last one will be the one used when the name is referenced
    /// again
    name_map: HashMap<String, Vec<String>>,
}

impl<'a> NameRegistry<'a> {
    pub fn new() -> Self {
        NameRegistry {
            parent: None,
            name_map: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: &'a NameRegistry) -> Self {
        NameRegistry {
            parent: Some(parent),
            name_map: HashMap::new(),
        }
    }

    pub fn use_name(&mut self, original_name: Option<&str>) -> String {
        let original_name = original_name.unwrap_or("");
        let count = self.count_ident(original_name);

        let internal_name = if original_name == "" {
            format!("v{}", count + 1)
        } else {
            if count > 0 {
                format!("{}_{}", original_name, count)
            } else {
                original_name.to_string()
            }
        };

        if !self.name_map.contains_key(original_name) {
            self.name_map.insert(original_name.to_string(), Vec::new());
        }

        let internal_names = self.name_map.get_mut(original_name).unwrap();
        internal_names.push(internal_name.clone());

        internal_name
    }

    fn count_ident(&self, ident: &str) -> usize {
        let mut count = self
            .name_map
            .get(ident)
            .map(|names| names.len())
            .unwrap_or(0);

        if let Some(parent) = self.parent {
            count += parent.count_ident(ident);
        }

        count
    }
}

struct InstrBuilder<'a> {
    names: NameRegistry<'a>,
    body: Vec<lex::Instr>,
}

impl<'a> InstrBuilder<'a> {
    pub fn new(parent: &'a NameRegistry<'a>) -> Self {
        InstrBuilder {
            names: NameRegistry::new_with_parent(&parent),
            body: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<lex::Instr> {
        self.body
    }

    pub fn add_stmt(&mut self, node: &ast::Stmt) -> lex::Value {
        match &node.stmt {
            Some(ast::stmt::Stmt::Var(var)) => {
                let ast::pat::Pat::Name(name_pat) = var.pat.pat.as_ref().unwrap();
                let name = self.names.use_name(Some(&name_pat.name));
                self.add_expr(&var.value, Some(&name))
            }
            Some(ast::stmt::Stmt::Fn(_)) => {
                todo!()
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
                let name = name.map_or_else(|| self.names.use_name(None), |v| v.to_string());
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

                let name = name.map_or_else(|| self.names.use_name(None), |v| v.to_string());

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
            ast::expr::Expr::FnExpr(_) => {
                todo!()
            }
        };
    }
}
