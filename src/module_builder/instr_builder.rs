use std::fmt::Debug;

use tree_sitter as ts;

use super::registry::Registry;
use super::registry::ValueRef;
use super::registry::ValueTypeDeps;
use super::types::num_type;
use super::types::{ambig, binop_sig, merge_types, possible_types, primitive};
use crate::proto::m_ir;
use crate::tree_sitter_utils::TreeSitterUtils;

#[derive(Debug)]
pub struct InstrBuilder<'a, R>
where
    R: Registry + Debug,
{
    pub registry: &'a mut R,
    pub body: Vec<m_ir::Instr>,
    source: &'a str,
}

impl<'a, R> InstrBuilder<'a, R>
where
    R: Registry + Debug,
{
    pub fn new(registry: &'a mut R, source: &'a str) -> Self {
        InstrBuilder {
            registry,
            body: Vec::new(),
            source,
        }
    }

    pub fn finish(self) -> Vec<m_ir::Instr> {
        self.body
    }

    pub fn add_stmt(&mut self, node: &ts::Node) {
        match node.kind() {
            "var_decl" => {
                let var_name_node = node.required_field("pat").of_kind("ident");
                let var_name = var_name_node.get_text(self.source);

                let (value, _) = self.add_expr(&node.required_field("value"));
                let value_ref = value.into();

                self.registry.idents_mut().insert(var_name, value_ref);

                if let Some(ty_node) = node.field("type") {
                    let ty = self.parse_type(&ty_node);
                    self.registry.set_value_type(value_ref, ty, None);
                }
            }
            k => panic!("Found unexpected statement `{}`", k),
        }
    }

    pub fn add_expr(&mut self, node: &ts::Node) -> (m_ir::Value, m_ir::Type) {
        match node.kind() {
            "number" => {
                let number = node.get_text(self.source);
                let ty = num_type(number);

                let target_idx =
                    self.registry
                        .register_local("", ty.clone(), ValueTypeDeps::default());

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::Const(m_ir::Const {
                        target_idx,
                        value: Some(m_ir::r#const::Value::Number(number.to_string())),
                    })),
                });

                let value = m_ir::Value {
                    value: Some(m_ir::value::Value::Local(target_idx)),
                };

                (value, ty)
            }
            "bin_op" => {
                let (left, left_ty) = self.add_expr(&node.required_field("left"));
                let (right, right_ty) = self.add_expr(&node.required_field("right"));

                // This will be implemented with typeclasses and generics so will be
                // like `for T: Sum fn(T, T): T` but none of this is implemented yet so we
                // will use the number types instead, with one signature for each type
                let num_ty = num_type("0");
                let ty = merge_types([&num_ty, &left_ty, &right_ty]).unwrap();
                let sigs = possible_types(&ty).into_iter().map(binop_sig);

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: vec![left.clone().into(), right.clone().into()],
                        sig: sigs.collect(),
                    },
                );

                let op_node = node.required_field("op");

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::BinOp(m_ir::BinOp {
                        target_idx,
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
                    value: Some(m_ir::value::Value::Local(target_idx)),
                };

                (value, ty)
            }
            "ident" => {
                let ident = node.get_text(self.source);
                let ident_ref = self
                    .registry
                    .idents()
                    .get(ident)
                    .expect(&format!("Identifier `{}` not found", ident));

                let ty = self
                    .registry
                    .value_type(&ident_ref)
                    .expect(&format!("Type for identifier `{}` not found", ident));

                let value_inner = match &ident_ref {
                    ValueRef::Local(idx) => m_ir::value::Value::Local(*idx),
                    ValueRef::Param(idx) => m_ir::value::Value::Param(*idx),
                    ValueRef::Global(idx) => {
                        let local_idx = self.registry.register_local(
                            "",
                            ty.clone(),
                            ValueTypeDeps {
                                sig: vec![m_ir::FnType {
                                    args: vec![ty.clone()],
                                    ret: vec![ty.clone()],
                                }],
                                refs: vec![ident_ref.clone()],
                            },
                        );

                        self.body.push(m_ir::Instr {
                            instr: Some(m_ir::instr::Instr::LoadGlobal(m_ir::LoadGlobal {
                                target_idx: local_idx,
                                global_idx: *idx,
                            })),
                        });

                        m_ir::value::Value::Local(local_idx)
                    }
                    ValueRef::Func(_) => todo!(),
                };

                let value = m_ir::Value {
                    value: Some(value_inner),
                };

                (value, ty)
            }
            "call" => {
                let mut args = Vec::new();
                for arg_node in node.iter_field("args") {
                    let (value, _) = self.add_expr(&arg_node);
                    args.push(value);
                }

                let func_node = node.required_field("callee");

                let (func_idx, func_ty) = match func_node.kind() {
                    "ident" => {
                        let func_ref = self
                            .registry
                            .idents()
                            .get(func_node.get_text(self.source))
                            .unwrap();

                        let ty = self.registry.value_type(&func_ref).unwrap();

                        let ValueRef::Func(idx) = func_ref else {
                            // FIXME: improve error handling
                            unreachable!()
                        };

                        (idx, ty)
                    }
                    _ => todo!(),
                };

                let fn_sigs: Vec<_> = possible_types(&func_ty)
                    .into_iter()
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

                let target_idx = self.registry.register_local(
                    "",
                    ret_ty.clone(),
                    ValueTypeDeps {
                        refs: args.iter().map(|a| a.clone().into()).collect(),
                        sig: fn_sigs,
                    },
                );

                self.body.push(m_ir::Instr {
                    instr: Some(m_ir::instr::Instr::FnCall(m_ir::FnCall {
                        target_idx,
                        func_idx,
                        args,
                    })),
                });

                let value = m_ir::Value {
                    value: Some(m_ir::value::Value::Local(target_idx)),
                };

                (value, ret_ty.clone())
            }
            "block" => {
                for stmt_node in node.iter_field("body") {
                    self.add_stmt(&stmt_node);
                }

                let old_idents = self.registry.idents().clone();

                let (value, ty) = self.add_expr(&node.required_field("value"));

                self.registry.idents_mut().clear();
                self.registry.idents_mut().extend(old_idents);

                (value, ty)
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
}
