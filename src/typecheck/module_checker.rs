use std::collections::HashSet;
use std::mem;

use itertools::{enumerate, izip};

use super::entry::{Constraint, TypeCheckEntry, TypeCheckEntryIdx};
use super::TypeError;
use crate::utils::SortedMap;
use crate::{bytecode as b, utils};

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlockEntry {
    result: TypeCheckEntryIdx,
    stack: Stack,
    is_loop: bool,
    branches: usize,
    never_branches: usize,
}

type Stack = utils::ValueStack<TypeCheckEntryIdx, BlockEntry>;

#[derive(Debug, Clone)]
pub struct TypeChecker {
    entries: Vec<TypeCheckEntry>,
    globals: Vec<TypeCheckEntryIdx>,
    funcs: Vec<(Vec<TypeCheckEntryIdx>, TypeCheckEntryIdx)>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            globals: vec![],
            funcs: vec![],
        }
    }

    pub fn check_module(&mut self, mut module: b::Module) -> (b::Module, Vec<TypeError>) {
        for func in &mut module.funcs {
            let params_idxs: Vec<_> = func
                .params
                .iter()
                .map(|p| self.add_entry_from_type(p.ty.clone()))
                .collect();
            let ret_idx = self.add_entry_from_type(func.ret.clone());
            self.funcs.push((params_idxs, ret_idx));
        }
        for global in &module.globals {
            let idx = self.add_entry_from_type(global.ty.clone());
            self.globals.push(idx);
        }

        for (i, func) in enumerate(&mut module.funcs) {
            let (params, ret) = self.funcs[i].clone();

            if func.body.len() > 0 {
                let res = self.add_body(&func.body, &params, Some(i as b::FuncIdx));
                self.merge_entries(&[res, ret]);
            }
        }
        for (i, global) in enumerate(&module.globals) {
            if global.body.len() == 0 {
                continue;
            }
            let entry = self.globals[i];

            let res = self.add_body(&global.body, &[], None);
            self.merge_entries(&[res, entry]);
        }

        let errors = self.validate();

        for (i, global) in enumerate(&mut module.globals) {
            let entry = self.globals[i];
            global.ty = self.entries[entry].ty.clone();
        }
        for (i, func) in enumerate(&mut module.funcs) {
            let (params, ret) = &self.funcs[i];
            for (param, param_entry) in izip!(&mut func.params, params) {
                param.ty = self.entries[*param_entry].ty.clone();
            }
            func.ret = self.entries[*ret].ty.clone();
        }

        (module, errors)
    }

    fn add_body(
        &mut self,
        body: &[b::Instr],
        inputs: &[TypeCheckEntryIdx],
        func_idx: Option<b::FuncIdx>,
    ) -> TypeCheckEntryIdx {
        assert!(body.len() >= 1);

        let mut stack = Stack::new();
        for input in inputs {
            stack.push(*input);
        }

        for instr in body {
            self.add_instr(instr, &mut stack, func_idx);
        }
        assert!(stack.len() == 1);
        assert!(stack.block_len() == 0);
        stack.pop()
    }

    fn add_instr(
        &mut self,
        instr: &b::Instr,
        stack: &mut Stack,
        func_idx: Option<b::FuncIdx>,
    ) {
        match instr {
            b::Instr::Dup(v) => stack.dup(*v),
            b::Instr::Drop(v) => {
                stack.drop(*v);
            }
            b::Instr::GetGlobal(v) => {
                stack.push(self.globals[*v as usize]);
            }
            b::Instr::GetField(v) => {
                assert!(stack.len() >= 1);
                let entry = *stack.get(0).unwrap();
                let parent = &mut self.entries[entry];
                let prop = parent.property(v).unwrap_or_else(|| {
                    let idx = self.add_entry();
                    self.add_constraint(entry, Constraint::Property(v.clone(), idx));
                    idx
                });
                stack.push(prop)
            }
            b::Instr::CreateBool(_) => {
                stack.push(self.add_entry_from_type(b::Type::Bool));
            }
            b::Instr::CreateNumber(v) => {
                // TODO: use interface
                let ty = if v.contains('.') {
                    b::Type::AnyFloat
                } else if v.starts_with('-') {
                    b::Type::AnySignedNumber
                } else {
                    b::Type::AnyNumber
                };
                let entry = self.add_entry_from_type(ty);
                stack.push(entry);
            }
            b::Instr::CreateString(v) => {
                let entry = self.add_entry_from_type(b::Type::String(b::StringType {
                    len: Some(v.len()),
                }));
                stack.push(entry);
            }
            b::Instr::CreateArray(len) => {
                assert!(stack.len() >= *len as usize);
                if *len == 0 {
                    todo!();
                }
                let item_entry = self.merge_entries(&stack.pop_many(*len as usize));
                let entry = self.add_entry();
                self.add_constraint(entry, Constraint::Array(item_entry));
                stack.push(entry);
            }
            b::Instr::CreateRecord(fields) => {
                assert!(stack.len() >= fields.len());
                if fields.len() == 0 {
                    todo!();
                }
                let values = stack.pop_many(fields.len());
                let entry = self.add_entry();
                for (key, value) in izip!(fields, values) {
                    self.add_constraint(entry, Constraint::Property(key.clone(), value));
                }
                stack.push(entry);
            }
            b::Instr::Add
            | b::Instr::Sub
            | b::Instr::Mul
            | b::Instr::Div
            | b::Instr::Mod
            | b::Instr::Pow => {
                assert!(stack.len() >= 2);
                let entry = self.merge_entries(&stack.pop_many(2));
                // FIXME: use interface/trait
                self.add_constraint(entry, Constraint::Is(b::Type::AnyNumber));
                stack.push(entry);
            }
            b::Instr::Eq
            | b::Instr::Neq
            | b::Instr::Gt
            | b::Instr::Gte
            | b::Instr::Lt
            | b::Instr::Lte => {
                assert!(stack.len() >= 2);
                let operand = self.merge_entries(&stack.pop_many(2));
                // FIXME: use interface/trait
                self.add_constraint(operand, Constraint::Is(b::Type::AnyNumber));
                stack.push(self.add_entry_from_type(b::Type::Bool));
            }
            b::Instr::Call(idx) => {
                let (params, ret) = self.funcs[*idx as usize].clone();
                assert!(stack.len() >= params.len());

                let args = stack.pop_many(params.len());
                for (param, arg) in izip!(params, args) {
                    self.add_constraint(arg, Constraint::TypeOf(param));
                }

                let entry = self.add_entry();

                if func_idx.is_some_and(|i| i == *idx) {
                    self.merge_entries(&[entry, ret]);
                } else {
                    self.add_constraint(entry, Constraint::TypeOf(ret));
                }

                stack.push(entry);
            }
            b::Instr::If => {
                let cond = stack.pop();
                self.add_constraint(cond, Constraint::Is(b::Type::Bool));

                let entry = self.add_entry();

                stack.push_block(BlockEntry {
                    stack: stack.clone(),
                    result: entry,
                    is_loop: false,
                    branches: 2,
                    never_branches: 0,
                });
            }
            b::Instr::Else => {
                assert!(stack.block_len() >= 1);
                let mut block = stack.pop_block();

                if !stack.unreachable {
                    assert!(stack.len() == block.stack.len() + 1);
                    let res = stack.pop();
                    self.merge_entries(&[res, block.result]);

                    for (old, curr) in izip!(&block.stack, stack as &Stack) {
                        if old != curr {
                            self.merge_entries(&[*old, *curr]);
                        }
                    }
                } else {
                    block.never_branches += 1;
                }

                *stack = block.stack.clone();
                stack.push_block(block);
            }
            b::Instr::End => {
                assert!(stack.block_len() >= 1);
                let mut block = stack.pop_block();

                if !stack.unreachable {
                    assert!(stack.len() == block.stack.len() + 1);
                    let res = stack.pop();
                    self.merge_entries(&[res, block.result]);

                    for (old, curr) in izip!(&block.stack, stack as &Stack) {
                        if old != curr {
                            self.merge_entries(&[*old, *curr]);
                        }
                    }
                } else {
                    block.never_branches += 1;
                }

                *stack = block.stack;
                stack.push(block.result);

                if block.branches == block.never_branches {
                    stack.unreachable = true;
                }
            }
            b::Instr::Loop => {
                let entry = self.add_entry();

                stack.push_block(BlockEntry {
                    stack: stack.clone(),
                    result: entry,
                    is_loop: true,
                    branches: 0,
                    never_branches: 0,
                });
            }
            b::Instr::Continue => {
                assert!(stack.block_len() >= 1);
                let block = stack.find_block(|b| b.is_loop).unwrap();
                assert!(block.stack.len() == stack.len());

                for (old, curr) in izip!(&block.stack, stack as &Stack) {
                    if old != curr {
                        self.merge_entries(&[*old, *curr]);
                    }
                }

                stack.unreachable = true;
            }
            b::Instr::CompileError => {
                let entry = self.add_entry_from_type(b::Type::unknown());
                stack.push(entry);
            }
        }
    }

    fn add_entry(&mut self) -> TypeCheckEntryIdx {
        self.entries.push(TypeCheckEntry::new(b::Type::unknown()));
        self.entries.len() - 1
    }

    fn add_entry_from_type(&mut self, ty: b::Type) -> TypeCheckEntryIdx {
        let ty_str = ty.to_string();

        let mut entry = TypeCheckEntry::new(ty.clone());

        if let b::Type::Infer(b::InferType { properties }) = ty {
            for (prop_name, prop_ty) in properties {
                let prop_idx = self.add_entry_from_type(prop_ty);
                entry
                    .constraints
                    .insert(Constraint::Property(prop_name, prop_idx));
            }
        } else {
            entry.constraints.insert(Constraint::Is(ty.clone()));
        }

        self.entries.push(entry);
        self.entries.len() - 1
    }

    fn add_constraint(&mut self, idx: TypeCheckEntryIdx, constraint: Constraint) {
        let entry = &mut self.entries[idx];

        if entry.same_of.len() > 0 {
            for idx in entry.same_of.clone() {
                self.add_constraint(idx, constraint.clone());
            }
            return;
        }

        entry.constraints.insert(constraint);
    }

    fn merge_entries(&mut self, entries: &[TypeCheckEntryIdx]) -> TypeCheckEntryIdx {
        let mut visited = HashSet::new();

        let head = entries[0];
        visited.insert(head);

        for idx in &entries[1..] {
            if visited.contains(idx) {
                continue;
            }

            self.entries[*idx].same_of.insert(head);
            let constraints =
                mem::replace(&mut self.entries[*idx].constraints, HashSet::new());

            for constraint in constraints {
                self.add_constraint(head, constraint);
            }

            visited.insert(*idx);
        }

        head
    }

    fn validate(&mut self) -> Vec<TypeError> {
        let mut errors = vec![];
        let mut visited = HashSet::new();

        for entry in 0..self.entries.len() {
            errors.extend(self.validate_entry(entry, &mut visited));
        }

        errors
    }

    fn validate_entry(
        &mut self,
        idx: TypeCheckEntryIdx,
        visited: &mut HashSet<TypeCheckEntryIdx>,
    ) -> Vec<TypeError> {
        if visited.contains(&idx) {
            return vec![];
        }
        visited.insert(idx);

        let mut errors = vec![];

        if self.entries[idx].same_of.len() > 0 {
            let tys: Vec<_> = self.entries[idx]
                .same_of
                .clone()
                .into_iter()
                .map(|same_of| {
                    errors.extend(self.validate_entry(same_of, visited));
                    let ty = self.entries[same_of].ty.clone();
                    ty
                })
                .collect();

            if errors.len() > 0 {
                return errors;
            }

            let mut result_ty = tys[0].clone();
            for ty in &tys[1..] {
                if let Some(ty) = result_ty.common_type(ty, &[]) {
                    result_ty = ty;
                } else {
                    return vec![TypeError::TypeMisatch(tys)];
                }
            }
            self.entries[idx].ty = result_ty;

            return vec![];
        }

        for cons in self.entries[idx].constraints.clone() {
            let dep = match cons {
                Constraint::TypeOf(target)
                | Constraint::Array(target)
                | Constraint::Property(_, target) => target,
                Constraint::Is(_) => continue,
            };

            errors.extend(self.validate_entry(dep, visited));
        }

        let merge_with: Vec<_> = self.entries[idx]
            .constraints
            .iter()
            .map(|cons| match cons {
                Constraint::Is(ty) => ty.clone(),
                Constraint::TypeOf(target) => {
                    let ty = self.entries[*target].ty.clone();
                    ty
                }
                Constraint::Array(target) => {
                    let ty = self.entries[*target].ty.clone();
                    b::Type::Array(b::ArrayType::new(ty, None))
                }
                Constraint::Property(key, target) => {
                    let ty = self.entries[*target].ty.clone();
                    b::Type::Infer(b::InferType {
                        properties: SortedMap::from([(key.clone(), ty)]),
                    })
                }
            })
            .collect();

        for ty in merge_with {
            let entry_ty = &self.entries[idx].ty;
            match entry_ty.intersection(&ty, &[]) {
                Some(res) => {
                    self.entries[idx].ty = res;
                }
                None => {
                    errors.push(TypeError::UnexpectedType {
                        expected: ty.clone(),
                        actual: entry_ty.to_owned(),
                    });
                }
            }
        }

        errors
    }
}
