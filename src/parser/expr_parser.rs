use std::collections::HashMap;
use std::usize;

use derive_new::new;
use itertools::Itertools;
use tree_sitter as ts;

use super::module_parser::ModuleParser;
use super::parser_value::{ValueRef, ValueRefBody};
use crate::bytecode::Loc;
use crate::utils::{ScopeStack, TreeSitterUtils};
use crate::{bytecode as b, context, errors, utils};

pub struct ExprParser<'a, 't> {
    pub module_parser: ModuleParser<'a, 't>,
    pub instrs: Vec<b::Instr>,
    pub is_loop: bool,
    pub scopes: ScopeStack<ScopePayload>,
    ctx: &'a context::BuildContext,
    src_idx: usize,
    mod_idx: usize,
    func_idx: Option<usize>,
}

impl<'a, 't> ExprParser<'a, 't> {
    #[tracing::instrument(skip(ctx, module_parser, inputs))]
    pub fn new(
        ctx: &'a context::BuildContext,
        module_parser: ModuleParser<'a, 't>,
        src_idx: usize,
        mod_idx: usize,
        func_idx: Option<usize>,
        inputs: impl IntoIterator<Item = (String, b::ValueIdx, b::Loc)>,
    ) -> Self {
        let mut idents = module_parser.idents.clone();

        for (ident, v, loc) in inputs.into_iter() {
            tracing::trace!(v, "insert input '{ident}'");
            idents.insert(ident, ValueRef::new(ValueRefBody::Value(v), loc));
        }

        ExprParser {
            ctx,
            mod_idx,
            src_idx,
            module_parser,
            scopes: ScopeStack::new(ScopePayload::new(idents)),
            func_idx,
            instrs: vec![],
            is_loop: false,
        }
    }

    pub fn finish(
        mut self,
        result: b::ValueIdx,
    ) -> (ModuleParser<'a, 't>, Vec<b::Instr>) {
        if self.is_loop {
            let func = &self.module_parser.funcs[self.func_idx.unwrap()].func;
            self.instrs.insert(
                0,
                b::Instr::new(
                    b::InstrBody::Loop(func.params.clone(), result),
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
                let v = self.add_instr_with_result(
                    b::Instr::new(b::InstrBody::CreateString(string), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(v), loc)
            }
            "array_lit" => {
                let items: Vec<_> = node
                    .iter_field("items")
                    .map(|item_node| {
                        let v = self.add_expr_node(item_node, None, false);
                        self.use_value_ref(&v, None)
                    })
                    .collect();
                let v = self.add_instr_with_result(
                    b::Instr::new(b::InstrBody::CreateArray(items), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(v), loc)
            }
            "record_lit" => {
                let fields = node
                    .iter_field("fields")
                    .map(|field_node| {
                        let name = field_node
                            .required_field("name")
                            .get_text(&self.ctx.source(self.src_idx).content().text);
                        let value_ref = self.add_expr_node(
                            field_node.required_field("value"),
                            None,
                            false,
                        );
                        let v = self.use_value_ref(&value_ref, None);
                        (name.to_string(), v)
                    })
                    .collect();
                let v = self.add_instr_with_result(
                    b::Instr::new(b::InstrBody::CreateRecord(fields), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(v), loc)
            }
            "ident" => {
                let ident = node.get_text(&self.ctx.source(self.src_idx).content().text);
                if let Some(value) = self.scopes.last().idents.get(ident) {
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
                self.scopes.begin_cloned();

                for stmt_node in node.iter_field("body") {
                    self.add_stmt_node(stmt_node);
                }
                let value =
                    self.add_expr_node(node.required_field("value"), target, returning);

                self.scopes.end();

                value
            }
            "if" => {
                let cond_value =
                    self.add_expr_node(node.required_field("cond"), None, false);
                let cond_v = self.use_value_ref(&cond_value, None);

                let block_len = self.scopes.len();

                let loc = b::Loc::from_node(self.src_idx, &node);
                let v = target.unwrap_or_else(|| {
                    self.module_parser.create_value(b::Type::unknown(None), loc)
                });
                self.scopes.begin_cloned();

                self.instrs
                    .push(b::Instr::new(b::InstrBody::If(cond_v, v), loc));
                let then_value =
                    self.add_expr_node(node.required_field("then"), Some(v), returning);

                if !then_value.is_never() {
                    self.use_value_ref(&then_value, Some(v));
                }

                let else_value = if let Some(else_node) = node.field("else") {
                    self.instrs.push(b::Instr::new(
                        b::InstrBody::Else,
                        Loc::from_node(self.src_idx, &else_node),
                    ));

                    let old_idents = self
                        .scopes
                        .get(self.scopes.len() - 2)
                        .unwrap()
                        .idents
                        .clone();
                    let scope = self.scopes.branch();
                    scope.idents = old_idents;

                    let else_value = self.add_expr_node(else_node, Some(v), returning);

                    if !else_value.is_never() {
                        self.use_value_ref(&else_value, Some(v));
                    }

                    else_value
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::Todo::new("if without else".to_string()).into(),
                        loc,
                    ));
                    ValueRef::new(ValueRefBody::CompileError, loc)
                };

                self.scopes.end();

                if !then_value.is_never() || !else_value.is_never() {
                    self.instrs.push(b::Instr::new(
                        b::InstrBody::End,
                        Loc::from_node(self.src_idx, &node),
                    ));
                    ValueRef::new(ValueRefBody::Value(v), loc)
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

    /// Uses value reference to the stack. This will add the necessary instruction to
    /// have the value accessible. `target` specifies the value index that need to be
    /// used. If not specified, a new one may be created. Returns the used value.
    pub fn use_value_ref<'v>(
        &mut self,
        value_ref: &'v ValueRef,
        target: Option<b::ValueIdx>,
    ) -> b::ValueIdx {
        let v = match &value_ref.body {
            ValueRefBody::Global(mod_idx, global_idx) => self.add_instr_with_result(
                b::Instr::new(
                    b::InstrBody::GetGlobal(*mod_idx, *global_idx),
                    value_ref.loc,
                ),
                target,
            ),
            ValueRefBody::Func(_, _) => {
                self.ctx.push_error(errors::Error::new(
                    errors::Todo::new("function as value".to_string()).into(),
                    value_ref.loc,
                ));
                self.add_instr_with_result(
                    b::Instr::new(b::InstrBody::CompileError, value_ref.loc),
                    target,
                )
            }
            ValueRefBody::Value(v) => *v,
            ValueRefBody::Bool(v) => self.add_instr_with_result(
                b::Instr::new(b::InstrBody::CreateBool(*v), value_ref.loc),
                target,
            ),
            ValueRefBody::Number(v) => self.add_instr_with_result(
                b::Instr::new(b::InstrBody::CreateNumber(v.clone()), value_ref.loc),
                target,
            ),
            ValueRefBody::Never | ValueRefBody::CompileError => self
                .add_instr_with_result(
                    b::Instr::new(b::InstrBody::CompileError, value_ref.loc),
                    target,
                ),
        };
        if let Some(ty) = &value_ref.ty {
            self.instrs.push(b::Instr::new(
                b::InstrBody::Type(v, ty.clone()),
                ty.loc.unwrap_or(value_ref.loc),
            ));
        }
        v
    }

    /// Adds an instruction and create a value as its target. Returns that value.
    fn add_instr_with_result(
        &mut self,
        instr: b::Instr,
        target: Option<b::ValueIdx>,
    ) -> b::ValueIdx {
        let v = target.unwrap_or_else(|| {
            self.module_parser
                .create_value(b::Type::unknown(None), instr.loc)
        });
        self.instrs.push(instr.with_results([v]));
        v
    }

    fn add_un_op(
        &mut self,
        op: ts::Node,
        operand: ValueRef,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        let operand_v = self.use_value_ref(&operand, None);
        let body = match op.kind() {
            "not" => b::InstrBody::Not(operand_v),
            kind => panic!("Unhandled unary operator: {kind}"),
        };
        let loc = b::Loc::from_node(self.src_idx, &op).merge(&operand.loc);
        let v = self.add_instr_with_result(b::Instr::new(body, loc), target);
        ValueRef::new(ValueRefBody::Value(v), loc)
    }

    fn add_bin_op(
        &mut self,
        op: ts::Node,
        left: ValueRef,
        right: ValueRef,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        let left_v = self.use_value_ref(&left, None);
        let right_v = self.use_value_ref(&right, None);
        let body = match op.kind() {
            "plus" => b::InstrBody::Add(left_v, right_v),
            "minus" => b::InstrBody::Sub(left_v, right_v),
            "percent" => b::InstrBody::Mod(left_v, right_v),
            "star" => b::InstrBody::Mul(left_v, right_v),
            "slash" => b::InstrBody::Div(left_v, right_v),
            "double_star" => {
                self.ctx.push_error(errors::Error::new(
                    errors::Todo::new("exponentiation".to_string()).into(),
                    Loc::from_node(self.src_idx, &op),
                ));
                b::InstrBody::CompileError
            }
            "double_eq" => b::InstrBody::Eq(left_v, right_v),
            "not_eq" => b::InstrBody::Neq(left_v, right_v),
            "gt" => b::InstrBody::Gt(left_v, right_v),
            "lt" => b::InstrBody::Lt(left_v, right_v),
            "gt_eq" => b::InstrBody::Gte(left_v, right_v),
            "lt_eq" => b::InstrBody::Lte(left_v, right_v),
            kind => panic!("Unhandled binary operator: {kind}"),
        };
        let loc = left.loc.merge(&right.loc);
        let v = self.add_instr_with_result(b::Instr::new(body, loc), target);
        ValueRef::new(ValueRefBody::Value(v), loc)
    }

    fn add_get_prop(
        &mut self,
        parent: ValueRef,
        prop_name: &str,
        loc: b::Loc,
        target: Option<b::ValueIdx>,
    ) -> ValueRef {
        let source_v = self.use_value_ref(&parent, None);
        let v = self.add_instr_with_result(
            b::Instr::new(
                b::InstrBody::GetProperty(source_v, prop_name.to_string()),
                loc,
            ),
            target,
        );
        ValueRef::new(ValueRefBody::Value(v), loc)
    }

    fn add_call(
        &mut self,
        callee: ValueRef,
        args: impl IntoIterator<Item = ValueRef>,
        loc: b::Loc,
        target: Option<b::ValueIdx>,
        returning: bool,
    ) -> ValueRef {
        let args_vs: Vec<_> = args
            .into_iter()
            .map(|arg| self.use_value_ref(&arg, None))
            .collect();
        match callee.body {
            ValueRefBody::Func(mod_idx, func_idx) => {
                if returning
                    && self.mod_idx == mod_idx
                    && self.func_idx.is_some_and(|i| i == func_idx)
                {
                    self.is_loop = true;
                    self.scopes.last_mut().mark_as_never();

                    self.instrs
                        .push(b::Instr::new(b::InstrBody::Continue(args_vs), loc));
                    ValueRef::new(ValueRefBody::Never, loc)
                } else {
                    let v = self.add_instr_with_result(
                        b::Instr::new(
                            b::InstrBody::Call(mod_idx, func_idx, args_vs),
                            loc,
                        ),
                        target,
                    );
                    ValueRef::new(ValueRefBody::Value(v), loc)
                }
            }
            ValueRefBody::Value(..) | ValueRefBody::Global(_, _) => {
                let callee_v = self.use_value_ref(&callee, None);

                let v = self.add_instr_with_result(
                    b::Instr::new(b::InstrBody::IndirectCall(callee_v, args_vs), loc),
                    target,
                );
                ValueRef::new(ValueRefBody::Value(v), loc)
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
                let source_v = self.use_value_ref(&source, None);

                let instr_body = match name {
                    "str_len" => b::InstrBody::StrLen(source_v),
                    "array_len" => b::InstrBody::ArrayLen(source_v),
                    _ => unreachable!(),
                };

                let v =
                    self.add_instr_with_result(b::Instr::new(instr_body, loc), target);
                ValueRef::new(ValueRefBody::Value(v), loc)
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

                let source_v = self.use_value_ref(&source, None);

                let instr_body = match name {
                    "str_ptr" => b::InstrBody::StrPtr(source_v, idx),
                    "array_ptr" => b::InstrBody::ArrayPtr(source_v, idx),
                    _ => unreachable!(),
                };

                let v =
                    self.add_instr_with_result(b::Instr::new(instr_body, loc), target);
                ValueRef::new(ValueRefBody::Value(v), loc)
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
                        self.scopes.last_mut().idents.insert(
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
