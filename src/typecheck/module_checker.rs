use std::collections::HashSet;
use std::{mem, usize};

use derive_new::new;
use itertools::{enumerate, izip};

use super::entry::{Constraint, TypeCheckEntry, TypeCheckEntryIdx};
use super::TypeError;
use crate::utils::SortedMap;
use crate::{bytecode as b, utils};

type Stack = utils::ValueStack<TypeCheckEntryIdx, ScopePayload>;

#[derive(Debug, Clone, new)]
struct GlobalEntry {
    result: TypeCheckEntryIdx,
    #[new(default)]
    instrs: Vec<Option<TypeCheckEntryIdx>>,
}

#[derive(Debug, Clone, new)]
struct FuncEntry {
    params: Vec<TypeCheckEntryIdx>,
    ret: TypeCheckEntryIdx,
    #[new(default)]
    instrs: Vec<Option<TypeCheckEntryIdx>>,
}

#[derive(Debug, Clone)]
pub struct TypeChecker {
    entries: Vec<TypeCheckEntry>,
    globals: Vec<GlobalEntry>,
    funcs: Vec<FuncEntry>,
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
            self.funcs.push(FuncEntry::new(params_idxs, ret_idx));
        }
        for global in &module.globals {
            let idx = self.add_entry_from_type(global.ty.clone());
            self.globals.push(GlobalEntry::new(idx));
        }

        for (i, func) in enumerate(&mut module.funcs) {
            let entry = &self.funcs[i];
            self.funcs[i].instrs = self.add_body(
                &func.body,
                &entry.params.clone(),
                entry.ret,
                Some(i as b::FuncIdx),
            );
        }
        for (i, global) in enumerate(&module.globals) {
            self.globals[i].instrs =
                self.add_body(&global.body, &[], self.globals[i].result, None);
        }

        let errors = self.validate();

        for (global, entry) in izip!(&mut module.globals, &self.globals) {
            global.ty = self.entries[entry.result].ty.clone();

            for (instr, instr_entry) in izip!(&mut global.body, &entry.instrs) {
                if let b::Instr::CreateNumber(ty, _) | b::Instr::CreateRecord(ty, _) =
                    instr
                {
                    *ty = self.entries[instr_entry.unwrap()].ty.clone();
                }
            }
        }
        for (func, entry) in izip!(&mut module.funcs, &self.funcs) {
            for (param, param_entry) in izip!(&mut func.params, &entry.params) {
                param.ty = self.entries[*param_entry].ty.clone();
            }
            func.ret = self.entries[entry.ret].ty.clone();

            for (instr, instr_entry) in izip!(&mut func.body, &entry.instrs) {
                if let b::Instr::CreateNumber(ty, _) | b::Instr::CreateRecord(ty, _) =
                    instr
                {
                    *ty = self.entries[instr_entry.unwrap()].ty.clone();
                }
            }
        }

        (module, errors)
    }

    fn add_body(
        &mut self,
        body: &[b::Instr],
        inputs: &[TypeCheckEntryIdx],
        result: TypeCheckEntryIdx,
        func_idx: Option<b::FuncIdx>,
    ) -> Vec<Option<TypeCheckEntryIdx>> {
        if body.is_empty() {
            return vec![];
        }

        let mut stack = Stack::new(ScopePayload::new(result));
        for input in inputs {
            stack.push(*input);
        }

        let mut instrs_entries = vec![];
        for instr in body {
            let entry = self.add_instr(instr, &mut stack, func_idx);
            instrs_entries.push(entry);
            if let Some(entry) = entry {
            stack.push(entry);
            }
        }

        assert!(stack.len() >= 1);
        assert!(stack.scope_len() == 1);
        self.merge_entries(&[stack.pop(), result]);

        instrs_entries
    }

    fn add_instr(
        &mut self,
        instr: &b::Instr,
        stack: &mut Stack,
        func_idx: Option<b::FuncIdx>,
    ) -> Option<TypeCheckEntryIdx> {
        match instr {
            b::Instr::Dup(v) => Some(*stack.get(*v).unwrap()),
            b::Instr::GetGlobal(v) => Some(self.globals[*v as usize].result),
            b::Instr::GetField(v) => {
                assert!(stack.len() >= 1);
                let entry = *stack.get(0).unwrap();
                let parent = &mut self.entries[entry];
                Some(parent.property(v).unwrap_or_else(|| {
                    let idx = self.add_entry();
                    self.add_constraint(entry, Constraint::Property(v.clone(), idx));
                    idx
                }))
            }
            b::Instr::CreateBool(_) => Some(self.add_entry_from_type(b::Type::Bool)),
            b::Instr::CreateNumber(ty, _) => Some(self.add_entry_from_type(ty.clone())),
            b::Instr::CreateString(v) => {
                Some(self.add_entry_from_type(b::Type::String(b::StringType {
                    len: Some(v.len()),
                })))
            }
            b::Instr::CreateArray(len) => {
                assert!(stack.len() >= *len as usize);
                if *len == 0 {
                    todo!();
                }
                let item_entry = self.merge_entries(&stack.pop_many(*len as usize));
                let entry = self.add_entry();
                self.add_constraint(entry, Constraint::Array(item_entry));
                Some(entry)
            }
            b::Instr::CreateRecord(_, fields) => {
                assert!(stack.len() >= fields.len());
                if fields.len() == 0 {
                    todo!();
                }
                let values = stack.pop_many(fields.len());
                let entry = self.add_entry();
                for (key, value) in izip!(fields, values) {
                    self.add_constraint(entry, Constraint::Property(key.clone(), value));
                }
                Some(entry)
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
                Some(entry)
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
                Some(self.add_entry_from_type(b::Type::Bool))
            }
            b::Instr::Call(idx) => {
                let func = self.funcs[*idx as usize].clone();
                assert!(stack.len() >= func.params.len());

                let args = stack.pop_many(func.params.len());
                for (param, arg) in izip!(func.params, args) {
                    self.add_constraint(arg, Constraint::TypeOf(param));
                }

                let entry = self.add_entry();

                if func_idx.is_some_and(|i| i == *idx) {
                    self.merge_entries(&[entry, func.ret]);
                } else {
                    self.add_constraint(entry, Constraint::TypeOf(func.ret));
                }

                Some(entry)
            }
            b::Instr::If => {
                let cond = stack.pop();
                self.add_constraint(cond, Constraint::Is(b::Type::Bool));

                let entry = self.add_entry();

                stack.create_scope(ScopePayload::new(entry));
                None
            }
            b::Instr::Else => {
                assert!(stack.scope_len() > 1);
                let is_never = stack.get_scope().is_never();
                let (scope, mut removed) = stack.branch_scope();

                if !is_never {
                    assert!(removed.len() >= 1);
                    let res = removed.pop().unwrap();
                    self.merge_entries(&[res, scope.payload.result]);
                }

                None
            }
            b::Instr::End => {
                assert!(stack.scope_len() >= 1);
                let (scope, mut removed) = stack.end_scope();

                if !scope.is_never() {
                    assert!(removed.len() >= 1);
                    let res = removed.pop().unwrap();
                    self.merge_entries(&[res, scope.payload.result]);
                }

                Some(scope.payload.result)
            }
            b::Instr::Loop(n) => {
                assert!(stack.len() >= *n as usize);
                let loop_args = stack.pop_many(*n as usize);

                let entry = self.add_entry();
                let scope = stack.create_scope(ScopePayload::new(entry));
                scope.is_loop = true;
                scope.loop_arity = *n;
                scope.payload.loop_args = loop_args.clone();

                stack.extend(loop_args);
                None
            }
            b::Instr::Continue => {
                assert!(stack.scope_len() >= 1);
                let scope = stack
                    .get_loop_scope()
                    .expect("continue should be inside a loop scope")
                    .clone();
                assert!(stack.len() >= scope.start() + scope.loop_arity as usize);

                for (old, curr) in izip!(
                    scope.payload.loop_args,
                    stack.pop_many(scope.loop_arity as usize)
                ) {
                    if old != curr {
                        self.merge_entries(&[old, curr]);
                    }
                }

                stack.get_scope_mut().mark_as_never();
                None
            }
            b::Instr::CompileError => Some(self.add_entry_from_type(b::Type::unknown())),
        }
    }

    fn add_entry(&mut self) -> TypeCheckEntryIdx {
        self.entries.push(TypeCheckEntry::new(b::Type::unknown()));
        self.entries.len() - 1
    }

    fn add_entry_from_type(&mut self, ty: b::Type) -> TypeCheckEntryIdx {
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

#[derive(Debug, Clone, new)]
struct ScopePayload {
    result: TypeCheckEntryIdx,
    #[new(default)]
    loop_args: Vec<TypeCheckEntryIdx>,
}
