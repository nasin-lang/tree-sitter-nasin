mod entry;

use std::collections::HashSet;
use std::{cmp, mem, usize};

use derive_new::new;
use itertools::{enumerate, izip, Itertools};

use self::entry::{Constraint, TypeCheckEntry, TypeCheckEntryIdx};
use crate::utils::SortedMap;
use crate::{bytecode as b, context, errors, utils};

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

#[derive(Debug, Clone, new)]
pub struct TypeChecker<'a> {
    ctx: &'a context::BuildContext,
    mod_idx: usize,
    #[new(default)]
    entries: Vec<TypeCheckEntry>,
    #[new(default)]
    globals: Vec<GlobalEntry>,
    #[new(default)]
    funcs: Vec<FuncEntry>,
}

impl<'a> TypeChecker<'a> {
    pub fn check(&mut self) {
        {
            let module = &self.ctx.lock_modules()[self.mod_idx];

            for func in &module.funcs {
                let params_idxs: Vec<_> = func
                    .params
                    .iter()
                    .map(|p| self.add_entry_from_type(p.ty.clone(), p.loc))
                    .collect();
                let ret_idx = self.add_entry_from_type(
                    func.ret.clone(),
                    func.ret.loc.unwrap_or(func.loc),
                );
                self.funcs.push(FuncEntry::new(params_idxs, ret_idx));
            }
            for global in &module.globals {
                let idx = self.add_entry_from_type(
                    global.ty.clone(),
                    global.ty.loc.unwrap_or(global.loc),
                );
                self.globals.push(GlobalEntry::new(idx));
            }

            for (i, func) in enumerate(&module.funcs) {
                let entry = &self.funcs[i];
                self.funcs[i].instrs =
                    self.add_body(&func.body, &entry.params.clone(), entry.ret, Some(i));
            }
            for (i, global) in enumerate(&module.globals) {
                self.globals[i].instrs =
                    self.add_body(&global.body, &[], self.globals[i].result, None);
            }
        };

        self.validate();

        {
            let module = &mut self.ctx.lock_modules_mut()[self.mod_idx];

            macro_rules! finish_body {
                ($body:expr, $entry:expr) => {
                    for (instr, instr_entry) in izip!(&mut ($body), &($entry).instrs) {
                        if let b::InstrBody::CreateNumber(ty, _)
                        | b::InstrBody::CreateArray(ty, _)
                        | b::InstrBody::CreateRecord(ty, _)
                        | b::InstrBody::If(ty)
                        | b::InstrBody::Loop(ty, _) = &mut instr.body
                        {
                            *ty = self.entries[instr_entry.unwrap()].ty.clone();
                        }
                    }
                };
            }

            for (global, entry) in izip!(&mut module.globals, &self.globals) {
                global.ty = self.entries[entry.result].ty.clone();
                finish_body!(global.body, entry);
            }
            for (func, entry) in izip!(&mut module.funcs, &self.funcs) {
                for (param, param_entry) in izip!(&mut func.params, &entry.params) {
                    param.ty = self.entries[*param_entry].ty.clone();
                }
                func.ret = self.entries[entry.ret].ty.clone();
                finish_body!(func.body, entry);
            }
        }
    }

    fn add_body(
        &mut self,
        body: &[b::Instr],
        inputs: &[TypeCheckEntryIdx],
        result: TypeCheckEntryIdx,
        func_idx: Option<usize>,
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
        func_idx: Option<usize>,
    ) -> Option<TypeCheckEntryIdx> {
        match &instr.body {
            b::InstrBody::Dup(v) => {
                let value = *stack.get(*v).unwrap();
                stack.push(value);
                Some(value)
            }
            b::InstrBody::GetGlobal(mod_idx, idx) => {
                let result = if *mod_idx == self.mod_idx {
                    self.globals[*idx].result
                } else {
                    self.add_entry_from_type(
                        self.ctx.lock_modules()[*mod_idx].globals[*idx].ty.clone(),
                        instr.loc,
                    )
                };
                stack.push(result);
                Some(result)
            }
            b::InstrBody::GetField(v) => {
                assert!(stack.len() >= 1);
                let property = self.property(stack.pop(), v, instr.loc);
                stack.push(property);
                None
            }
            b::InstrBody::CreateBool(_) => {
                let entry = self.add_entry_from_type(
                    b::Type::new(b::TypeBody::Bool, None),
                    instr.loc,
                );
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::CreateNumber(ty, _) => {
                let entry = self.add_entry_from_type(ty.clone(), instr.loc);
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::CreateString(v) => {
                let entry = self.add_entry_from_type(
                    b::Type::new(
                        b::TypeBody::String(b::StringType { len: Some(v.len()) }),
                        None,
                    ),
                    instr.loc,
                );
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::CreateArray(ty, len) => {
                assert!(stack.len() >= *len);
                let item_entry = if *len > 0 {
                    self.merge_entries(&stack.pop_many(*len))
                } else {
                    self.add_entry(instr.loc)
                };
                let entry = self.add_entry(instr.loc);
                self.add_constraint(entry, Constraint::Is(ty.clone()));
                self.add_constraint(entry, Constraint::Array(item_entry));
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::CreateRecord(ty, fields) => {
                assert!(stack.len() >= fields.len());
                let values = stack.pop_many(fields.len());
                let entry = self.add_entry(instr.loc);
                self.add_constraint(entry, Constraint::Is(ty.clone()));
                for (key, value) in izip!(fields, values) {
                    self.add_constraint(entry, Constraint::Property(key.clone(), value));
                }
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::Add
            | b::InstrBody::Sub
            | b::InstrBody::Mul
            | b::InstrBody::Div
            | b::InstrBody::Mod => {
                assert!(stack.len() >= 2);
                let entry = self.merge_entries(&stack.pop_many(2));
                // FIXME: use interface/trait
                self.add_constraint(
                    entry,
                    Constraint::Is(b::Type::new(b::TypeBody::AnyNumber, None)),
                );
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::Eq
            | b::InstrBody::Neq
            | b::InstrBody::Gt
            | b::InstrBody::Gte
            | b::InstrBody::Lt
            | b::InstrBody::Lte => {
                assert!(stack.len() >= 2);
                let operand = self.merge_entries(&stack.pop_many(2));
                // FIXME: use interface/trait
                self.add_constraint(
                    operand,
                    Constraint::Is(b::Type::new(b::TypeBody::AnyNumber, None)),
                );
                let entry = self.add_entry_from_type(
                    b::Type::new(b::TypeBody::Bool, None),
                    instr.loc,
                );
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::Call(mod_idx, idx) => {
                let entry = if *mod_idx == self.mod_idx {
                    let func = self.funcs[*idx].clone();
                    assert!(stack.len() >= func.params.len());

                    let args = stack.pop_many(func.params.len());
                    for (param, arg) in izip!(func.params, args) {
                        self.add_constraint(arg, Constraint::TypeOf(param));
                    }

                    let entry = self.add_entry(instr.loc);

                    if func_idx.is_some_and(|i| i == *idx) {
                        self.merge_entries(&[entry, func.ret]);
                    } else {
                        self.add_constraint(entry, Constraint::TypeOf(func.ret));
                    }

                    entry
                } else {
                    let modules = self.ctx.lock_modules();
                    let func = &modules[*mod_idx].funcs[*idx];

                    let args = stack.pop_many(func.params.len());
                    for (param, arg) in izip!(&func.params, args) {
                        self.add_constraint(arg, Constraint::Is(param.ty.clone()));
                    }

                    self.add_entry_from_type(func.ret.clone(), instr.loc)
                };

                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::If(_) => {
                let cond = stack.pop();
                self.add_constraint(
                    cond,
                    Constraint::Is(b::Type::new(b::TypeBody::Bool, None)),
                );

                let entry = self.add_entry(instr.loc);

                stack.create_scope(ScopePayload::new(entry));
                Some(entry)
            }
            b::InstrBody::Else => {
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
            b::InstrBody::End => {
                assert!(stack.scope_len() >= 1);
                let (scope, mut removed) = stack.end_scope();
                let result = scope.payload.result;

                if !scope.is_never() {
                    assert!(removed.len() >= 1);
                    let res = removed.pop().unwrap();
                    self.merge_entries(&[res, result]);
                }

                stack.push(result);
                Some(result)
            }
            b::InstrBody::Loop(_, n) => {
                assert!(stack.len() >= *n);
                let loop_args = stack.pop_many(*n);

                let entry = self.add_entry(instr.loc);
                let scope = stack.create_scope(ScopePayload::new(entry));
                scope.is_loop = true;
                scope.loop_arity = *n;
                scope.payload.loop_args = loop_args.clone();

                stack.extend(loop_args);
                Some(entry)
            }
            b::InstrBody::Continue => {
                assert!(stack.scope_len() >= 1);
                let scope = stack
                    .get_loop_scope()
                    .expect("continue should be inside a loop scope")
                    .clone();
                assert!(stack.len() >= scope.start() + scope.loop_arity);

                for (old, curr) in
                    izip!(scope.payload.loop_args, stack.pop_many(scope.loop_arity))
                {
                    if old != curr {
                        self.merge_entries(&[old, curr]);
                    }
                }

                stack.get_scope_mut().mark_as_never();
                None
            }
            b::InstrBody::ArrayLen => {
                assert!(stack.scope_len() >= 1);
                let array = stack.pop();
                self.add_constraint(
                    array,
                    Constraint::Is(b::Type::new(
                        b::TypeBody::Array(b::ArrayType::new(
                            b::Type::unknown(None).into(),
                            None,
                        )),
                        None,
                    )),
                );
                let entry = self.add_entry_from_type(
                    b::Type::new(b::TypeBody::USize, None),
                    instr.loc,
                );
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::ArrayPtr(_) => {
                assert!(stack.scope_len() >= 1);
                let source = stack.pop();
                let item = self.array_item(source, instr.loc);
                let entry = self.add_entry(instr.loc);
                self.add_constraint(entry, Constraint::Ptr(item));
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::StrLen => {
                assert!(stack.scope_len() >= 1);
                let string = stack.pop();
                self.add_constraint(
                    string,
                    Constraint::Is(b::Type::new(
                        b::TypeBody::String(b::StringType::new(None)),
                        None,
                    )),
                );
                let entry = self.add_entry_from_type(
                    b::Type::new(b::TypeBody::USize, None),
                    instr.loc,
                );
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::StrPtr(_) => {
                assert!(stack.scope_len() >= 1);
                let string = stack.pop();
                self.add_constraint(
                    string,
                    Constraint::Is(b::Type::new(
                        b::TypeBody::String(b::StringType::new(None)),
                        None,
                    )),
                );
                let item = self
                    .add_entry_from_type(b::Type::new(b::TypeBody::U8, None), instr.loc);
                let entry = self.add_entry(instr.loc);
                self.add_constraint(entry, Constraint::Ptr(item));
                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::CompileError => {
                Some(self.add_entry_from_type(b::Type::unknown(None), instr.loc))
            }
        }
    }

    fn add_entry(&mut self, loc: b::Loc) -> TypeCheckEntryIdx {
        self.entries
            .push(TypeCheckEntry::new(b::Type::unknown(None), loc));
        self.entries.len() - 1
    }

    fn add_entry_from_type(&mut self, ty: b::Type, loc: b::Loc) -> TypeCheckEntryIdx {
        let mut entry = TypeCheckEntry::new(ty.clone(), loc);

        if let b::TypeBody::Inferred(b::InferredType { properties }) = ty.body {
            for (prop_name, prop_ty) in properties {
                let prop_idx = self.add_entry_from_type(prop_ty, loc);
                entry
                    .constraints
                    .push(Constraint::Property(prop_name, prop_idx));
            }
        } else {
            entry.constraints.push(Constraint::Is(ty.clone()));
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

        entry.constraints.push(constraint);
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
            let constraints = mem::replace(&mut self.entries[*idx].constraints, vec![]);

            for constraint in constraints {
                self.add_constraint(head, constraint);
            }

            visited.insert(*idx);
        }

        head
    }

    fn validate(&mut self) {
        let mut visited = HashSet::new();
        for entry in 0..self.entries.len() {
            self.validate_entry(entry, &mut visited);
        }
    }

    fn validate_entry(
        &mut self,
        idx: TypeCheckEntryIdx,
        visited: &mut HashSet<TypeCheckEntryIdx>,
    ) -> bool {
        if visited.contains(&idx) {
            return true;
        }
        visited.insert(idx);

        let mut success = true;

        if self.entries[idx].same_of.len() > 0 {
            let tys: Vec<_> = self.entries[idx]
                .same_of
                .clone()
                .into_iter()
                .map(|same_of| {
                    success = self.validate_entry(same_of, visited) && success;
                    self.entries[same_of].ty.clone()
                })
                .collect();

            if !success {
                return false;
            };

            let mut result_ty = tys[0].clone();
            for ty in &tys[1..] {
                if let Some(ty) = result_ty.common_type(ty, &self.ctx.lock_modules()) {
                    result_ty = ty;
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::TypeMisatch::new(tys).into(),
                        self.entries[idx].loc,
                    ));
                    return false;
                }
            }
            self.entries[idx].ty = result_ty;
        }

        for cons in self.entries[idx].constraints.clone() {
            let dep = match cons {
                Constraint::TypeOf(target)
                | Constraint::Array(target)
                | Constraint::Property(_, target)
                | Constraint::Ptr(target) => target,
                Constraint::Is(_) => continue,
            };
            success = self.validate_entry(dep, visited) && success;
        }

        let mut merge_with = self.entries[idx]
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
                    b::Type::new(
                        b::TypeBody::Array(b::ArrayType::new(ty.into(), None)),
                        None,
                    )
                }
                Constraint::Property(key, target) => {
                    let ty = self.entries[*target].ty.clone();
                    b::Type::new(
                        b::TypeBody::Inferred(b::InferredType {
                            properties: SortedMap::from([(key.clone(), ty)]),
                        }),
                        None,
                    )
                }
                Constraint::Ptr(target) => {
                    let ty = self.entries[*target].ty.clone();
                    b::Type::new(b::TypeBody::Ptr(ty.into()), None)
                }
            })
            .collect_vec();
        merge_with.sort_by(|a, b| match (&a.body, &b.body) {
            (b::TypeBody::Inferred(_), _) => cmp::Ordering::Less,
            (
                b::TypeBody::AnyNumber
                | b::TypeBody::AnySignedNumber
                | b::TypeBody::AnyFloat,
                _,
            ) => cmp::Ordering::Greater,
            _ => cmp::Ordering::Equal,
        });

        for ty in merge_with {
            let entry_ty = &self.entries[idx].ty;
            match entry_ty.intersection(&ty, &self.ctx.lock_modules()) {
                Some(res) => {
                    self.entries[idx].ty = res;
                }
                None => {
                    self.ctx.push_error(errors::Error::new(
                        errors::UnexpectedType::new(entry_ty.to_owned(), ty.clone())
                            .into(),
                        self.entries[idx].loc,
                    ));
                    success = false;
                }
            }
        }

        if matches!(
            &self.entries[idx].ty.body,
            b::TypeBody::AnyNumber
                | b::TypeBody::AnySignedNumber
                | b::TypeBody::AnyFloat
                | b::TypeBody::Inferred(_)
        ) {
            self.ctx.push_error(errors::Error::new(
                errors::ErrorDetail::UnknownType,
                self.entries[idx].loc,
            ));
            success = false;
        }

        success
    }

    fn property(
        &mut self,
        idx: TypeCheckEntryIdx,
        name: &str,
        loc: b::Loc,
    ) -> TypeCheckEntryIdx {
        let entry = &self.entries[idx];

        for item in &entry.constraints {
            if let Constraint::Property(prop_name, prop_idx) = item {
                if prop_name == name {
                    return *prop_idx;
                }
            }
        }

        let res = match entry.same_of.len() {
            0 => self.add_entry(loc),
            1 => self.property(*entry.same_of.iter().next().unwrap(), name, loc),
            _ => {
                let res = self.add_entry(loc);
                for i in self.entries[idx].same_of.clone() {
                    let prop = self.property(i, name, loc);
                    self.entries[res].same_of.insert(prop);
                }
                res
            }
        };

        self.add_constraint(idx, Constraint::Property(name.to_string(), res));
        res
    }

    fn array_item(&mut self, idx: TypeCheckEntryIdx, loc: b::Loc) -> TypeCheckEntryIdx {
        let entry = &self.entries[idx];

        for item in &entry.constraints {
            if let Constraint::Array(item_idx) = item {
                return *item_idx;
            }
        }

        let res = match entry.same_of.len() {
            0 => self.add_entry(loc),
            1 => self.array_item(*entry.same_of.iter().next().unwrap(), loc),
            _ => {
                let res = self.add_entry(loc);
                for i in self.entries[idx].same_of.clone() {
                    let prop = self.array_item(i, loc);
                    self.entries[res].same_of.insert(prop);
                }
                res
            }
        };

        self.add_constraint(idx, Constraint::Array(res));
        res
    }
}

#[derive(Debug, Clone, new)]
struct ScopePayload {
    result: TypeCheckEntryIdx,
    #[new(default)]
    loop_args: Vec<TypeCheckEntryIdx>,
}
