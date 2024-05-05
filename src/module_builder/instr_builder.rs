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
    loaded_globals: HashMap<u32, u32>,
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
            loaded_globals: HashMap::new(),
        }
    }

    pub fn create_nested_builder(&mut self) -> InstrBuilder<'_, R> {
        InstrBuilder {
            body: Vec::new(),
            registry: self.registry,
            source: self.source,
            loaded_globals: self.loaded_globals.clone(),
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

    pub fn add_expr(&mut self, node: &ts::Node) -> (VirtualValue, mir::Type) {
        match node.kind() {
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
                let (items, items_types): (Vec<_>, Vec<_>) =
                    node.iter_field("items").map(|n| self.add_expr(&n)).unzip();

                let item_ty = mir::Type::merge(&items_types)
                    .expect("Array items must have same type");
                let ty = mir::Type::array_type(item_ty.clone(), Some(items.len()));

                (VirtualValue::Array(items), ty)
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
                    left: self.use_virtual_value(&left),
                    right: self.use_virtual_value(&right),
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
                    let (v_value, _) = self.add_expr(&arg_node);
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

                let (value, ty) = self.add_expr(&node.required_field("value"));

                self.registry.idents_mut().clear();
                self.registry.idents_mut().extend(old_idents);

                (value, ty)
            }
            "if" => {
                let (cond_value, _) = self.add_expr(&node.required_field("cond"));
                let cond_value = self.use_virtual_value(&cond_value);

                let (then_value, then_ty, mut then_body) =
                    if let Some(then_node) = &node.field("then") {
                        let mut nested = self.create_nested_builder();

                        let (then_v, then_ty) = nested.add_expr(then_node);
                        (nested.use_virtual_value(&then_v), then_ty, nested.body)
                    } else {
                        todo!();
                    };

                let (else_value, else_ty, mut else_body) =
                    if let Some(else_node) = &node.field("else") {
                        let mut nested = self.create_nested_builder();

                        let (else_v, else_ty) = nested.add_expr(else_node);
                        (nested.use_virtual_value(&else_v), else_ty, nested.body)
                    } else {
                        todo!();
                    };

                let ty = mir::Type::merge([&then_ty, &else_ty])
                    .expect(&format!("Type mismatch: {} and {}", then_ty, else_ty));

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

                // TODO: multi-value
                then_body.push(mir::Instr::Break(mir::BreakInstr {
                    count: 1,
                    values: vec![then_value],
                }));

                else_body.push(mir::Instr::Break(mir::BreakInstr {
                    count: 1,
                    values: vec![else_value],
                }));

                self.body.push(mir::Instr::If(mir::IfInstr {
                    target_idx_list: vec![local_idx],
                    cond: cond_value,
                    then_body,
                    else_body,
                }));

                (VirtualValue::Local(local_idx), ty)
            }
            k => panic!("Found unexpected expression `{}`", k),
        }
    }

    pub fn add_return(&mut self, node: &ts::Node, ty: Option<mir::Type>) -> mir::Type {
        match node.kind() {
            "block" => {
                for stmt_node in node.iter_field("body") {
                    self.add_stmt(&stmt_node);
                }

                let old_idents = self.registry.idents().clone();

                let ty = self.add_return(&node.required_field("value"), ty);

                self.registry.idents_mut().clear();
                self.registry.idents_mut().extend(old_idents);

                ty
            }
            "if" => {
                let (cond_v_value, _) = self.add_expr(&node.required_field("cond"));
                let cond_value = self.use_virtual_value(&cond_v_value);

                self.registry
                    .set_value_type(cond_v_value, mir::Type::Bool, None);

                let (then_ty, then_body) = if let Some(then_node) = &node.field("then") {
                    let mut nested = self.create_nested_builder();

                    let then_ty = nested.add_return(then_node, ty.clone());
                    (then_ty, nested.body)
                } else {
                    todo!();
                };

                let (else_ty, else_body) = if let Some(else_node) = &node.field("else") {
                    let mut nested = self.create_nested_builder();

                    let else_ty = nested.add_return(else_node, ty.clone());
                    (else_ty, nested.body)
                } else {
                    todo!();
                };

                self.body.push(mir::Instr::If(mir::IfInstr {
                    target_idx_list: vec![],
                    cond: cond_value,
                    then_body,
                    else_body,
                }));

                mir::Type::merge([&then_ty, &else_ty])
                    .expect(&format!("Type mismatch: {} and {}", then_ty, else_ty))
            }
            _ => {
                let (v_value, _) = self.add_expr(&node);

                let value = self.use_virtual_value(&v_value);
                self.body
                    .push(mir::Instr::Return(mir::ReturnInstr { value: Some(value) }));

                if let Some(ret) = ty {
                    self.registry.set_value_type(v_value.clone(), ret, None);
                }

                self.registry.value_type(&v_value).unwrap()
            }
        }
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
