use std::collections::HashMap;
use std::usize;

use derive_new::new;
use itertools::Itertools;
use tree_sitter as ts;

use super::module_parser::ModuleParser;
use super::parser_value::{Value, ValueBody};
use crate::bytecode::Loc;
use crate::utils::{TreeSitterUtils, ValueStack};
use crate::{bytecode as b, context, errors, utils};

type Stack = ValueStack<(), ScopePayload>;

pub struct ExprParser<'a, 't> {
    pub module_parser: ModuleParser<'a, 't>,
    pub instrs: Vec<b::Instr>,
    pub is_loop: bool,
    pub idents: HashMap<String, Value>,
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
        inputs: impl IntoIterator<Item = (String, b::Loc)>,
    ) -> Self {
        let mut idents = module_parser.idents.clone();

        let mut stack = Stack::new(ScopePayload::new(idents.clone()));
        for (i, (ident, loc)) in inputs.into_iter().enumerate() {
            idents.insert(ident, Value::new(ValueBody::Local(i), loc));
            stack.push(());
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
                    b::InstrBody::Loop(b::Type::unknown(None), func.params.len()),
                    self.instrs[0].loc,
                ),
            );
            self.instrs
                .push(b::Instr::new(b::InstrBody::End, self.instrs[0].loc));
        }
        (self.module_parser, self.instrs)
    }

    pub fn add_expr_node(&mut self, node: ts::Node<'t>, returning: bool) -> Value {
        let loc = Loc::from_node(self.src_idx, &node);
        match node.kind() {
            "true" => Value::new(ValueBody::Bool(true), loc),
            "false" => Value::new(ValueBody::Bool(false), loc),
            "number" => {
                let number = node.get_text(&self.ctx.source(self.src_idx).content().text);
                Value::new(ValueBody::Number(number.to_string()), loc)
            }
            "string_lit" => {
                let string = utils::decode_string_lit(
                    node.required_field("content")
                        .get_text(&self.ctx.source(self.src_idx).content().text),
                );
                let local_idx = self.add_instr_with_result(
                    0,
                    b::Instr::new(b::InstrBody::CreateString(string), loc),
                );
                Value::new(ValueBody::Local(local_idx), loc)
            }
            "array_lit" => {
                let items: Vec<_> = node
                    .iter_field("items")
                    .map(|item_node| self.add_expr_node(item_node, false))
                    .collect();
                self.push_values(&items, false);
                let ty = b::Type::new(
                    b::TypeBody::Array(b::ArrayType::new(
                        b::Type::unknown(None).into(),
                        Some(items.len()),
                    )),
                    None,
                );
                let local_idx = self.add_instr_with_result(
                    items.len(),
                    b::Instr::new(b::InstrBody::CreateArray(ty, items.len()), loc),
                );
                Value::new(ValueBody::Local(local_idx), loc)
            }
            "record_lit" => {
                let fields: utils::SortedMap<_, _> = node
                    .iter_field("fields")
                    .map(|field_node| {
                        let field_name = field_node
                            .required_field("name")
                            .get_text(&self.ctx.source(self.src_idx).content().text);
                        let field_value =
                            self.add_expr_node(field_node.required_field("value"), false);
                        (field_name.to_string(), field_value)
                    })
                    .collect();
                self.push_values(fields.values(), false);
                let ty = b::Type::new(
                    b::TypeBody::Inferred(b::InferredType {
                        properties: fields
                            .keys()
                            .map(|k| (k.clone(), b::Type::unknown(None)))
                            .collect(),
                    }),
                    None,
                );
                let local_idx = self.add_instr_with_result(
                    fields.len(),
                    b::Instr::new(
                        b::InstrBody::CreateRecord(ty, fields.keys().cloned().collect()),
                        loc,
                    ),
                );
                Value::new(ValueBody::Local(local_idx), loc)
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
                    Value::new(ValueBody::CompileError, loc)
                }
            }
            "bin_op" => {
                let op = node.required_field("op");
                let left = self.add_expr_node(node.required_field("left"), false);
                let right = self.add_expr_node(node.required_field("right"), false);
                self.add_bin_op(op, left, right)
            }
            "get_prop" => {
                let parent = self.add_expr_node(node.required_field("parent"), false);
                let prop_name_node = node.required_field("prop_name");
                let prop_name = prop_name_node
                    .get_text(&self.ctx.source(self.src_idx).content().text);
                self.add_get_prop(
                    parent,
                    prop_name,
                    Loc::from_node(self.src_idx, &prop_name_node),
                )
            }
            "call" => {
                let callee = self.add_expr_node(node.required_field("callee"), false);
                let args: Vec<_> = node
                    .iter_field("args")
                    .map(|arg_node| self.add_expr_node(arg_node, false))
                    .collect();
                self.add_call(callee, args, loc, returning)
            }
            "block" => {
                let old_idents = self.idents.clone();

                for stmt_node in node.iter_field("body") {
                    self.add_stmt_node(stmt_node);
                }
                let value = self.add_expr_node(node.required_field("value"), returning);

                self.idents = old_idents;

                value
            }
            "if" => {
                let cond_value = self.add_expr_node(node.required_field("cond"), false);
                self.push_values([&cond_value], false);
                self.stack.pop(); // consume condition

                let stack_len = self.stack.len();
                let block_len = self.stack.scope_len();

                self.stack
                    .create_scope(ScopePayload::new(self.idents.clone()));

                self.instrs.push(b::Instr::new(
                    b::InstrBody::If(b::Type::unknown(None)),
                    b::Loc::from_node(self.src_idx, &node),
                ));
                let then_value =
                    self.add_expr_node(node.required_field("then"), returning);

                if !then_value.is_never() {
                    self.push_values([&then_value], true);
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

                    let else_value = self.add_expr_node(else_node, returning);

                    if !else_value.is_never() {
                        self.push_values([&else_value], true);
                        assert!(self.stack.len() >= stack_len + 1);
                    }

                    else_value
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::Todo::new("if without else".to_string()).into(),
                        loc,
                    ));
                    Value::new(ValueBody::CompileError, loc)
                };

                assert!(self.stack.scope_len() >= block_len + 1);
                let (scope, _) = self.stack.end_scope();
                self.idents = scope.payload.idents;

                if !then_value.is_never() || !else_value.is_never() {
                    let idx = self.add_instr_with_result(
                        0,
                        b::Instr::new(
                            b::InstrBody::End,
                            Loc::from_node(self.src_idx, &node),
                        ),
                    );
                    Value::new(ValueBody::Local(idx), loc)
                } else {
                    Value::new(ValueBody::Never, loc)
                }
            }
            "macro" => {
                let name = node
                    .required_field("name")
                    .of_kind("ident")
                    .get_text(&self.ctx.source(self.src_idx).content().text);
                let args = node.iter_field("args").collect_vec();
                self.add_macro(name, &args, b::Loc::from_node(self.src_idx, &node))
            }
            k => panic!("Found unexpected expression `{}`", k),
        }
    }

    /// Push values to the stack. This will add the necessary instruction to have the
    /// values accessible in the stack in the specified order. `is_result` specifies that
    /// the values will be used as the result of the current scope. In that case, it will
    /// avoid adding unnecessary `dup` instructions
    pub fn push_values<'v>(
        &mut self,
        values: impl IntoIterator<Item = &'v Value>,
        is_result: bool,
    ) {
        for value in values {
            match &value.body {
                ValueBody::Global(mod_idx, global_idx) => {
                    self.add_instr_with_result(
                        0,
                        b::Instr::new(
                            b::InstrBody::GetGlobal(*mod_idx, *global_idx),
                            value.loc,
                        ),
                    );
                }
                ValueBody::Func(_, _) => {
                    self.ctx.push_error(errors::Error::new(
                        errors::Todo::new("function as value".to_string()).into(),
                        value.loc,
                    ));
                    self.add_instr_with_result(
                        0,
                        b::Instr::new(b::InstrBody::CompileError, value.loc),
                    );
                }
                ValueBody::Local(idx) => {
                    assert!(*idx <= self.stack.len() - 1);
                    let rel_value = self.stack.len() - idx - 1;
                    if is_result && rel_value == 0 {
                        continue;
                    }
                    self.add_instr_with_result(
                        0,
                        b::Instr::new(b::InstrBody::Dup(rel_value), value.loc),
                    );
                }
                ValueBody::Bool(v) => {
                    self.add_instr_with_result(
                        0,
                        b::Instr::new(b::InstrBody::CreateBool(*v), value.loc),
                    );
                }
                ValueBody::Number(v) => {
                    // TODO: use better type
                    let ty_body = if v.contains('.') {
                        b::TypeBody::AnyFloat
                    } else if v.starts_with('-') {
                        b::TypeBody::AnySignedNumber
                    } else {
                        b::TypeBody::AnyNumber
                    };
                    self.add_instr_with_result(
                        0,
                        b::Instr::new(
                            b::InstrBody::CreateNumber(
                                b::Type::new(ty_body, None),
                                v.clone(),
                            ),
                            value.loc,
                        ),
                    );
                }
                ValueBody::Never | ValueBody::CompileError => {
                    self.add_instr_with_result(
                        0,
                        b::Instr::new(b::InstrBody::CompileError, value.loc),
                    );
                }
            }
        }
    }

    fn add_instr_with_result(&mut self, input_count: usize, instr: b::Instr) -> usize {
        self.instrs.push(instr);
        for _ in 0..input_count {
            self.stack.pop();
        }
        self.stack.push(());
        self.stack.len() - 1
    }

    fn add_bin_op(&mut self, op: ts::Node, left: Value, right: Value) -> Value {
        self.push_values([&left, &right], false);
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
        let idx = self.add_instr_with_result(2, b::Instr::new(body, loc));
        Value::new(ValueBody::Local(idx), loc)
    }

    fn add_get_prop(&mut self, parent: Value, prop_name: &str, loc: b::Loc) -> Value {
        self.push_values([&parent], false);
        let idx = self.add_instr_with_result(
            1,
            b::Instr::new(b::InstrBody::GetField(prop_name.to_string()), loc),
        );
        Value::new(ValueBody::Local(idx), loc)
    }

    fn add_call(
        &mut self,
        callee: Value,
        args: impl IntoIterator<Item = Value>,
        loc: b::Loc,
        returning: bool,
    ) -> Value {
        let args: Vec<_> = args.into_iter().collect();
        match callee.body {
            ValueBody::Func(mod_idx, func_idx) => {
                if returning
                    && self.mod_idx == mod_idx
                    && self.func_idx.is_some_and(|i| i == func_idx)
                {
                    self.is_loop = true;
                    self.stack.get_scope_mut().mark_as_never();

                    self.push_values(&args, true);

                    self.instrs.push(b::Instr::new(b::InstrBody::Continue, loc));
                    Value::new(ValueBody::Never, loc)
                } else {
                    self.push_values(&args, false);

                    let idx = self.add_instr_with_result(
                        args.len(),
                        b::Instr::new(b::InstrBody::Call(mod_idx, func_idx), loc),
                    );
                    Value::new(ValueBody::Local(idx), loc)
                }
            }
            ValueBody::Local(_) | ValueBody::Global(_, _) => {
                self.ctx.push_error(errors::Error::new(
                    errors::Todo::new("inderect call".to_string()).into(),
                    loc,
                ));
                callee.with_loc(loc)
            }
            ValueBody::CompileError => callee.with_loc(loc),
            _ => {
                // TODO: better error handling
                panic!("Value is not a function")
            }
        }
    }

    fn add_macro(&mut self, name: &str, args: &[ts::Node<'t>], loc: b::Loc) -> Value {
        match name {
            "str_len" | "array_len" => {
                // TODO: better error handling
                assert!(args.len() == 1, "@{name}() expects a single argument");

                let source = self.add_expr_node(args[0], false);
                self.push_values(&[source], false);

                let instr_body = match name {
                    "str_len" => b::InstrBody::StrLen,
                    "array_len" => b::InstrBody::ArrayLen,
                    _ => unreachable!(),
                };

                let idx = self.add_instr_with_result(1, b::Instr::new(instr_body, loc));
                Value::new(ValueBody::Local(idx), loc)
            }
            "str_ptr" | "array_ptr" => {
                // TODO: better error handling
                assert!(args.len() == 2, "@{name}() expects 2 arguments");

                let source = self.add_expr_node(args[0], false);

                let ValueBody::Number(idx) = self.add_expr_node(args[1], false).body
                else {
                    // TODO: better error handling
                    panic!("index can only be a number");
                };
                // TODO: better error handling
                let idx: u64 = idx.parse().expect("index is not a valid number");

                self.push_values(&[source], false);

                let instr_body = match name {
                    "str_ptr" => b::InstrBody::StrPtr(idx),
                    "array_ptr" => b::InstrBody::ArrayPtr(idx),
                    _ => unreachable!(),
                };

                let idx = self.add_instr_with_result(1, b::Instr::new(instr_body, loc));
                Value::new(ValueBody::Local(idx), loc)
            }
            _ => {
                panic!("unhandled macro: `{name}`")
            }
        }
    }

    fn add_stmt_node(&mut self, node: ts::Node<'t>) {
        match node.kind() {
            "var_decl" => {
                let value = self.add_expr_node(node.required_field("value"), false);
                let pat_node = node.required_field("pat");
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
    idents: HashMap<String, Value>,
}
