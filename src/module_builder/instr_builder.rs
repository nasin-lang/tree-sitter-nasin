use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;
use tree_sitter as ts;

use super::registry::Registry;
use super::registry::ValueTypeDeps;
use super::registry::VirtualValue;
use crate::mir;
use crate::tree_sitter_utils::TreeSitterUtils;
use crate::utils;

#[derive(Debug)]
pub struct InstrBuilder<'a> {
    pub registry: &'a mut Registry,
    pub body: Vec<mir::Instr>,
    pub source: &'a str,
    pub func_idx: Option<u32>,
    loaded_globals: HashMap<u32, u32>,
    tco: bool,
}

impl<'a> InstrBuilder<'a> {
    pub fn new(
        registry: &'a mut Registry,
        source: &'a str,
        func_idx: Option<u32>,
    ) -> Self {
        InstrBuilder {
            registry,
            body: Vec::new(),
            source,
            loaded_globals: HashMap::new(),
            func_idx,
            tco: false,
        }
    }

    pub fn create_nested_builder<'b>(&'b mut self) -> InstrBuilder<'b> {
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
                let sig: Vec<_> = self
                    .registry
                    .possible_types(&param.ty)
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

                self.registry
                    .idents
                    .insert(var_name.to_string(), v_value.clone());

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
                let ty = self.registry.create_num_type(number);

                (VirtualValue::Number(number.to_string()), ty)
            }
            "string_lit" => {
                let string = utils::decode_string_lit(
                    node.required_field("content").get_text(self.source),
                );
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

                let item_ty = self
                    .registry
                    .merge_types(&items_types)
                    .expect("Array items must have same type");
                let ty = self
                    .registry
                    .create_array_type(item_ty.clone(), Some(items.len()));

                (VirtualValue::Array(items), ty)
            }
            "record_lit" => {
                let (props, props_ty): (Vec<_>, Vec<_>) = node
                    .iter_field("fields")
                    .map(|n| {
                        let name =
                            n.required_field("name").get_text(self.source).to_string();
                        let (value, ty) = self.add_expr(&n.required_field("value"), None);
                        ((name.clone(), value), (name, ty))
                    })
                    .unzip();

                let ty = self.registry.create_object_type(props_ty);

                (VirtualValue::Record(props), ty)
            }
            "bin_op" => {
                let op_node = node.required_field("op");
                let op = op_node.get_text(self.source);

                let (left, left_ty) = self.add_expr(&node.required_field("left"), None);
                let left = self.use_virtual_value(&left, &left_ty);

                let (right, right_ty) =
                    self.add_expr(&node.required_field("right"), None);
                let right = self.use_virtual_value(&right, &right_ty);

                // This will be implemented with typeclasses and generics so will be
                // like `for T: Sum fn(T, T): T` but none of this is implemented yet so we
                // will use the number types instead, with one signature for each type
                let operand_ty =
                    self.registry.merge_types([&left_ty, &right_ty]).unwrap();
                let sig: Vec<_> = self
                    .registry
                    .possible_types(&operand_ty)
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
                let ty = self
                    .registry
                    .create_ambig_type(sig.iter().map(|s| s.ret[0].clone()));

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
                    .idents
                    .get(ident_text)
                    .expect(&format!("Identifier `{}` not found", ident_text))
                    .clone();

                let ty = self
                    .registry
                    .value_type(&v_value)
                    .expect(&format!("Type for identifier `{}` not found", ident_text));

                (v_value, ty)
            }
            "get_prop" => {
                let (parent, parent_ty) =
                    self.add_expr(&node.required_field("parent"), None);
                let parent = self.use_virtual_value(&parent, &parent_ty);
                let parent_v_value: VirtualValue = parent.clone().into();

                let prop_name_node = node.required_field("prop_name");
                let prop_name = prop_name_node.get_text(self.source).to_string();

                self.registry.set_value_type(
                    parent_v_value.clone(),
                    self.registry
                        .create_object_type([(prop_name.clone(), mir::Type::unknown())]),
                    None,
                );
                let parent_ty =
                    self.registry.value_type(&parent_v_value.clone()).unwrap();

                let Some(prop_ty) = self.registry.get_prop_type(&parent_ty, &prop_name)
                else {
                    panic!(
                        "Type {} doesn't have any property named {}",
                        parent_ty, prop_name
                    );
                };

                let local_idx = self.registry.register_local(
                    "",
                    prop_ty.clone(),
                    ValueTypeDeps {
                        sig: self
                            .registry
                            .possible_types(&parent_ty)
                            .into_iter()
                            .flat_map(|parent_ty| {
                                let prop_ty = self
                                    .registry
                                    .get_prop_type(&parent_ty, &prop_name)
                                    .unwrap();
                                self.registry
                                    .into_possible_types(prop_ty)
                                    .into_iter()
                                    .map(move |prop_ty| {
                                        mir::FuncType::new(
                                            vec![parent_ty.clone()],
                                            vec![prop_ty.clone()],
                                        )
                                    })
                            })
                            .collect(),
                        refs: vec![parent_v_value],
                    },
                );

                let Some((field_idx, _)) =
                    self.registry.get_type_field(&parent_ty, &prop_name)
                else {
                    // FIXME: to make this work with inferred types, I need to implement
                    // "uncertain" field indexes. This is the same problem of overloaded
                    // functions
                    todo!("property access of inferred types");
                };

                self.body.push(mir::Instr::LoadField(mir::LoadFieldInstr {
                    target_idx: local_idx,
                    field_idx,
                    source: parent,
                }));

                (VirtualValue::Local(local_idx), prop_ty)
            }
            "call" => {
                let mut args = Vec::new();
                for arg_node in node.iter_field("args") {
                    let (v_value, ty) = self.add_expr(&arg_node, None);
                    let value = self.use_virtual_value(&v_value, &ty);
                    args.push(value);
                }

                let func_node = node.required_field("callee");

                let (func_idx, func_ty) = match func_node.kind() {
                    "ident" => {
                        let func_name = func_node.get_text(self.source);
                        let func_v_value = self
                            .registry
                            .idents
                            .get(func_name)
                            .expect(&format!("Function `{}` not found", func_name))
                            .clone();

                        let ty = self.registry.value_type(&func_v_value).unwrap();

                        let VirtualValue::Func(idx) = func_v_value else {
                            // FIXME: improve error handling
                            unreachable!()
                        };

                        (idx, ty)
                    }
                    _ => todo!(),
                };

                let func_sigs: Vec<_> = self
                    .registry
                    .possible_types(&func_ty)
                    .into_iter()
                    .filter_map(|ty| match &ty {
                        mir::Type::Func(func_ty) => Some(func_ty.clone()),
                        _ => None,
                    })
                    .collect();

                let ret_ty = self.registry.create_ambig_type(
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

                let old_idents = self.registry.idents.clone();

                let (value, ty) =
                    self.add_expr(&node.required_field("value"), return_ty.clone());

                self.registry.idents.clear();
                self.registry.idents.extend(old_idents);

                (value, ty)
            }
            "if" => {
                let (cond_value, cond_ty) =
                    self.add_expr(&node.required_field("cond"), None);
                let cond_value = self.use_virtual_value(&cond_value, &cond_ty);

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
                                let value = nested.use_virtual_value(&v_value, &ty);

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
                                let value = nested.use_virtual_value(&v_value, &ty);

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

                let ty = self
                    .registry
                    .merge_types([&then_ty, &else_ty])
                    .expect(&format!("Type mismatch: {} and {}", then_ty, else_ty));

                let (target_idx_list, v_value) = if let Some(_) = return_ty.clone() {
                    (vec![], VirtualValue::Never)
                } else {
                    let local_idx = self.registry.register_local(
                        "",
                        ty.clone(),
                        ValueTypeDeps {
                            sig: self
                                .registry
                                .possible_types(&ty)
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
            let Some(ty) = self.registry.merge_types([&ty, &return_ty]) else {
                panic!("Type mismatch: {} and {}", &return_ty, &ty);
            };

            // These have special behavior for returning value
            if matches!(&value, VirtualValue::Never) {
                return (VirtualValue::Never, ty);
            }

            let value = self.use_virtual_value(&value, &ty);
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

    pub fn use_virtual_value(
        &mut self,
        v_value: &VirtualValue,
        ty: &mir::Type,
    ) -> mir::Value {
        let v_value_ty = self.registry.value_type(v_value).unwrap();

        if !self.registry.match_types([ty, &v_value_ty]) {
            panic!("Type mismatch:\n    {}\n    {}", ty, &v_value_ty)
        }

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

                self.loaded_globals.insert(*idx, local_idx);

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
                let ty = self.registry.create_num_type(n);

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
                let mir::Type::Array(mir::ArrayType { item: item_ty, .. }) = ty else {
                    unreachable!();
                };

                let items_values: Vec<_> = items
                    .iter()
                    .map(|item| self.use_virtual_value(item, item_ty))
                    .collect();

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: items_values
                            .iter()
                            .cloned()
                            .map(VirtualValue::from)
                            .collect(),
                        sig: self
                            .registry
                            .possible_types(&item_ty)
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
            VirtualValue::Record(fields) => {
                let mir::Type::TypeRef(type_ref) = ty else {
                    panic!("Record type should be known at this point");
                };

                let mir::TypeDefBody::Record(rec_ty) = &self
                    .registry
                    .typedef(*type_ref)
                    .expect("Type not defined")
                    .body
                else {
                    panic!("Expect record type");
                };

                let fields_def = rec_ty.fields.clone();

                let fields: Vec<_> = fields_def
                    .iter()
                    .map(|f| {
                        let (_, field) = fields
                            .iter()
                            .find(|(name, _)| name == &f.name)
                            .expect("Field not found");
                        self.use_virtual_value(field, &f.ty)
                    })
                    .collect();

                let fields_types = fields_def
                    .iter()
                    .map(|f| self.registry.possible_types(&f.ty).into_iter().cloned())
                    .multi_cartesian_product();

                let target_idx = self.registry.register_local(
                    "",
                    ty.clone(),
                    ValueTypeDeps {
                        refs: fields.iter().cloned().map(VirtualValue::from).collect(),
                        sig: fields_types
                            .map(|t| mir::FuncType::new(t, vec![ty.clone()]))
                            .collect(),
                    },
                );

                self.body.push(mir::Instr::CreateData(mir::CreateDataInstr {
                    target_idx,
                    items: fields,
                }));

                mir::Value::Local(target_idx)
            }
            VirtualValue::Never => panic!("VirtualValue::Never does not have value"),
        }
    }

    pub fn get_const_value(
        &self,
        v_value: &VirtualValue,
        ty: &mir::Type,
    ) -> Option<mir::ConstValue> {
        match v_value {
            VirtualValue::Bool(v) => Some(mir::ConstValue::Bool(*v)),
            VirtualValue::Number(v) => Some(mir::ConstValue::Number(v.clone())),
            VirtualValue::String(v) => Some(mir::ConstValue::String(v.clone())),
            VirtualValue::Array(values) => {
                let mir::Type::Array(mir::ArrayType { item: item_ty, .. }) = ty else {
                    return None;
                };

                let values = values
                    .iter()
                    .map(|v| self.get_const_value(v, item_ty))
                    .collect::<Option<Vec<_>>>()?;
                Some(mir::ConstValue::Array(values))
            }
            VirtualValue::Record(fields) => {
                let mir::Type::TypeRef(type_ref) = ty else {
                    return None;
                };

                let mir::TypeDefBody::Record(rec_ty) =
                    &self.registry.typedef(*type_ref)?.body
                else {
                    return None;
                };

                let values = rec_ty
                    .fields
                    .iter()
                    .map(|f| {
                        let (_, field) =
                            fields.iter().find(|(name, _)| name == &f.name)?;
                        self.get_const_value(field, &f.ty)
                    })
                    .collect::<Option<Vec<_>>>()?;
                Some(mir::ConstValue::Record(values))
            }
            _ => None,
        }
    }

    pub fn parse_type(&mut self, node: &ts::Node<'_>) -> mir::Type {
        match node.kind() {
            "ident" => {
                let ident = node.get_text(self.source);
                match self.registry.type_idents.get(ident) {
                    Some(ty) => ty.clone(),
                    None => {
                        // TODO: improve error handling
                        panic!("Type \"{ident}\" not found");
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
                self.registry.create_array_type(item_ty, len)
            }
            k => panic!("Found unexpected type `{}`", k),
        }
    }
}
