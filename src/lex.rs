use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

use crate::proto::ast;
use crate::proto::m_ir;

impl From<&ast::Module> for m_ir::Module {
    fn from(value: &ast::Module) -> Self {
        let mut builder = InstrBuilder::new();

        for stmt in value.body.iter() {
            builder.add_stmt(&stmt);
        }

        m_ir::Module {
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
    body: Vec<m_ir::Instr>,
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

    pub fn finish(self) -> Vec<m_ir::Instr> {
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

    pub fn add_stmt(&mut self, node: &ast::Stmt) -> m_ir::Value {
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
                    let ret = m_ir::FnReturn {
                        value: fn_builder.add_expr(&ret, None),
                    };
                    fn_builder.body.push(m_ir::Instr {
                        instr: Some(m_ir::instr::Instr::FnReturn(ret)),
                    });
                }

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::FnDecl(m_ir::FnDecl {
                        name: name.to_string(),
                        args,
                        body: fn_builder.finish(),
                    })),
                });

                m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(name.to_string())),
                }
            }
            _ => {
                // FIXME: melhorar tratamento de erro
                unreachable!()
            }
        }
    }

    pub fn add_expr(&mut self, expr: &ast::Expr, name: Option<&str>) -> m_ir::Value {
        match expr.expr.as_ref().unwrap() {
            ast::expr::Expr::Num(num) => {
                let value = m_ir::Value {
                    value: Some(m_ir::value::Value::Num(m_ir::NumLit {
                        value: num.value.clone(),
                    })),
                };
                if let Some(name) = name {
                    self.body.push(m_ir::Instr {
                        instr: Some(m_ir::instr::Instr::Assign(m_ir::Assign {
                            name: name.to_string(),
                            value,
                        })),
                    });
                    return m_ir::Value {
                        value: Some(m_ir::value::Value::Ident(name.to_string())),
                    };
                }
                return value;
            }
            ast::expr::Expr::BinOp(op) => {
                let left = self.add_expr(&op.left, None);
                let right = self.add_expr(&op.right, None);
                let name = name.map_or_else(|| self.add_name(None), |v| v.to_string());
                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::BinOp(m_ir::BinOp {
                        name: name.clone(),
                        op: match op.op() {
                            ast::BinOpType::Add => m_ir::BinOpType::Add,
                            ast::BinOpType::Sub => m_ir::BinOpType::Sub,
                            ast::BinOpType::Mod => m_ir::BinOpType::Mod,
                            ast::BinOpType::Mul => m_ir::BinOpType::Mul,
                            ast::BinOpType::Div => m_ir::BinOpType::Div,
                            ast::BinOpType::Pow => m_ir::BinOpType::Pow,
                        }
                        .into(),
                        left,
                        right,
                    })),
                });
                return m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(name)),
                };
            }
            ast::expr::Expr::Ident(ident) => {
                let value = m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(ident.name.clone())),
                };
                if let Some(name) = name {
                    self.body.push(m_ir::Instr {
                        instr: Some(m_ir::instr::Instr::Assign(m_ir::Assign {
                            name: name.to_string(),
                            value,
                        })),
                    });
                    return m_ir::Value {
                        value: Some(m_ir::value::Value::Ident(name.to_string())),
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
                    Some(m_ir::value::Value::Ident(name)) => name,
                    _ => {
                        // TODO: improve error handling
                        unreachable!()
                    }
                };

                let name = name.map_or_else(|| self.add_name(None), |v| v.to_string());

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::FnCall(m_ir::FnCall {
                        name: name.clone(),
                        callee,
                        args,
                    })),
                });

                return m_ir::Value {
                    value: Some(m_ir::value::Value::Ident(name)),
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

fn fmt_scope(scope: &[m_ir::Instr], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for instr in scope.iter() {
        for line in format!("{}", instr).lines() {
            write!(f, "    {}\n", line)?;
        }
    }
    Ok(())
}

impl Display for m_ir::Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mod {}\n", self.name)?;
        fmt_scope(&self.body, f)
    }
}

impl Display for m_ir::Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instr {
            Some(m_ir::instr::Instr::FnDecl(func)) => {
                write!(f, "fn {}({}) =\n", func.name, func.args.join(", "),)?;
                fmt_scope(&func.body, f)
            }
            Some(m_ir::instr::Instr::Assign(assign)) => {
                write!(f, "{} := {}", assign.name, assign.value)
            }
            Some(m_ir::instr::Instr::BinOp(op)) => write!(
                f,
                "{} = {} {} {}",
                op.name,
                op.left,
                match op.op() {
                    m_ir::BinOpType::Add => "+",
                    m_ir::BinOpType::Sub => "-",
                    m_ir::BinOpType::Mod => "%",
                    m_ir::BinOpType::Mul => "*",
                    m_ir::BinOpType::Div => "/",
                    m_ir::BinOpType::Pow => "**",
                },
                op.right
            ),
            Some(m_ir::instr::Instr::FnCall(call)) => write!(
                f,
                "{} = {}({})",
                call.name,
                call.callee,
                call.args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Some(m_ir::instr::Instr::FnReturn(ret)) => {
                write!(f, "return {}", ret.value)
            }
            None => Ok(()),
        }
    }
}

impl Display for m_ir::Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(m_ir::value::Value::Num(num)) => write!(f, "{}", num.value),
            Some(m_ir::value::Value::Ident(ident)) => write!(f, "{}", ident),
            None => Ok(()),
        }
    }
}
