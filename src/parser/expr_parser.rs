use std::collections::HashMap;
use std::usize;

use derive_new::new;
use itertools::Itertools;
use tree_sitter as ts;

use super::module_parser::ModuleParser;
use super::parser_value::{ValueRef, ValueRefBody};
use crate::bytecode::Loc;
use crate::utils::{SortedMap, TreeSitterUtils, ValueStack};
use crate::{bytecode as b, context, errors, utils};

type Stack = ValueStack<b::ValueIdx, ScopePayload>;

pub struct ExprParser<'a, 't> {
    pub module_parser: ModuleParser<'a, 't>,
    pub instrs: Vec<b::Instr>,
    pub is_loop: bool,
    pub idents: HashMap<String, ValueRef>,
    ctx: &'a context::BuildContext,
    src_idx: usize,
    mod_idx: usize,
    func_idx: Option<usize>,
    stack: Stack,
}

impl<'a, 't> ExprParser<'a, 't> {
    pub fn new(
        ctx: &'a context::BuildContext,
        module_parser: ModuleParser<'a, 't>,
        src_idx: usize,
        mod_idx: usize,
        func_idx: Option<usize>,
        inputs: impl IntoIterator<Item = (String, b::ValueIdx, b::Loc)>,
    ) -> Self {
        let mut idents = module_parser.idents.clone();

        let mut stack = Stack::new(ScopePayload::new(idents.clone()));
        for (i, (ident, v, loc)) in inputs.into_iter().enumerate() {
            idents.insert(ident, ValueRef::new(ValueRefBody::Value(i, v), loc));
            stack.push(v);
        }

        ExprParser {
            ctx,
            mod_idx,
            src_idx,
            module_parser,
            idents,
            func_idx,
            instrs: vec![],
            is_loop: false,
            stack,
        }
    }

    pub fn finish(mut self) -> (ModuleParser<'a, 't>, Vec<b::Instr>) {
        assert!(self.stack.scope_len() == 1);

        if self.is_loop {
            let func = &self.module_parser.funcs[self.func_idx.unwrap()].func;
            self.instrs.insert(
                0,
                b::Instr::new(
                    b::InstrBody::Loop(b::Type::unknown(None), func.params_desc.len()),
                    self.instrs[0].loc,
                ),
            );
            self.instrs
                .push(b::Instr::new(b::InstrBody::End, self.instrs[0].loc));
        }
        (self.module_parser, self.instrs)
    }

    pub fn add_expr_node(
        &mut self,
        node: ts::Node<'t>,
        target: Option<b::ValueIdx>,
        returning: bool,
    ) -> ValueRef {
        let loc = Loc::from_node(self.src_idx, &node);
        match node.kind() {
            "true" => ValueRef::new(ValueRefBody::Bool(true), loc),
            "false" => ValueRef::new(ValueRefBody::Bool(false), loc),
            "number" => {
                let number = node.get_text(&self.ctx.source(self.src_idx).content().text);
                ValueRef::new(ValueRefBody::Number(number.to_string()), loc)
            }
            "string_lit" => {
                let string = utils::decode_string_lit(
                    node.required_field("content")
                        .get_text(&self.ctx.source(self.src_idx).content().text),
                );
                let (idx, v) = self.add_instr_with_result_and_push(
                    b::Instr::new(b::InstrBody::CreateString(string), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(idx, v), loc)
            }
            "array_lit" => {
                let items: Vec<_> = node
                    .iter_field("items")
                    .map(|item_node| self.add_expr_node(item_node, None, false))
                    .collect();
                for item in &items {
                    self.push_value_ref(&item, None);
                }
                let ty = b::Type::new(
                    b::TypeBody::Array(b::ArrayType::new(
                        b::Type::unknown(None).into(),
                        Some(items.len()),
                    )),
                    None,
                );
                self.stack.pop_many(items.len());
                let (idx, v) = self.add_instr_with_result_and_push(
                    b::Instr::new(b::InstrBody::CreateArray(ty, items.len()), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(idx, v), loc)
            }
            "record_lit" => {
                let fields: utils::SortedMap<_, _> = node
                    .iter_field("fields")
                    .map(|field_node| {
                        let field_name = field_node
                            .required_field("name")
                            .get_text(&self.ctx.source(self.src_idx).content().text);
                        let field_value = self.add_expr_node(
                            field_node.required_field("value"),
                            None,
                            false,
                        );
                        (field_name.to_string(), field_value)
                    })
                    .collect();
                for value in fields.values() {
                    self.push_value_ref(value, None);
                }
                let members: SortedMap<_, _> = fields
                    .keys()
                    .map(|k| (k.clone(), b::Type::unknown(None)))
                    .collect();
                let ty = b::Type::new(
                    b::TypeBody::Inferred(b::InferredType {
                        properties: members.clone(),
                        members,
                    }),
                    None,
                );
                self.stack.pop_many(fields.len());
                let (idx, v) = self.add_instr_with_result_and_push(
                    b::Instr::new(
                        b::InstrBody::CreateRecord(ty, fields.keys().cloned().collect()),
                        loc,
                    ),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(idx, v), loc)
            }
            "ident" => {
                let ident = node.get_text(&self.ctx.source(self.src_idx).content().text);
                if let Some(value) = self.idents.get(ident) {
                    value.with_loc(loc)
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::ValueNotFound::new(ident.to_string()).into(),
                        loc,
                    ));
                    ValueRef::new(ValueRefBody::CompileError, loc)
                }
            }
            "un_op" => {
                let op = node.required_field("op");
                let operand =
                    self.add_expr_node(node.required_field("operand"), None, false);
                self.add_un_op(op, operand, target)
            }
            "bin_op" => {
                let op = node.required_field("op");
                let left = self.add_expr_node(node.required_field("left"), None, false);
                let right = self.add_expr_node(node.required_field("right"), None, false);
                self.add_bin_op(op, left, right, target)
            }
            "type_bind" => {
                let mut value =
                    self.add_expr_node(node.required_field("value"), target, returning);
                let ty = self
                    .module_parser
                    .types
                    .parse_type_expr(node.required_field("type"));
                value.ty = Some(ty);
                value
            }
            "get_prop" => {
                let parent =
                    self.add_expr_node(node.required_field("parent"), None, false);
                let prop_name_node = node.required_field("prop_name");
                let prop_name = prop_name_node
                    .get_text(&self.ctx.source(self.src_idx).content().text);
                self.add_get_prop(
                    parent,
                    prop_name,
                    Loc::from_node(self.src_idx, &prop_name_node),
                    target,
                )
            }
            "call" => {
                let callee =
                    self.add_expr_node(node.required_field("callee"), None, false);
                let args: Vec<_> = node
                    .iter_field("args")
                    .map(|arg_node| self.add_expr_node(arg_node, None, false))
                    .collect();
                self.add_call(callee, args, loc, target, returning)
            }
            "block" => {
                let old_idents = self.idents.clone();

                for stmt_node in node.iter_field("body") {
                    self.add_stmt_node(stmt_node);
                }
                let value =
                    self.add_expr_node(node.required_field("value"), target, returning);

                self.idents = old_idents;

                value
            }
            "if" => {
                let cond_value =
                    self.add_expr_node(node.required_field("cond"), None, false);
                self.push_value_ref(&cond_value, None);
                self.stack.pop(); // consume condition

                let stack_len = self.stack.len();
                let block_len = self.stack.scope_len();

                let loc = b::Loc::from_node(self.src_idx, &node);
                let v = target.unwrap_or_else(|| {
                    self.module_parser.create_value(b::Type::unknown(None), loc)
                });
                self.stack
                    .create_scope(ScopePayload::new(self.idents.clone()));

                self.instrs.push(b::Instr::new(b::InstrBody::If(v), loc));
                let then_value =
                    self.add_expr_node(node.required_field("then"), Some(v), returning);

                if !then_value.is_never() {
                    self.push_value_ref(&then_value, Some(v));
                    assert!(self.stack.len() >= stack_len + 1);
                }

                let else_value = if let Some(else_node) = node.field("else") {
                    assert!(self.stack.scope_len() >= block_len + 1);
                    self.instrs.push(b::Instr::new(
                        b::InstrBody::Else,
                        Loc::from_node(self.src_idx, &else_node),
                    ));

                    let (scope, _) = self.stack.branch_scope();
                    self.idents = scope.payload.idents.clone();

                    let else_value = self.add_expr_node(else_node, Some(v), returning);

                    if !else_value.is_never() {
                        self.push_value_ref(&else_value, Some(v));
                        assert!(self.stack.len() >= stack_len + 1);
                    }

                    else_value
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::Todo::new("if without else".to_string()).into(),
                        loc,
                    ));
                    ValueRef::new(ValueRefBody::CompileError, loc)
                };

                assert!(self.stack.scope_len() >= block_len + 1);
                let (scope, _) = self.stack.end_scope();
                self.idents = scope.payload.idents;

                if !then_value.is_never() || !else_value.is_never() {
                    self.instrs.push(b::Instr::new(
                        b::InstrBody::End,
                        Loc::from_node(self.src_idx, &node),
                    ));
                    self.stack.push(v);
                    ValueRef::new(ValueRefBody::Value(self.stack.len() - 1, v), loc)
                } else {
                    ValueRef::new(ValueRefBody::Never, loc)
                }
            }
            "macro" => {
                let name = node
                    .required_field("name")
                    .of_kind("ident")
                    .get_text(&self.ctx.source(self.src_idx).content().text);
                let args = node.iter_field("args").collect_vec();
                self.add_macro(
                    name,
                    &args,
                    b::Loc::from_node(self.src_idx, &node),
                    target,
                )
            }
            k => panic!("Found unexpected expression `{}`", k),
        }
    }

    /// Push a value reference to the stack. This will add the necessary instruction to
    /// have the value accessible in the stack. `target` specifies the value index that
    /// need to be used. If not specified, a new one may be created
    pub fn push_value_ref<'v>(
        &mut self,
        value: &'v ValueRef,
        target: Option<b::ValueIdx>,
    ) {
        match &value.body {
            ValueRefBody::Global(mod_idx, global_idx) => {
                self.add_instr_with_result_and_push(
                    b::Instr::new(
                        b::InstrBody::GetGlobal(*mod_idx, *global_idx),
                        value.loc,
                    ),
                    target,
                );
            }
            ValueRefBody::Func(_, _) => {
                self.ctx.push_error(errors::Error::new(
                    errors::Todo::new("function as value".to_string()).into(),
                    value.loc,
                ));
                self.add_instr_with_result_and_push(
                    b::Instr::new(b::InstrBody::CompileError, value.loc),
                    target,
                );
            }
            ValueRefBody::Value(idx, v) => {
                assert!(*idx <= self.stack.len() - 1);
                let rel_value = self.stack.len() - idx - 1;
                if target.is_some() && rel_value == 0 {
                    return;
                }
                if target.is_some_and(|x| x != *v) {
                    self.module_parser.values[*v].redirects_to = target.clone();
                }
                self.instrs
                    .push(b::Instr::new(b::InstrBody::Dup(rel_value), value.loc));
                self.stack.push(target.unwrap_or(*v));
            }
            ValueRefBody::Bool(v) => {
                self.add_instr_with_result_and_push(
                    b::Instr::new(b::InstrBody::CreateBool(*v), value.loc),
                    target,
                );
            }
            ValueRefBody::Number(v) => {
                // TODO: use better type
                let ty_body = if v.contains('.') {
                    b::TypeBody::AnyFloat
                } else if v.starts_with('-') {
                    b::TypeBody::AnySignedNumber
                } else {
                    b::TypeBody::AnyNumber
                };
                self.add_instr_with_result_and_push(
                    b::Instr::new(
                        b::InstrBody::CreateNumber(
                            b::Type::new(ty_body, None),
                            v.clone(),
                        ),
                        value.loc,
                    ),
                    target,
                );
            }
            ValueRefBody::Never | ValueRefBody::CompileError => {
                self.add_instr_with_result_and_push(
                    b::Instr::new(b::InstrBody::CompileError, value.loc),
                    target,
                );
            }
        }
        if let Some(ty) = &value.ty {
            self.instrs.push(b::Instr::new(
                b::InstrBody::Type(ty.clone()),
                ty.loc.unwrap_or(value.loc),
            ));
        }
    }

    /// Add an instruction and create a value as its target. Return that value
    fn add_instr_with_result_and_push(
        &mut self,
        instr: b::Instr,
        target: Option<b::ValueIdx>,
    ) -> (usize, b::ValueIdx) {
        let v = target.unwrap_or_else(|| {
            self.module_parser
                .create_value(b::Type::unknown(None), instr.loc)
        });
        self.instrs.push(instr.with_results([v]));
        self.stack.push(v);
        (self.stack.len() - 1, v)
    }

    fn add_un_op(
        &mut self,
        op: ts::Node,
        operand: ValueRef,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        self.push_value_ref(&operand, None);
        let body = match op.kind() {
            "not" => b::InstrBody::Not,
            kind => panic!("Unhandled unary operator: {kind}"),
        };
        let loc = b::Loc::from_node(self.src_idx, &op).merge(&operand.loc);
        self.stack.pop();
        let (idx, v) =
            self.add_instr_with_result_and_push(b::Instr::new(body, loc), target);
        ValueRef::new(ValueRefBody::Value(idx, v), loc)
    }

    fn add_bin_op(
        &mut self,
        op: ts::Node,
        left: ValueRef,
        right: ValueRef,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        self.push_value_ref(&left, None);
        self.push_value_ref(&right, None);
        let body = match op.kind() {
            "plus" => b::InstrBody::Add,
            "minus" => b::InstrBody::Sub,
            "percent" => b::InstrBody::Mod,
            "star" => b::InstrBody::Mul,
            "slash" => b::InstrBody::Div,
            "double_star" => {
                self.ctx.push_error(errors::Error::new(
                    errors::Todo::new("exponentiation".to_string()).into(),
                    Loc::from_node(self.src_idx, &op),
                ));
                b::InstrBody::CompileError
            }
            "double_eq" => b::InstrBody::Eq,
            "not_eq" => b::InstrBody::Neq,
            "gt" => b::InstrBody::Gt,
            "lt" => b::InstrBody::Lt,
            "gt_eq" => b::InstrBody::Gte,
            "lt_eq" => b::InstrBody::Lte,
            kind => panic!("Unhandled binary operator: {kind}"),
        };
        let loc = left.loc.merge(&right.loc);
        self.stack.pop_many(2);
        let (idx, v) =
            self.add_instr_with_result_and_push(b::Instr::new(body, loc), target);
        ValueRef::new(ValueRefBody::Value(idx, v), loc)
    }

    fn add_get_prop(
        &mut self,
        parent: ValueRef,
        prop_name: &str,
        loc: b::Loc,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        self.push_value_ref(&parent, None);
        self.stack.pop();
        let (idx, v) = self.add_instr_with_result_and_push(
            b::Instr::new(b::InstrBody::GetProperty(prop_name.to_string()), loc),
            target,
        );
        ValueRef::new(ValueRefBody::Value(idx, v), loc)
    }

    fn add_call(
        &mut self,
        callee: ValueRef,
        args: impl IntoIterator<Item = ValueRef>,
        loc: b::Loc,
        target: Option<b::ValueIdx>,
        returning: bool,
    ) -> ValueRef {
        let args: Vec<_> = args.into_iter().collect();
        match callee.body {
            ValueRefBody::Func(mod_idx, func_idx) => {
                if returning
                    && self.mod_idx == mod_idx
                    && self.func_idx.is_some_and(|i| i == func_idx)
                {
                    self.is_loop = true;
                    self.stack.get_scope_mut().mark_as_never();

                    for arg in &args {
                        self.push_value_ref(arg, None);
                    }

                    self.instrs.push(b::Instr::new(b::InstrBody::Continue, loc));
                    ValueRef::new(ValueRefBody::Never, loc)
                } else {
                    for arg in &args {
                        self.push_value_ref(arg, None);
                    }

                    self.stack.pop_many(args.len());
                    let (idx, v) = self.add_instr_with_result_and_push(
                        b::Instr::new(b::InstrBody::Call(mod_idx, func_idx), loc),
                        target,
                    );
                    ValueRef::new(ValueRefBody::Value(idx, v), loc)
                }
            }
            ValueRefBody::Value(..) | ValueRefBody::Global(_, _) => {
                for arg in &args {
                    self.push_value_ref(arg, None);
                }
                self.push_value_ref(&callee, None);

                self.stack.pop_many(args.len());
                let (idx, v) = self.add_instr_with_result_and_push(
                    b::Instr::new(b::InstrBody::IndirectCall(args.len()), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(idx, v), loc)
            }
            ValueRefBody::CompileError => ValueRef::new(ValueRefBody::CompileError, loc),
            _ => {
                panic!("Value is not a function")
            }
        }
    }

    fn add_macro(
        &mut self,
        name: &str,
        args: &[ts::Node<'t>],
        loc: b::Loc,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        match name {
            "str_len" | "array_len" => {
                // TODO: better error handling
                assert!(args.len() == 1, "@{name}() expects a single argument");

                let source = self.add_expr_node(args[0], None, false);
                self.push_value_ref(&source, None);

                let instr_body = match name {
                    "str_len" => b::InstrBody::StrLen,
                    "array_len" => b::InstrBody::ArrayLen,
                    _ => unreachable!(),
                };

                self.stack.pop();
                let (idx, v) = self.add_instr_with_result_and_push(
                    b::Instr::new(instr_body, loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(idx, v), loc)
            }
            "str_ptr" | "array_ptr" => {
                // TODO: better error handling
                assert!(args.len() == 2, "@{name}() expects 2 arguments");

                let source = self.add_expr_node(args[0], None, false);

                let ValueRefBody::Number(idx) =
                    self.add_expr_node(args[1], None, false).body
                else {
                    // TODO: better error handling
                    panic!("index can only be a number");
                };
                // TODO: better error handling
                let idx: u64 = idx.parse().expect("index is not a valid number");

                self.push_value_ref(&source, None);

                let instr_body = match name {
                    "str_ptr" => b::InstrBody::StrPtr(idx),
                    "array_ptr" => b::InstrBody::ArrayPtr(idx),
                    _ => unreachable!(),
                };

                self.stack.pop();
                let (idx, v) = self.add_instr_with_result_and_push(
                    b::Instr::new(instr_body, loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(idx, v), loc)
            }
            _ => {
                panic!("unhandled macro: `{name}`")
            }
        }
    }

    fn add_stmt_node(&mut self, node: ts::Node<'t>) {
        match node.kind() {
            "let_stmt" => {
                let mut value =
                    self.add_expr_node(node.required_field("value"), None, false);
                let pat_node = node.required_field("pat");
                if let Some(ty_node) = node.field("type") {
                    value.ty = Some(self.module_parser.types.parse_type_expr(ty_node));
                }
                match pat_node.kind() {
                    "ident" => {
                        let ident = pat_node
                            .get_text(&self.ctx.source(self.src_idx).content().text);
                        self.idents.insert(
                            ident.to_string(),
                            value.with_loc(Loc::from_node(self.src_idx, &node)),
                        );
                    }
                    kind => panic!("Found unexpected pattern `{kind}`"),
                }
            }
            kind => panic!("Found unexpected statement `{kind}`"),
        }
    }
}

#[derive(Debug, Clone, new)]
struct ScopePayload {
    idents: HashMap<String, ValueRef>,
}
