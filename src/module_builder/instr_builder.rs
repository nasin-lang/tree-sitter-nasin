use std::collections::HashMap;
use std::fmt::Debug;

use tree_sitter as ts;

use super::registry::Registry;
use super::registry::ValueTypeDeps;
use super::registry::VirtualValue;
use crate::mir;
use crate::tree_sitter_utils::TreeSitterUtils;

#[derive(Debug)]
pub struct InstrBuilder<'a, R>
where
    R: Registry + Debug,
{
    pub registry: &'a mut R,
    pub body: Vec<mir::Instr>,
    pub source: &'a str,
    pub func_idx: Option<u32>,
    loaded_globals: HashMap<u32, u32>,
    tco: bool,
}

impl<'a, R> InstrBuilder<'a, R>
where
    R: Registry + Debug,
{
    pub fn new(registry: &'a mut R, source: &'a str, func_idx: Option<u32>) -> Self {
        InstrBuilder {
            registry,
            body: Vec::new(),
            source,
            loaded_globals: HashMap::new(),
            func_idx,
            tco: false,
        }
    }

    pub fn create_nested_builder(&mut self) -> InstrBuilder<'_, R> {
        InstrBuilder {
            body: Vec::new(),
            registry: self.registry,
            source: self.source,
            loaded_globals: self.loaded_globals.clone(),
            func_idx: self.func_idx,
            tco: self.tco,
        }
    }

    pub fn finish(mut self) -> Vec<mir::Instr> {
        if self.tco {
            let mut body = self.body;

            self.body = vec![];

            let params: Vec<_> = self.registry.get_params().collect();
            let mut updating_idx_list = vec![];

            for (param_idx, param) in params.into_iter().enumerate() {
                let sig: Vec<_> = param
                    .ty
                    .possible_types()
                    .into_iter()
                    .map(mir::FuncType::id_sig)
                    .collect();
                let target_idx = self.registry.register_local(
                    "",
                    param.ty.clone(),
                    ValueTypeDeps {
                        sig,
                        refs: vec![VirtualValue::Param(param_idx as u32)],
                    },
                );
                self.body.push(mir::Instr::Bind(mir::BindInstr {
                    target_idx,
                    value: mir::Value::Param(param_idx as u32),
                }));

                updating_idx_list.push(target_idx);
            }

            let param_locals: Vec<_> = updating_idx_list
                .iter()
                .map(|i| mir::Value::Local(*i))
                .collect();

            for ins in &mut body {
                ins.replace_params(&param_locals);
            }

            self.body.push(mir::Instr::Loop(mir::LoopInstr {
                target_idx_list: vec![],
                updating_idx_list,
                body,
            }));
        }

        self.body
    }

    pub fn add_stmt(&mut self, node: &ts::Node) {
        match node.kind() {
            "var_decl" => {
                let var_name_node = node.required_field("pat").of_kind("ident");
                let var_name = var_name_node.get_text(self.source);

                let (value, _) = self.add_expr(&node.required_field("value"), None);
                let v_value: VirtualValue = value.into();

                self.registry.idents_mut().insert(var_name, v_value.clone());

                if let Some(ty_node) = node.field("type") {
                    let ty = self.parse_type(&ty_node);
                    self.registry.set_value_type(v_value, ty, None);
                }
            }
            k => panic!("Found unexpected statement `{}`", k),
        }
    }

    pub fn add_expr(
        &mut self,
        node: &ts::Node,
        return_ty: Option<mir::Type>,
    ) -> (VirtualValue, mir::Type) {
        let (value, ty) = match node.kind() {
            "true" => (VirtualValue::Bool(true), mir::Type::Bool),
            "false" => (VirtualValue::Bool(false), mir::Type::Bool),
            "number" => {
                let number = node.get_text(self.source);
                let ty = mir::Type::num_type(number);

                (VirtualValue::Number(number.to_string()), ty)
            }
            "string_lit" => {
                let string = node
                    .required_field("content")
                    .get_text(self.source)
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\r", "\r")
                    .replace("\\\\", "\\")
                    .to_string();
                let ty = mir::Type::String(mir::StringType {
                    len: Some(string.len()),
                });

                (VirtualValue::String(string), ty)
            }
            "array_lit" => {
                let (items, items_types): (Vec<_>, Vec<_>) = node
                    .iter_field("items")
                    .map(|n| self.add_expr(&n, None))
                    .unzip();

                let item_ty = mir::Type::merge(&items_types)
                    .expect("Array items must have same type");
                let ty = mir::Type::array_type(item_ty.clone(), Some(items.len()));

                (VirtualValue::Array(items), ty)
            }
            "bin_op" => {
                let op_node = node.required_field("op");
                let op = op_node.get_text(self.source);

                let (left, left_ty) = self.add_expr(&node.required_field("left"), None);
                let left = self.use_virtual_value(&left);

                let (right, right_ty) =
                    self.add_expr(&node.required_field("right"), None);
                let right = self.use_virtual_value(&right);

                // This will be implemented with typeclasses and generics so will be
                // like `for T: Sum fn(T, T): T` but none of this is implemented yet so we
                // will use the number types instead, with one signature for each type
                let operand_ty = mir::Type::merge([&left_ty, &right_ty]).unwrap();
                let sig: Vec<_> = operand_ty
                    .possible_types()
                    .into_iter()
                    .map(|ty| match op {
                        "+" | "-" | "%" | "*" | "/" | "**" => {
                            mir::FuncType::binop_sig(ty, ty)
                        }
                        "==" | "!=" | ">" | "<" | ">=" | "<=" => {
                            mir::FuncType::binop_sig(ty, &mir::Type::Bool)
                        }
                        op => panic!("Unhandled binary operator: {op}"),
                    })
                    .collect();
                let ty = mir::Type::ambig(sig.iter().map(|s| s.ret[0].clone()));

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: vec![left.clone().into(), right.clone().into()],
                        sig,
                    },
                );

                let bin_op_instr = mir::BinOpInstr {
                    target_idx,
                    left,
                    right,
                };

                self.body.push(match op {
                    "+" => mir::Instr::Add(bin_op_instr),
                    "-" => mir::Instr::Sub(bin_op_instr),
                    "%" => mir::Instr::Mod(bin_op_instr),
                    "*" => mir::Instr::Mul(bin_op_instr),
                    "/" => mir::Instr::Div(bin_op_instr),
                    "**" => mir::Instr::Pow(bin_op_instr),
                    "==" => mir::Instr::Eq(bin_op_instr),
                    "!=" => mir::Instr::Neq(bin_op_instr),
                    ">" => mir::Instr::Gt(bin_op_instr),
                    "<" => mir::Instr::Lt(bin_op_instr),
                    ">=" => mir::Instr::Gte(bin_op_instr),
                    "<=" => mir::Instr::Lte(bin_op_instr),
                    op => panic!("Unhandled binary operator: {op}"),
                });

                (VirtualValue::Local(target_idx), ty)
            }
            "ident" => {
                let ident_text = node.get_text(self.source);
                let v_value = self
                    .registry
                    .idents()
                    .get(ident_text)
                    .expect(&format!("Identifier `{}` not found", ident_text));

                let ty = self
                    .registry
                    .value_type(&v_value)
                    .expect(&format!("Type for identifier `{}` not found", ident_text));

                (v_value, ty)
            }
            "call" => {
                let mut args = Vec::new();
                for arg_node in node.iter_field("args") {
                    let (v_value, _) = self.add_expr(&arg_node, None);
                    let value = self.use_virtual_value(&v_value);
                    args.push(value);
                }

                let func_node = node.required_field("callee");

                let (func_idx, func_ty) = match func_node.kind() {
                    "ident" => {
                        let func_name = func_node.get_text(self.source);
                        let func_v_value = self
                            .registry
                            .idents()
                            .get(func_name)
                            .expect(&format!("Function `{}` not found", func_name));

                        let ty = self.registry.value_type(&func_v_value).unwrap();

                        let VirtualValue::Func(idx) = func_v_value else {
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
                        .map(|mut f| f.ret.remove(0)),
                );

                if let Some(self_func_idx) = self.func_idx {
                    if self_func_idx == func_idx {
                        self.tco = true;
                        self.body.push(mir::Instr::Continue(mir::ContinueInstr {
                            count: 1,
                            values: args,
                        }));

                        return (VirtualValue::Never, ret_ty);
                    }
                }

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

                (VirtualValue::Local(target_idx), ret_ty)
            }
            "block" => {
                for stmt_node in node.iter_field("body") {
                    self.add_stmt(&stmt_node);
                }

                let old_idents = self.registry.idents().clone();

                let (value, ty) =
                    self.add_expr(&node.required_field("value"), return_ty.clone());

                self.registry.idents_mut().clear();
                self.registry.idents_mut().extend(old_idents);

                (value, ty)
            }
            "if" => {
                let (cond_value, _) = self.add_expr(&node.required_field("cond"), None);
                let cond_value = self.use_virtual_value(&cond_value);

                self.registry.set_value_type(
                    cond_value.clone().into(),
                    mir::Type::Bool,
                    None,
                );

                let (then_value, then_ty, then_body) =
                    if let Some(then_node) = &node.field("then") {
                        let (value, ty, body, tco) = {
                            let mut nested = self.create_nested_builder();

                            let (value, ty) = if let Some(return_ty) = return_ty.clone() {
                                let (_, ty) = nested.add_expr(then_node, Some(return_ty));
                                (VirtualValue::Never, ty)
                            } else {
                                let (v_value, ty) = nested.add_expr(then_node, None);
                                let value = nested.use_virtual_value(&v_value);

                                // TODO: multi-value
                                nested.body.push(mir::Instr::Break(mir::BreakInstr {
                                    count: 1,
                                    values: vec![value.clone()],
                                }));

                                (value.into(), ty)
                            };

                            (value, ty, nested.body, nested.tco)
                        };

                        self.tco = tco;
                        (value, ty, body)
                    } else {
                        todo!();
                    };

                let (else_value, else_ty, else_body) =
                    if let Some(else_node) = &node.field("else") {
                        let (value, ty, body, tco) = {
                            let mut nested = self.create_nested_builder();

                            let (value, ty) = if let Some(return_ty) = return_ty.clone() {
                                let (_, ty) = nested.add_expr(else_node, Some(return_ty));
                                (VirtualValue::Never, ty)
                            } else {
                                let (v_value, ty) = nested.add_expr(else_node, None);
                                let value = nested.use_virtual_value(&v_value);

                                // TODO: multi-value
                                nested.body.push(mir::Instr::Break(mir::BreakInstr {
                                    count: 1,
                                    values: vec![value.clone()],
                                }));

                                (VirtualValue::from(value), ty)
                            };

                            (value, ty, nested.body, nested.tco)
                        };

                        self.tco = tco;
                        (value, ty, body)
                    } else {
                        todo!();
                    };

                let ty = mir::Type::merge([&then_ty, &else_ty])
                    .expect(&format!("Type mismatch: {} and {}", then_ty, else_ty));

                let (target_idx_list, v_value) = if let Some(_) = return_ty.clone() {
                    (vec![], VirtualValue::Never)
                } else {
                    let local_idx = self.registry.register_local(
                        "",
                        ty.clone(),
                        ValueTypeDeps {
                            sig: ty
                                .possible_types()
                                .into_iter()
                                .map(|ty| mir::FuncType::if_sig(ty))
                                .collect(),
                            refs: vec![
                                cond_value.clone().into(),
                                then_value.clone().into(),
                                else_value.clone().into(),
                            ],
                        },
                    );

                    (vec![local_idx], VirtualValue::Local(local_idx))
                };

                self.body.push(mir::Instr::If(mir::IfInstr {
                    target_idx_list,
                    cond: cond_value,
                    then_body,
                    else_body,
                }));

                (v_value, ty)
            }
            k => panic!("Found unexpected expression `{}`", k),
        };

        if let Some(return_ty) = return_ty {
            // These have special behavior for returning value
            if matches!(&value, VirtualValue::Never) {
                return (VirtualValue::Never, ty);
            }

            let value = self.use_virtual_value(&value);
            let v_value: VirtualValue = value.clone().into();

            self.body.push(mir::Instr::Return(mir::ReturnInstr {
                value: Some(value.clone()),
            }));

            self.registry
                .set_value_type(v_value.clone(), return_ty, None);

            let ty = self.registry.value_type(&v_value).unwrap();
            return (VirtualValue::Never, ty);
        }

        (value, ty)
    }

    pub fn use_virtual_value(&mut self, v_value: &VirtualValue) -> mir::Value {
        let ty = self.registry.value_type(v_value).unwrap();

        match v_value {
            VirtualValue::Local(idx) => mir::Value::Local(*idx),
            VirtualValue::Param(idx) => mir::Value::Param(*idx),
            VirtualValue::Global(idx) => {
                if let Some(local_idx) = self.loaded_globals.get(idx) {
                    return mir::Value::Local(*local_idx);
                }

                let local_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    // FIXME: allow local to constrain the type of the global
                    ValueTypeDeps {
                        sig: vec![],
                        refs: vec![],
                    },
                );

                self.body.push(mir::Instr::LoadGlobal(mir::LoadGlobalInstr {
                    target_idx: local_idx,
                    global_idx: *idx,
                }));

                mir::Value::Local(local_idx)
            }
            VirtualValue::Func(_idx) => todo!(),
            VirtualValue::Bool(b) => {
                let ty = mir::Type::Bool;

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps::default(),
                );

                self.body.push(mir::Instr::CreateBool(mir::CreateBoolInstr {
                    target_idx,
                    value: *b,
                }));

                mir::Value::Local(target_idx)
            }
            VirtualValue::Number(n) => {
                let ty = mir::Type::num_type(n);

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps::default(),
                );

                self.body
                    .push(mir::Instr::CreateNumber(mir::CreateNumberInstr {
                        target_idx,
                        value: n.to_string(),
                    }));

                mir::Value::Local(target_idx)
            }
            VirtualValue::String(s) => {
                let ty = mir::Type::String(mir::StringType { len: Some(s.len()) });

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps::default(),
                );

                self.body
                    .push(mir::Instr::CreateString(mir::CreateStringInstr {
                        target_idx,
                        value: s.clone(),
                    }));

                mir::Value::Local(target_idx)
            }
            VirtualValue::Array(items) => {
                let items_values: Vec<_> = items
                    .iter()
                    .map(|item| self.use_virtual_value(item))
                    .collect();

                let mir::Type::Array(mir::ArrayType { item: item_ty, .. }) = &ty else {
                    unreachable!();
                };

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: items.clone(),
                        sig: item_ty
                            .possible_types()
                            .into_iter()
                            .map(|t| mir::FuncType::array_sig(&t, items.len()))
                            .collect(),
                    },
                );

                self.body.push(mir::Instr::CreateData(mir::CreateDataInstr {
                    target_idx,
                    items: items_values,
                }));

                mir::Value::Local(target_idx)
            }
            VirtualValue::Never => panic!("VirtualValue::Never does not have value"),
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
                    "str" => mir::Type::String(mir::StringType { len: None }),
                    _ => {
                        // TODO: improve error handling
                        panic!("{} is not a type, dummy", node.get_text(self.source));
                    }
                }
            }
            "array_type" => {
                let item_ty = self.parse_type(&node.required_field("item_type"));
                let len = node.field("length").map(|n| {
                    n.get_text(self.source)
                        .parse::<usize>()
                        .expect("Cannot cast length to integer")
                });
                mir::Type::array_type(item_ty, len)
            }
            k => panic!("Found unexpected type `{}`", k),
        }
    }
}
