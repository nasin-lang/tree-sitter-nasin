mod entry;

use std::collections::HashSet;
use std::mem;

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

        macro_rules! mut_module {
            () => {
                &mut self.ctx.lock_modules_mut()[self.mod_idx]
            };
        }

        macro_rules! finish_body {
            ($body:expr, $entry:expr) => {
                let len = { ($body).len() };
                for i in 0..len {
                    let instr_entry = &($entry).instrs[i];

                    let get_property_key = {
                        let instr = &mut ($body)[i];
                        match &mut instr.body {
                            b::InstrBody::CreateNumber(ty, _)
                            | b::InstrBody::CreateArray(ty, _)
                            | b::InstrBody::CreateRecord(ty, _)
                            | b::InstrBody::If(ty)
                            | b::InstrBody::Loop(ty, _) => {
                                *ty = self.entries[instr_entry.unwrap()].ty.clone();
                                continue;
                            }
                            b::InstrBody::GetProperty(key) => Some(key.clone()),
                            _ => continue,
                        }
                    };

                    if let Some(key) = get_property_key {
                        let parent = self.entries[instr_entry.unwrap()]
                            .constraints
                            .iter()
                            .filter_map(|c| {
                                let Constraint::IsProperty(target, _key) = c else {
                                    return None;
                                };
                                if &key != _key {
                                    return None;
                                }
                                Some(target)
                            })
                            .take(1)
                            .next();
                        if let Some(parent) = parent {
                            let (is_field, is_method) = {
                                let parent_ty = &self.entries[*parent].ty;
                                let modules = &self.ctx.lock_modules();
                                (
                                    parent_ty.field(&key, modules).is_some(),
                                    parent_ty.method(&key, modules).is_some(),
                                )
                            };

                            {
                                let instr = &mut ($body)[i];
                                if is_field {
                                    instr.body = b::InstrBody::GetField(key.clone());
                                } else if is_method {
                                    instr.body = b::InstrBody::GetMethod(key.clone());
                                }
                            }
                        }
                    }
                }
            };
        }

        for i in 0..self.globals.len() {
            let entry = &self.globals[i];
            {
                mut_module!().globals[i].ty = self.entries[entry.result].ty.clone();
            }
            finish_body!(&mut mut_module!().globals[i].body, entry);
        }
        for i in 0..self.funcs.len() {
            let entry = &self.funcs[i];
            {
                let func = &mut mut_module!().funcs[i];
                for (param, param_entry) in izip!(&mut func.params, &entry.params) {
                    param.ty = self.entries[*param_entry].ty.clone();
                }
                func.ret = self.entries[entry.ret].ty.clone();
            }
            finish_body!(&mut mut_module!().funcs[i].body, entry);
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
            b::InstrBody::GetProperty(v)
            | b::InstrBody::GetField(v)
            | b::InstrBody::GetMethod(v) => {
                assert!(stack.len() >= 1);
                let entry = self.property(stack.pop(), v, instr.loc);
                stack.push(entry);
                Some(entry)
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
                self.add_constraint(
                    entry,
                    Constraint::Members(
                        izip!(fields, &values)
                            .map(|(k, v)| (k.clone(), *v))
                            .collect(),
                    ),
                );
                for (key, value) in izip!(fields, values) {
                    self.add_constraint(
                        entry,
                        Constraint::HasProperty(key.clone(), value),
                    );
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
            b::InstrBody::Not => {
                assert!(stack.len() >= 1);
                let entry = stack.pop();
                // FIXME: use interface/trait
                self.add_constraint(
                    entry,
                    Constraint::Is(b::Type::new(b::TypeBody::Bool, None)),
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
                    let args = stack.pop_many(func.params.len());

                    let entry = self.add_entry(instr.loc);

                    if func_idx.is_some_and(|i| i == *idx) {
                        self.merge_entries(&[entry, func.ret]);
                        for (arg, param) in izip!(args, func.params) {
                            self.merge_entries(&[arg, param]);
                        }
                    } else {
                        for (arg, param) in izip!(args, func.params) {
                            self.add_constraint(arg, Constraint::TypeOf(param));
                        }
                        self.add_constraint(entry, Constraint::TypeOf(func.ret));
                    }

                    entry
                } else {
                    let modules = self.ctx.lock_modules();
                    let func = &modules[*mod_idx].funcs[*idx];

                    let args = stack.pop_many(func.params.len());
                    for (arg, param) in izip!(args, &func.params) {
                        self.add_constraint(arg, Constraint::Is(param.ty.clone()));
                    }

                    self.add_entry_from_type(func.ret.clone(), instr.loc)
                };

                stack.push(entry);
                Some(entry)
            }
            b::InstrBody::IndirectCall(n) => {
                let func = stack.pop();
                let args = stack.pop_many(*n);

                self.add_constraint(func, Constraint::Func(args.len()));

                let entry = self.add_entry(instr.loc);

                for (i, arg) in enumerate(args) {
                    self.add_constraint(arg, Constraint::ParameterOf(func, i));
                }
                self.add_constraint(entry, Constraint::ReturnOf(func));

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

        if let b::TypeBody::Inferred(b::InferredType {
            members,
            properties,
        }) = ty.body
        {
            if members.len() != 0 {
                let members = members
                    .into_iter()
                    .map(|(m_name, m_ty)| (m_name, self.add_entry_from_type(m_ty, loc)))
                    .collect();
                entry.constraints.insert(Constraint::Members(members));
            }
            for (prop_name, prop_ty) in properties {
                let prop_idx = self.add_entry_from_type(prop_ty, loc);
                entry
                    .constraints
                    .insert(Constraint::HasProperty(prop_name, prop_idx));
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

    fn validate(&mut self) -> bool {
        let mut success = true;

        let mut visited = HashSet::new();
        for idx in 0..self.entries.len() {
            success = self.validate_entry(idx, &mut visited) && success;
        }

        success
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
                if let Some(ty) = result_ty.union(ty, &self.ctx.lock_modules()) {
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

            return success;
        }

        let mut constraints = self.entries[idx].constraints.iter().cloned().collect_vec();
        constraints.sort_by(|a, b| b.priority().cmp(&a.priority()));

        for c in constraints {
            let merge_with = match c {
                Constraint::Is(ty) => ty.clone(),
                Constraint::TypeOf(target) => {
                    success = self.validate_entry(target, visited) && success;
                    self.entries[target].ty.clone()
                }
                Constraint::Array(target) => {
                    success = self.validate_entry(target, visited) && success;
                    let ty = self.entries[target].ty.clone();
                    b::Type::new(
                        b::TypeBody::Array(b::ArrayType::new(ty.into(), None)),
                        None,
                    )
                }
                Constraint::Ptr(target) => {
                    success = self.validate_entry(target, visited) && success;
                    let ty = self.entries[target].ty.clone();
                    b::Type::new(b::TypeBody::Ptr(ty.into()), None)
                }
                Constraint::ReturnOf(target) => {
                    success = self.validate_entry(target, visited) && success;
                    if let b::TypeBody::Func(func_ty) = &self.entries[target].ty.body {
                        func_ty.ret.clone()
                    } else {
                        b::Type::unknown(None)
                    }
                }
                Constraint::ParameterOf(target, idx) => {
                    success = self.validate_entry(target, visited) && success;
                    if let b::TypeBody::Func(func_ty) = &self.entries[target].ty.body {
                        func_ty
                            .params
                            .get(idx)
                            .cloned()
                            .unwrap_or(b::Type::unknown(None))
                    } else {
                        b::Type::unknown(None)
                    }
                }
                Constraint::IsProperty(target, key) => {
                    success = self.validate_entry(target, visited) && success;
                    for entry in
                        self.get_property_deps(target, &key, &self.ctx.lock_modules())
                    {
                        success = self.validate_entry(entry, visited) && success;
                    }
                    if let Some(ty) =
                        self.get_property_type(target, &key, &self.ctx.lock_modules())
                    {
                        ty
                    } else {
                        success = false;
                        b::Type::unknown(None)
                    }
                }
                Constraint::Members(members) => {
                    for member in members.values() {
                        success = self.validate_entry(*member, visited) && success;
                    }
                    b::Type::new(
                        b::TypeBody::Inferred(b::InferredType {
                            members: members
                                .iter()
                                .map(|(k, v)| (k.clone(), self.entries[*v].ty.clone()))
                                .collect(),
                            properties: SortedMap::new(),
                        }),
                        None,
                    )
                }
                Constraint::HasProperty(key, target) => {
                    success = self.validate_entry(target, visited) && success;
                    let ty = self.entries[target].ty.clone();
                    b::Type::new(
                        b::TypeBody::Inferred(b::InferredType {
                            properties: SortedMap::from([(key.clone(), ty)]),
                            members: SortedMap::new(),
                        }),
                        None,
                    )
                }
                Constraint::Func(n) => b::Type::new(
                    b::TypeBody::Func(Box::new(b::FuncType {
                        params: vec![b::Type::unknown(None); n],
                        ret: b::Type::unknown(None),
                    })),
                    None,
                ),
            };

            if let Some(result_ty) = self.entries[idx]
                .ty
                .intersection(&merge_with, &self.ctx.lock_modules())
            {
                self.entries[idx].ty = result_ty;
            } else {
                self.ctx.push_error(errors::Error::new(
                    errors::UnexpectedType::new(
                        self.entries[idx].ty.to_owned(),
                        merge_with.clone(),
                    )
                    .into(),
                    self.entries[idx].loc,
                ));
                success = false;
            }
        }

        if success
            && matches!(
                &self.entries[idx].ty.body,
                b::TypeBody::AnyNumber
                    | b::TypeBody::AnySignedNumber
                    | b::TypeBody::AnyFloat
                    | b::TypeBody::Inferred(_)
            )
        {
            self.ctx.push_error(errors::Error::new(
                errors::ErrorDetail::TypeNotFinal,
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
            if let Constraint::HasProperty(prop_name, prop_idx) = item {
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

        self.add_constraint(idx, Constraint::HasProperty(name.to_string(), res));
        self.add_constraint(res, Constraint::IsProperty(idx, name.to_string()));
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

    fn get_property_deps(
        &self,
        idx: TypeCheckEntryIdx,
        key: &str,
        modules: &[b::Module],
    ) -> Vec<TypeCheckEntryIdx> {
        let parent = self.entries[idx].ty.clone();
        let b::TypeBody::TypeRef(mod_idx, ty_idx) = &parent.body else {
            return vec![];
        };
        let Some(func) = modules
            .get(*mod_idx)
            .and_then(|module| module.typedefs.get(*ty_idx))
            .and_then(|typedef| match &typedef.body {
                b::TypeDefBody::Record(rec) => rec.methods.get(key),
            })
            .and_then(|method| {
                if method.func_ref.0 == self.mod_idx {
                    return self.funcs.get(method.func_ref.1);
                } else {
                    return None;
                }
            })
        else {
            return vec![];
        };
        return func.params.iter().cloned().chain([func.ret]).collect();
    }

    fn get_property_type(
        &self,
        idx: TypeCheckEntryIdx,
        key: &str,
        modules: &[b::Module],
    ) -> Option<b::Type> {
        let parent = self.entries[idx].ty.clone();
        'from_entries: {
            let b::TypeBody::TypeRef(mod_idx, ty_idx) = &parent.body else {
                break 'from_entries;
            };
            let Some((func, loc)) = modules
                .get(*mod_idx)
                .and_then(|module| module.typedefs.get(*ty_idx))
                .and_then(|typedef| match &typedef.body {
                    b::TypeDefBody::Record(rec) => rec.methods.get(key),
                })
                .and_then(|method| {
                    if method.func_ref.0 == self.mod_idx {
                        return Some((self.funcs.get(method.func_ref.1)?, method.loc));
                    } else {
                        return None;
                    }
                })
            else {
                break 'from_entries;
            };

            let [params @ .., self_param] = &func.params[..] else {
                break 'from_entries;
            };
            // is static?
            if self.entries[*self_param].ty.body != parent.body {
                return None;
            }
            // functions without parameters are just values
            if params.len() == 0 {
                return Some(self.entries[func.ret].ty.clone());
            }
            return Some(b::Type::new(
                b::TypeBody::Func(
                    b::FuncType::new(
                        params.iter().map(|x| self.entries[*x].ty.clone()).collect(),
                        self.entries[func.ret].ty.clone(),
                    )
                    .into(),
                ),
                Some(loc),
            ));
        }
        return parent.property(key, modules).map(|ty| ty.into_owned());
    }
}

#[derive(Debug, Clone, new)]
struct ScopePayload {
    result: TypeCheckEntryIdx,
    #[new(default)]
    loop_args: Vec<TypeCheckEntryIdx>,
}
