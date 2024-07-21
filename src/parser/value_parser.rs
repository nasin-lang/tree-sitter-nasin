use std::collections::HashMap;

use tree_sitter as ts;

use super::module_parser::ModuleParser;
use super::parser_value::ParserValue;
use crate::bytecode::GlobalIdx;
use crate::utils::{TreeSitterUtils, ValueStack};
use crate::{bytecode as b, utils};

type Stack<'a> = ValueStack<(), Block<'a>>;

pub struct ValueParser<'a> {
    pub module_parser: ModuleParser<'a>,
    pub instrs: Vec<b::Instr>,
    pub is_loop: bool,
    pub idents: HashMap<&'a str, ParserValue>,
    src: &'a str,
    func_idx: Option<usize>,
    stack: Stack<'a>,
}

impl<'a> ValueParser<'a> {
    pub fn new(
        src: &'a str,
        module_parser: ModuleParser<'a>,
        func_idx: Option<usize>,
        input_idents: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let mut idents = module_parser.idents.clone();

        let mut stack = Stack::new();
        for (i, ident) in input_idents.into_iter().enumerate() {
            idents.insert(ident, ParserValue::Local(i));
            stack.push(());
        }

        ValueParser {
            src,
            module_parser,
            idents,
            func_idx,
            instrs: vec![],
            is_loop: false,
            stack,
        }
    }

    pub fn finish(mut self) -> (ModuleParser<'a>, Vec<b::Instr>) {
        assert!(self.stack.block_len() == 0);

        if self.is_loop {
            self.instrs.insert(0, b::Instr::Loop);
            let params = &self.module_parser.funcs[self.func_idx.unwrap()].params;
            self.trim_stack(params.len(), 1);
            self.instrs.push(b::Instr::End);
        }
        self.trim_stack(0, 1);
        (self.module_parser, self.instrs)
    }

    pub fn add_value_node(&mut self, node: ts::Node<'a>, returning: bool) -> ParserValue {
        match node.kind() {
            "true" => ParserValue::Bool(true),
            "false" => ParserValue::Bool(false),
            "number" => {
                let number = node.get_text(self.src);
                ParserValue::Number(number.to_string())
            }
            "string_lit" => {
                let string = utils::decode_string_lit(
                    node.required_field("content").get_text(self.src),
                );
                ParserValue::String(string)
            }
            "array_lit" => {
                let items = node
                    .iter_field("items")
                    .map(|item_node| self.add_value_node(item_node, false))
                    .collect();
                ParserValue::Array(items)
            }
            "record_lit" => {
                let fields = node
                    .iter_field("fields")
                    .map(|field_node| {
                        let field_name =
                            field_node.required_field("name").get_text(self.src);
                        let field_value = self
                            .add_value_node(field_node.required_field("value"), false);
                        (field_name.to_string(), field_value)
                    })
                    .collect();
                ParserValue::Record(fields)
            }
            "ident" => {
                let ident = node.get_text(self.src);
                let Some(value) = self.idents.get(ident) else {
                    // TODO: better error handling
                    panic!("Value \"{ident}\" not found");
                };
                value.clone()
            }
            "bin_op" => {
                let op = node.required_field("op").get_text(self.src);
                let left = self.add_value_node(node.required_field("left"), false);
                let right = self.add_value_node(node.required_field("right"), false);
                self.add_bin_op(op, left, right)
            }
            "get_prop" => {
                let parent = self.add_value_node(node.required_field("parent"), false);
                let prop_name = node.required_field("prop_name").get_text(self.src);
                self.add_get_prop(parent, prop_name)
            }
            "call" => {
                let callee = self.add_value_node(node.required_field("callee"), false);
                let args: Vec<_> = node
                    .iter_field("args")
                    .map(|arg_node| self.add_value_node(arg_node, false))
                    .collect();
                self.add_call(callee, args, returning)
            }
            "block" => {
                let old_idents = self.idents.clone();

                for stmt_node in node.iter_field("body") {
                    self.add_stmt_node(stmt_node);
                }
                let value = self.add_value_node(node.required_field("value"), returning);

                self.idents = old_idents;

                value
            }
            "if" => {
                let cond_value = self.add_value_node(node.required_field("cond"), false);
                self.push_values([&cond_value]);
                self.stack.pop(); // consume condition

                let stack_len = self.stack.len();
                let block_len = self.stack.block_len();

                self.stack.push_block(Block {
                    stack: self.stack.clone(),
                    idents: self.idents.clone(),
                });

                self.instrs.push(b::Instr::If);
                let then_value =
                    self.add_value_node(node.required_field("then"), returning);

                if !then_value.is_never() {
                    self.push_values([&then_value]);
                    assert!(self.stack.len() >= stack_len + 1);
                    self.trim_stack(stack_len, 1);
                }

                let else_value = if let Some(else_node) = node.field("else") {
                    assert!(self.stack.block_len() >= block_len + 1);
                    let block = self.stack.pop_block();
                    self.stack = block.stack.clone();
                    self.idents = block.idents.clone();

                    self.stack.push_block(block);

                    self.instrs.push(b::Instr::Else);
                    let else_value = self.add_value_node(else_node, returning);

                    if !else_value.is_never() {
                        self.push_values([&else_value]);
                        assert!(self.stack.len() >= stack_len + 1);
                        self.trim_stack(stack_len, 1);
                    }

                    else_value
                } else {
                    todo!("if without else");
                };

                assert!(self.stack.block_len() >= block_len + 1);
                let block = self.stack.pop_block();
                self.stack = block.stack;
                self.idents = block.idents;

                if !then_value.is_never() || !else_value.is_never() {
                    let idx = self.add_instr_with_result(0, b::Instr::End);
                    ParserValue::Local(idx)
                } else {
                    self.stack.unreachable = true;
                    ParserValue::Never
                }
            }
            k => panic!("Found unexpected expression `{}`", k),
        }
    }

    pub fn push_values<'v>(&mut self, values: impl IntoIterator<Item = &'v ParserValue>) {
        let mut values: Vec<_> = values.into_iter().cloned().collect();
        // Values that need to be constructed need to be prepared first so they don't mess
        // with the stack
        for value in &mut values {
            let local_idx = match value {
                ParserValue::Global(_)
                | ParserValue::Local(_)
                | ParserValue::Number(_)
                | ParserValue::Bool(_)
                | ParserValue::Never => {
                    continue;
                }
                ParserValue::Func(_) => todo!("func as value"),
                ParserValue::String(v) => {
                    self.add_instr_with_result(0, b::Instr::CreateString(v.clone()))
                }
                ParserValue::Array(vs) => {
                    self.push_values(vs as &_);
                    self.add_instr_with_result(
                        vs.len(),
                        b::Instr::CreateArray(vs.len() as u32),
                    )
                }
                ParserValue::Record(fields) => {
                    self.push_values(fields.values());
                    let fields: Vec<_> = fields.keys().cloned().collect();
                    self.add_instr_with_result(
                        fields.len(),
                        b::Instr::CreateRecord(fields),
                    )
                }
            };
            *value = ParserValue::Local(local_idx);
        }
        for value in values {
            match value {
                ParserValue::Global(idx) => {
                    self.add_instr_with_result(0, b::Instr::GetGlobal(idx as GlobalIdx));
                }
                ParserValue::Local(idx) => {
                    assert!(idx <= self.stack.len() - 1);
                    let rel_value = (self.stack.len() - idx - 1) as b::RelativeValue;
                    self.add_instr_with_result(0, b::Instr::Dup(rel_value));
                }
                ParserValue::Bool(v) => {
                    self.add_instr_with_result(0, b::Instr::CreateBool(v));
                }
                ParserValue::Number(v) => {
                    self.add_instr_with_result(0, b::Instr::CreateNumber(v));
                }
                ParserValue::Never => {
                    self.add_instr_with_result(0, b::Instr::CompileError);
                }
                ParserValue::Global(_)
                | ParserValue::Func(_)
                | ParserValue::String(_)
                | ParserValue::Array(_)
                | ParserValue::Record(_) => {
                    unreachable!();
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

    fn add_bin_op(
        &mut self,
        op: &str,
        left: ParserValue,
        right: ParserValue,
    ) -> ParserValue {
        self.push_values([&left, &right]);
        let idx = self.add_instr_with_result(
            2,
            match op {
                "+" => b::Instr::Add,
                "-" => b::Instr::Sub,
                "%" => b::Instr::Mod,
                "*" => b::Instr::Mul,
                "/" => b::Instr::Div,
                "**" => b::Instr::Pow,
                "==" => b::Instr::Eq,
                "!=" => b::Instr::Neq,
                ">" => b::Instr::Gt,
                "<" => b::Instr::Lt,
                ">=" => b::Instr::Gte,
                "<=" => b::Instr::Lte,
                op => panic!("Unhandled binary operator: {op}"),
            },
        );
        ParserValue::Local(idx)
    }

    fn add_get_prop(&mut self, parent: ParserValue, prop_name: &str) -> ParserValue {
        match parent {
            ParserValue::Array(_) => todo!("Array subscripting"),
            ParserValue::Record(fields) => {
                let Some((_, field_value)) = fields
                    .into_iter()
                    .find(|(field_name, _)| field_name == prop_name)
                else {
                    // TODO: better error handling
                    panic!("Field not found: {prop_name}");
                };
                field_value
            }
            _ => {
                self.push_values([&parent]);
                let idx = self
                    .add_instr_with_result(1, b::Instr::GetField(prop_name.to_string()));
                ParserValue::Local(idx)
            }
        }
    }

    fn add_call(
        &mut self,
        callee: ParserValue,
        args: impl IntoIterator<Item = ParserValue>,
        returning: bool,
    ) -> ParserValue {
        let mut args: Vec<_> = args.into_iter().collect();
        match callee {
            ParserValue::Func(idx) => {
                self.push_values(&args);

                if self.func_idx.is_some_and(|i| i == idx) && returning {
                    self.is_loop = true;
                    self.stack.unreachable = true;

                    self.trim_stack(0, self.module_parser.funcs[idx].params.len());

                    self.instrs.push(b::Instr::Continue);
                    ParserValue::Never
                } else {
                    let idx = self.add_instr_with_result(
                        args.len(),
                        b::Instr::Call(idx as b::FuncIdx),
                    );
                    ParserValue::Local(idx)
                }
            }
            ParserValue::Local(_) | ParserValue::Global(_) | ParserValue::Record(_) => {
                todo!("inderect call")
            }
            _ => {
                // TODO: better error handling
                panic!("Value is not a function")
            }
        }
    }

    fn add_stmt_node(&mut self, node: ts::Node<'a>) {
        match node.kind() {
            "var_decl" => {
                let value = self.add_value_node(node.required_field("value"), false);
                let pat_node = node.required_field("pat");
                match pat_node.kind() {
                    "ident" => {
                        let ident = pat_node.get_text(self.src);
                        self.idents.insert(ident, value);
                    }
                    kind => panic!("Found unexpected pattern `{kind}`"),
                }
            }
            kind => panic!("Found unexpected statement `{kind}`"),
        }
    }

    fn trim_stack(&mut self, bottom_len: usize, top_len: usize) {
        if let Some(b::Instr::Dup(0)) = self.instrs.last() {
            self.stack.pop();
            self.instrs.pop();
        }

        let len = bottom_len + top_len;

        if self.stack.len() < len {
            return;
        }

        let delete_count = self.stack.len() - len;
        for _ in 0..delete_count {
            self.instrs
                .push(b::Instr::Drop(top_len as b::RelativeValue));
            self.stack.pop();
        }
    }
}

#[derive(Debug, Clone)]
struct Block<'a> {
    stack: Stack<'a>,
    idents: HashMap<&'a str, ParserValue>,
}
