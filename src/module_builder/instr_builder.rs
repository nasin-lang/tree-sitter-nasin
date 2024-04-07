use std::fmt::Debug;

use tree_sitter as ts;

use super::registry::Registry;
use super::registry::ValueRef;
use super::registry::ValueTypeDeps;
use crate::mir;
use crate::tree_sitter_utils::TreeSitterUtils;

#[derive(Debug)]
pub struct InstrBuilder<'a, R>
where
    R: Registry + Debug,
{
    pub registry: &'a mut R,
    pub body: Vec<mir::Instr>,
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

    pub fn finish(self) -> Vec<mir::Instr> {
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

    pub fn add_expr(&mut self, node: &ts::Node) -> (mir::Value, mir::Type) {
        match node.kind() {
            "number" => {
                let number = node.get_text(self.source);
                let ty = mir::Type::num_type(number);

                let target_idx =
                    self.registry
                        .register_local("", ty.clone(), ValueTypeDeps::default());

                self.body.push(mir::Instr::Const(mir::ConstInstr {
                    target_idx,
                    value: mir::ConstValue::Number(number.to_string()),
                }));

                let value = mir::Value::Local(target_idx);

                (value, ty)
            }
            "array_lit" => {
                let (items, items_types): (Vec<_>, Vec<_>) =
                    node.iter_field("items").map(|n| self.add_expr(&n)).unzip();

                let item_ty =
                    mir::Type::merge(&items_types).expect("Array items must have same type");
                let ty = mir::Type::array_type(item_ty.clone());

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: items.iter().cloned().map(ValueRef::from).collect(),
                        sig: item_ty
                            .possible_types()
                            .into_iter()
                            .map(|t| mir::FuncType::array_sig(&t, items.len()))
                            .collect(),
                    },
                );

                self.body
                    .push(mir::Instr::CreateArray(mir::CreateArrayInstr {
                        target_idx,
                        items,
                    }));

                let value = mir::Value::Local(target_idx);

                (value, ty)
            }
            "bin_op" => {
                let (left, left_ty) = self.add_expr(&node.required_field("left"));
                let (right, right_ty) = self.add_expr(&node.required_field("right"));

                // This will be implemented with typeclasses and generics so will be
                // like `for T: Sum fn(T, T): T` but none of this is implemented yet so we
                // will use the number types instead, with one signature for each type
                let num_ty = mir::Type::num_type("0");
                let ty = mir::Type::merge([&num_ty, &left_ty, &right_ty]).unwrap();
                let sigs = ty
                    .possible_types()
                    .into_iter()
                    .map(mir::FuncType::binop_sig);

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: vec![left.clone().into(), right.clone().into()],
                        sig: sigs.collect(),
                    },
                );

                let op_node = node.required_field("op");

                let bin_op_instr = mir::BinOpInstr {
                    target_idx,
                    left,
                    right,
                };

                self.body.push(match op_node.get_text(self.source) {
                    "+" => mir::Instr::Add(bin_op_instr),
                    "-" => mir::Instr::Sub(bin_op_instr),
                    "%" => mir::Instr::Mod(bin_op_instr),
                    "*" => mir::Instr::Mul(bin_op_instr),
                    "/" => mir::Instr::Div(bin_op_instr),
                    "**" => mir::Instr::Pow(bin_op_instr),
                    _ => unreachable!(),
                });

                (mir::Value::Local(target_idx), ty)
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

                let value = match &ident_ref {
                    ValueRef::Local(idx) => mir::Value::Local(*idx),
                    ValueRef::Param(idx) => mir::Value::Param(*idx),
                    ValueRef::Global(idx) => {
                        let local_idx = self.registry.register_local(
                            "",
                            ty.clone(),
                            ValueTypeDeps {
                                sig: vec![mir::FuncType::new(vec![ty.clone()], vec![ty.clone()])],
                                refs: vec![ident_ref.clone()],
                            },
                        );

                        self.body.push(mir::Instr::LoadGlobal(mir::LoadGlobalInstr {
                            target_idx: local_idx,
                            global_idx: *idx,
                        }));

                        mir::Value::Local(local_idx)
                    }
                    ValueRef::Func(_) => todo!(),
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
                        let func_name = func_node.get_text(self.source);
                        let func_ref = self
                            .registry
                            .idents()
                            .get(func_name)
                            .expect(&format!("Function `{}` not found", func_name));

                        let ty = self.registry.value_type(&func_ref).unwrap();

                        let ValueRef::Func(idx) = func_ref else {
                            // FIXME: improve error handling
                            unreachable!()
                        };

                        (idx, ty)
                    }
                    _ => todo!(),
                };

                let func_sigs: Vec<_> = func_ty
                    .possible_types()
                    .into_iter()
                    .filter_map(|ty| match &ty {
                        mir::Type::Func(func_ty) => Some(func_ty.clone()),
                        _ => None,
                    })
                    .collect();

                let ret_ty = mir::Type::ambig(
                    func_sigs
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
                        sig: func_sigs,
                    },
                );

                self.body.push(mir::Instr::Call(mir::CallInstr {
                    target_idx,
                    func_idx,
                    args,
                }));

                (mir::Value::Local(target_idx), ret_ty.clone())
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

    pub fn parse_type(&mut self, node: &ts::Node<'_>) -> mir::Type {
        match node.kind() {
            "ident" => {
                match node.get_text(self.source) {
                    "i8" => mir::Type::I8,
                    "i16" => mir::Type::I16,
                    "i32" => mir::Type::I32,
                    "i64" => mir::Type::I64,
                    "u8" => mir::Type::U8,
                    "u16" => mir::Type::U16,
                    "u32" => mir::Type::U32,
                    "u64" => mir::Type::U64,
                    "usize" => mir::Type::USize,
                    "f32" => mir::Type::F32,
                    "f64" => mir::Type::F64,
                    _ => {
                        // TODO: improve error handling
                        panic!("{} is not a type, dummy", node.to_sexp());
                    }
                }
            }
            "array_type" => {
                let item_ty = self.parse_type(&node.required_field("item_type"));
                mir::Type::Array(Box::new(item_ty))
            }
            k => panic!("Found unexpected type `{}`", k),
        }
    }
}
