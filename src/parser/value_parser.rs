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
    loaded_globals: HashMap<usize, usize>,
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
            loaded_globals: HashMap::new(),
            stack,
        }
    }

    pub fn finish(mut self, result_len: usize) -> (ModuleParser<'a>, Vec<b::Instr>) {
        self.trim_stack(0, result_len);
        (self.module_parser, self.instrs)
    }

    pub fn add_value_node(&mut self, node: ts::Node<'a>) -> ParserValue {
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
                    .map(|item_node| self.add_value_node(item_node))
                    .collect();
                ParserValue::Array(items)
            }
            "record_lit" => {
                let fields = node
                    .iter_field("fields")
                    .map(|field_node| {
                        let field_name =
                            field_node.required_field("name").get_text(self.src);
                        let field_value =
                            self.add_value_node(field_node.required_field("value"));
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
                let left = self.add_value_node(node.required_field("left"));
                let right = self.add_value_node(node.required_field("right"));
                self.add_bin_op(op, left, right)
            }
            "get_prop" => {
                let parent = self.add_value_node(node.required_field("parent"));
                let prop_name = node.required_field("prop_name").get_text(self.src);
                self.add_get_prop(parent, prop_name)
            }
            "call" => {
                let callee = self.add_value_node(node.required_field("callee"));
                let args: Vec<_> = node
                    .iter_field("args")
                    .map(|arg_node| self.add_value_node(arg_node))
                    .collect();
                self.add_call(callee, args)
            }
            "block" => {
                let old_idents = self.idents.clone();

                for stmt_node in node.iter_field("body") {
                    self.add_stmt_node(stmt_node);
                }
                let value = self.add_value_node(node.required_field("value"));

                self.idents = old_idents;

                value
            }
            "if" => {
                let cond_value = self.add_value_node(node.required_field("cond"));
                self.push_value(&cond_value, false);
                self.stack.pop(); // consume condition

                let stack_len = self.stack.len();
                let block_len = self.stack.block_len();

                self.stack.push_block(Block {
                    stack: self.stack.clone(),
                    idents: self.idents.clone(),
                });

                self.instrs.push(b::Instr::If);
                let then_value = self.add_value_node(node.required_field("then"));
                self.push_value(&then_value, true);

                assert!(self.stack.len() >= stack_len + 1);
                self.trim_stack(stack_len, 1);

                if let Some(else_node) = node.field("else") {
                    assert!(self.stack.block_len() >= block_len + 1);
                    let block = self.stack.pop_block();
                    self.stack = block.stack.clone();
                    self.idents = block.idents.clone();

                    self.stack.push_block(block);

                    self.instrs.push(b::Instr::Else);
                    let then_value = self.add_value_node(else_node);
                    self.push_value(&then_value, true);

                    assert!(self.stack.len() >= stack_len + 1);
                    self.trim_stack(stack_len, 1);
                } else {
                    todo!("if without else");
                }

                assert!(self.stack.block_len() >= block_len + 1);
                let block = self.stack.pop_block();
                self.stack = block.stack;
                self.idents = block.idents;

                let idx = self.add_instr_with_result(0, b::Instr::End);

                ParserValue::Local(idx)
            }
            k => panic!("Found unexpected expression `{}`", k),
        }
    }

    pub fn push_value(&mut self, value: &ParserValue, will_move: bool) {
        match value {
            ParserValue::MovedLocal => {
                // TODO: better error handling
                panic!("access of moved value")
            }
            ParserValue::Local(idx) => {
                assert!(*idx <= self.stack.len() - 1);

                let rel_value = (self.stack.len() - idx - 1) as b::RelativeValue;

                if will_move && rel_value == 0 {
                    return;
                }

                if will_move {
                    self.instrs.push(b::Instr::Pull(rel_value));
                    self.remove_value(*idx);
                } else {
                    self.instrs.push(b::Instr::Ref(rel_value));
                    self.stack.push(());
                }
            }
            ParserValue::Global(idx) => {
                if let Some(local_idx) = self.loaded_globals.get(idx) {
                    self.push_value(&ParserValue::Local(*local_idx), will_move);
                } else {
                    if !will_move {
                        self.loaded_globals.insert(*idx, self.stack.len());
                    }
                    self.add_instr_with_result(0, b::Instr::GetGlobal(*idx as GlobalIdx));
                }
            }
            ParserValue::Func(_) => todo!("func as value"),
            ParserValue::Bool(v) => {
                self.add_instr_with_result(0, b::Instr::CreateBool(*v));
            }
            ParserValue::Number(v) => {
                self.add_instr_with_result(0, b::Instr::CreateNumber(v.clone()));
            }
            ParserValue::String(v) => {
                self.add_instr_with_result(0, b::Instr::CreateString(v.clone()));
            }
            ParserValue::Array(vs) => {
                for item in vs {
                    self.push_value(item, true);
                }
                self.add_instr_with_result(
                    vs.len(),
                    b::Instr::CreateArray(vs.len() as u32),
                );
            }
            ParserValue::Record(fields) => {
                for (_, value) in fields {
                    self.push_value(value, true);
                }
                let fields: Vec<_> = fields.keys().cloned().collect();
                self.add_instr_with_result(fields.len(), b::Instr::CreateRecord(fields));
            }
        }
    }

    fn remove_value(&mut self, idx: usize) {
        for (_, v) in &mut self.idents {
            if let ParserValue::Local(stored_idx) = v {
                if *stored_idx == idx {
                    *v = ParserValue::MovedLocal;
                } else if *stored_idx > idx {
                    *stored_idx -= 1;
                }
            }
        }
        for (g, stored_idx) in self.loaded_globals.clone() {
            if stored_idx == idx {
                self.loaded_globals.remove(&g);
            } else if stored_idx > idx {
                self.loaded_globals.insert(g, stored_idx - 1);
            }
        }
        self.stack.pop();
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
        self.push_value(&left, false);
        self.push_value(&right, false);
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
                self.push_value(&parent, false);
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
    ) -> ParserValue {
        let mut args: Vec<_> = args.into_iter().collect();
        for arg in &mut args {
            // only if value wont move
            if !matches!(
                arg,
                ParserValue::Local(_) | ParserValue::Bool(_) | ParserValue::Number(_)
            ) {
                self.push_value(arg, true);
                *arg = ParserValue::Local(self.stack.len() - 1);
            }
        }
        for arg in &args {
            self.push_value(arg, false);
        }
        match callee {
            ParserValue::Func(idx) => {
                let idx = self
                    .add_instr_with_result(args.len(), b::Instr::Call(idx as b::FuncIdx));
                ParserValue::Local(idx)
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
                let value = self.add_value_node(node.required_field("value"));
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
