mod constraints;

use std::collections::HashSet;
use std::mem;

use derive_new::new;
use itertools::{enumerate, izip, repeat_n, Itertools};

use self::constraints::Constraint;
use crate::utils::SortedMap;
use crate::{bytecode as b, context, errors, utils};

type Stack = utils::ValueStack<b::ValueIdx, ScopePayload>;

#[derive(Debug, Clone, new)]
pub struct TypeChecker<'a> {
    ctx: &'a context::BuildContext,
    mod_idx: usize,
    #[new(default)]
    value_constraints: Vec<HashSet<Constraint>>,
}

impl<'a> TypeChecker<'a> {
    #[tracing::instrument(skip(self))]
    pub fn check(&mut self) {
        tracing::trace!("check started");

        let (globals_len, funcs_len, values_len) = {
            let module = &self.ctx.lock_modules()[self.mod_idx];
            (
                module.globals.len(),
                module.funcs.len(),
                module.values.len(),
            )
        };

        self.value_constraints = repeat_n(HashSet::new(), values_len).collect();

        for i in 0..funcs_len {
            tracing::trace!(i, "adding func");
            let (params, ret) = {
                let module = &self.ctx.lock_modules()[self.mod_idx];
                (module.funcs[i].params.clone(), module.funcs[i].ret)
            };
            self.add_body(|module| &module.funcs[i].body, &params, ret, Some(i));
        }

        for i in 0..globals_len {
            tracing::trace!(i, "adding global");
            let value = {
                let module = &self.ctx.lock_modules()[self.mod_idx];
                module.globals[i].value
            };
            self.add_body(|module| &module.globals[i].body, &[], value, None);
        }

        self.validate();

        for i in 0..globals_len {
            {
                let module = &mut self.ctx.lock_modules_mut()[self.mod_idx];
                let global = &mut module.globals[i];
                global.ty = module.values[global.value].ty.clone();
            }
            self.finish_body(
                |module| &module.globals[i].body,
                |module| &mut module.globals[i].body,
            );
        }
        for i in 0..funcs_len {
            {
                let module = &mut self.ctx.lock_modules_mut()[self.mod_idx];
                let func = &mut module.funcs[i];
                for (param_desc, param) in izip!(&mut func.params_desc, &func.params) {
                    param_desc.ty = module.values[*param].ty.clone();
                }
                func.ret_ty = module.values[func.ret].ty.clone();
            }
            self.finish_body(
                |module| &module.funcs[i].body,
                |module| &mut module.funcs[i].body,
            );
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn add_body(
        &mut self,
        get_body: impl for<'m> Fn(&'m b::Module) -> &'m [b::Instr],
        inputs: &[b::ValueIdx],
        result: b::ValueIdx,
        func_idx: Option<usize>,
    ) {
        tracing::trace!("add body");

        let body_len = {
            let module = &self.ctx.lock_modules()[self.mod_idx];
            get_body(module).len()
        };

        if body_len == 0 {
            return;
        }

        let mut stack = Stack::new(ScopePayload::new(result));
        for input in inputs {
            stack.push(*input);
        }

        for i in 0..body_len {
            let instr = {
                let module = &self.ctx.lock_modules()[self.mod_idx];
                get_body(module)[i].clone()
            };
            self.add_instr(instr, &mut stack, func_idx);
        }

        assert!(stack.len() >= 1);
        assert!(stack.scope_len() == 1);
        self.merge_types(&[result, stack.pop()]);
    }

    #[tracing::instrument(skip(self, instr, stack), fields(%instr))]
    fn add_instr(&mut self, instr: b::Instr, stack: &mut Stack, func_idx: Option<usize>) {
        match &instr.body {
            b::InstrBody::Dup(rel_value) => {
                let v = *stack.get(*rel_value).unwrap();
                stack.push(v);
            }
            b::InstrBody::GetGlobal(mod_idx, idx) => {
                let v = instr.results[0];
                if *mod_idx == self.mod_idx {
                    let gv = { self.ctx.lock_modules()[*mod_idx].globals[*idx].value };
                    self.merge_types(&[gv, v]);
                } else {
                    let ty =
                        { self.ctx.lock_modules()[*mod_idx].globals[*idx].ty.clone() };
                    self.add_constraint(v, Constraint::Is(ty));
                };
                stack.push(v);
            }
            b::InstrBody::GetProperty(name)
            | b::InstrBody::GetField(name)
            | b::InstrBody::GetMethod(name) => {
                assert!(stack.len() >= 1);
                let v = instr.results[0];
                self.define_property(stack.pop(), v, name, instr.loc);
                stack.push(v);
            }
            b::InstrBody::CreateBool(_) => {
                let v = instr.results[0];
                let ty = b::Type::new(b::TypeBody::Bool, None);
                self.add_constraint(v, Constraint::Is(ty));
                stack.push(v);
            }
            b::InstrBody::CreateNumber(num) => {
                let v = instr.results[0];
                // TODO: use better type
                let ty_body = if num.contains('.') {
                    b::TypeBody::AnyFloat
                } else if num.starts_with('-') {
                    b::TypeBody::AnySignedNumber
                } else {
                    b::TypeBody::AnyNumber
                };
                self.add_constraint(v, Constraint::Is(b::Type::new(ty_body, None)));
                stack.push(v);
            }
            b::InstrBody::CreateString(x) => {
                let v = instr.results[0];
                let ty = b::Type::new(
                    b::TypeBody::String(b::StringType { len: Some(x.len()) }),
                    None,
                );
                self.add_constraint(v, Constraint::Is(ty.clone()));
                stack.push(v);
            }
            b::InstrBody::CreateArray(len) => {
                assert!(stack.len() >= *len);
                let v = instr.results[0];
                if *len > 0 {
                    let items = stack.pop_many(*len);
                    self.merge_types(&items);
                    self.add_constraint(v, Constraint::Array(items[0]));
                } else {
                    let item_ty = b::Type::new(b::TypeBody::Never, None);
                    let arr_ty = b::Type::new(
                        b::TypeBody::Array(b::ArrayType {
                            len: Some(0),
                            item: item_ty.into(),
                        }),
                        None,
                    );
                    self.add_constraint(v, Constraint::Is(arr_ty));
                }
                stack.push(v);
            }
            b::InstrBody::CreateRecord(fields) => {
                assert!(stack.len() >= fields.len());
                let v = instr.results[0];
                let values = stack.pop_many(fields.len());
                self.add_constraint(
                    v,
                    Constraint::Members(
                        izip!(fields, &values)
                            .map(|(k, v)| (k.clone(), *v))
                            .collect(),
                    ),
                );
                for (key, value) in izip!(fields, values) {
                    self.define_property(v, value, key, instr.loc);
                }
                stack.push(v);
            }
            b::InstrBody::Add
            | b::InstrBody::Sub
            | b::InstrBody::Mul
            | b::InstrBody::Div
            | b::InstrBody::Mod => {
                assert!(stack.len() >= 2);
                let v = instr.results[0];
                let mut vs = stack.pop_many(2);
                vs.push(v);
                self.merge_types(&vs);
                // FIXME: use interface/trait
                let ty = b::Type::new(b::TypeBody::AnyNumber, None);
                self.add_constraint(v, Constraint::Is(ty));
                stack.push(v);
            }
            b::InstrBody::Not => {
                assert!(stack.len() >= 1);
                let v = instr.results[0];
                let input = stack.pop();
                self.merge_types(&[input, v]);
                // FIXME: use interface/trait
                self.add_constraint(
                    v,
                    Constraint::Is(b::Type::new(b::TypeBody::Bool, None)),
                );
                stack.push(v);
            }
            b::InstrBody::Eq
            | b::InstrBody::Neq
            | b::InstrBody::Gt
            | b::InstrBody::Gte
            | b::InstrBody::Lt
            | b::InstrBody::Lte => {
                assert!(stack.len() >= 2);
                let v = instr.results[0];
                let operands = stack.pop_many(2);
                self.merge_types(&operands);
                // FIXME: use interface/trait
                let number_ty = b::Type::new(b::TypeBody::AnyNumber, None);
                let bool_ty = b::Type::new(b::TypeBody::Bool, None);
                self.add_constraint(operands[0], Constraint::Is(number_ty));
                self.add_constraint(v, Constraint::Is(bool_ty));
                stack.push(v);
            }
            b::InstrBody::Call(mod_idx, idx) => {
                let v = instr.results[0];

                if *mod_idx == self.mod_idx {
                    let func = self.ctx.lock_modules()[self.mod_idx].funcs[*idx].clone();
                    let args = stack.pop_many(func.params.len());

                    if func_idx.is_some_and(|i| i == *idx) {
                        self.merge_types(&[v, func.ret]);
                        for (arg, param) in izip!(args, func.params) {
                            self.merge_types(&[arg, param]);
                        }
                    } else {
                        for (arg, param) in izip!(args, func.params) {
                            self.add_constraint(arg, Constraint::TypeOf(param));
                        }
                        self.add_constraint(v, Constraint::TypeOf(func.ret));
                    }
                } else {
                    let modules = self.ctx.lock_modules();
                    let func = &modules[*mod_idx].funcs[*idx];

                    let args = stack.pop_many(func.params_desc.len());
                    for (arg, param) in izip!(args, &func.params_desc) {
                        self.add_constraint(arg, Constraint::Is(param.ty.clone()));
                    }
                    self.add_constraint(v, Constraint::Is(func.ret_ty.clone()))
                }

                stack.push(v);
            }
            b::InstrBody::IndirectCall(n) => {
                let v = instr.results[0];

                let func = stack.pop();
                let args = stack.pop_many(*n);

                self.add_constraint(func, Constraint::Func(args.len()));

                for (i, arg) in enumerate(args) {
                    self.add_constraint(arg, Constraint::ParameterOf(func, i));
                }
                self.add_constraint(v, Constraint::ReturnOf(func));

                stack.push(v);
            }
            b::InstrBody::If(v) => {
                let cond = stack.pop();
                self.add_constraint(
                    cond,
                    Constraint::Is(b::Type::new(b::TypeBody::Bool, None)),
                );

                stack.create_scope(ScopePayload::new(*v));
            }
            b::InstrBody::Else => {
                assert!(stack.scope_len() > 1);
                let is_never = stack.get_scope().is_never();
                let (scope, mut removed) = stack.branch_scope();

                if !is_never {
                    assert!(removed.len() >= 1);
                    let res = removed.pop().unwrap();
                    self.merge_types(&[res, scope.payload.result]);
                }
            }
            b::InstrBody::End => {
                assert!(stack.scope_len() >= 1);
                let (scope, mut removed) = stack.end_scope();
                let result = scope.payload.result;

                if !scope.is_never() {
                    assert!(removed.len() >= 1);
                    let res = removed.pop().unwrap();
                    self.merge_types(&[res, result]);
                }

                stack.push(result);
            }
            b::InstrBody::Loop(n) => {
                todo!()
                //assert!(stack.len() >= *n);
                //let loop_args = stack.pop_many(*n);
                //
                //let entry = self.add_entry(instr.loc);
                //let scope = stack.create_scope(ScopePayload::new(entry));
                //scope.is_loop = true;
                //scope.loop_arity = *n;
                //scope.payload.loop_args = loop_args.clone();
                //
                //stack.extend(loop_args);
            }
            b::InstrBody::Continue => {
                todo!()
                //assert!(stack.scope_len() >= 1);
                //let scope = stack
                //    .get_loop_scope()
                //    .expect("continue should be inside a loop scope")
                //    .clone();
                //assert!(stack.len() >= scope.start() + scope.loop_arity);
                //
                //for (old, curr) in
                //    izip!(scope.payload.loop_args, stack.pop_many(scope.loop_arity))
                //{
                //    if old != curr {
                //        self.merge_types(&[old, curr]);
                //    }
                //}
                //
                //stack.get_scope_mut().mark_as_never();
            }
            b::InstrBody::ArrayLen => {
                let v = instr.results[0];
                let input = stack.pop();
                let item_ty = b::Type::unknown(None);
                let arr_ty = b::Type::new(
                    b::TypeBody::Array(b::ArrayType::new(item_ty.into(), None)),
                    None,
                );
                self.add_constraint(input, Constraint::Is(arr_ty));
                let ty = b::Type::new(b::TypeBody::USize, None);
                self.add_constraint(v, Constraint::Is(ty));
                stack.push(v);
            }
            b::InstrBody::ArrayPtr(_) => {
                let v = instr.results[0];
                let source = stack.pop();
                let arr_ty = b::Type::new(
                    b::TypeBody::Array(b::ArrayType::new(
                        b::Type::unknown(None).into(),
                        None,
                    )),
                    None,
                );
                self.add_constraint(source, Constraint::Is(arr_ty));
                self.add_constraint(v, Constraint::ArrayElemPtr(source));
                stack.push(v);
            }
            b::InstrBody::StrLen => {
                let v = instr.results[0];
                let input = stack.pop();
                let str_ty =
                    b::Type::new(b::TypeBody::String(b::StringType::new(None)), None);
                self.add_constraint(input, Constraint::Is(str_ty));
                let ty = b::Type::new(b::TypeBody::USize, None);
                self.add_constraint(v, Constraint::Is(ty));
                stack.push(v);
            }
            b::InstrBody::StrPtr(_) => {
                let v = instr.results[0];
                let input = stack.pop();
                let str_ty =
                    b::Type::new(b::TypeBody::String(b::StringType::new(None)), None);
                self.add_constraint(input, Constraint::Is(str_ty));
                let ty = b::Type::new(
                    b::TypeBody::Ptr(b::Type::new(b::TypeBody::U8, None).into()),
                    None,
                );
                self.add_constraint(v, Constraint::Is(ty));
                stack.push(v);
            }
            b::InstrBody::Type(ty) => {
                assert!(stack.scope_len() >= 1);
                let v = *stack.get(0).unwrap();
                self.add_constraint(v, Constraint::Is(ty.clone()));
            }
            b::InstrBody::CompileError => {}
        }
    }

    #[tracing::instrument(skip(self))]
    fn add_constraint(&mut self, idx: b::ValueIdx, constraint: Constraint) {
        tracing::trace!("add constraint");

        let same_of = {
            let value = &self.ctx.lock_modules()[self.mod_idx].values[idx];
            if let Some(redirects_to) = &value.redirects_to {
                [*redirects_to].into()
            } else {
                value.same_type_of.clone()
            }
        };

        if same_of.len() > 0 {
            for idx in &same_of {
                self.add_constraint(*idx, constraint.clone());
            }
            return;
        }

        // Some constraints cannot be repeated, and instead indicates that two values have
        // the same type. In these cases, e merge the values types
        for c in &self.value_constraints[idx].clone() {
            match (c, &constraint) {
                (Constraint::Array(a), Constraint::Array(b))
                | (Constraint::Ptr(a), Constraint::Ptr(b))
                | (Constraint::ArrayElemPtr(a), Constraint::ArrayElemPtr(b)) => {
                    self.merge_types(&[*a, *b]);
                }
                (
                    Constraint::HasProperty(name_a, a),
                    Constraint::HasProperty(name_b, b),
                )
                | (
                    Constraint::IsProperty(a, name_a),
                    Constraint::IsProperty(b, name_b),
                ) if name_a == name_b => {
                    self.merge_types(&[*a, *b]);
                }
                _ => {}
            }
        }

        self.value_constraints[idx].insert(constraint);
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn merge_types(&mut self, values: &[b::ValueIdx]) {
        tracing::trace!("merge types");

        let mut visited = HashSet::new();

        let head = values[0];
        visited.insert(head);

        let mut merge_with = values[1..].to_vec();

        while let Some(idx) = merge_with.pop() {
            if visited.contains(&idx) {
                continue;
            }

            let constraints = {
                let values = &mut self.ctx.lock_modules_mut()[self.mod_idx].values;

                values[idx].same_type_of.insert(head);
                mem::replace(&mut self.value_constraints[idx], HashSet::new())
            };

            for constraint in constraints {
                self.add_constraint(head, constraint);
            }

            visited.insert(idx);
        }
    }

    #[tracing::instrument(skip(self))]
    fn validate(&mut self) -> bool {
        let mut success = true;
        let mut visited = HashSet::new();

        let len = { self.ctx.lock_modules()[self.mod_idx].values.len() };

        for idx in 0..len {
            success = self.validate_value(idx, &mut visited) && success;
        }

        tracing::info!(success, "validation completed");

        success
    }

    #[tracing::instrument(level = "trace", skip(self, visited))]
    fn validate_value(
        &mut self,
        idx: b::ValueIdx,
        visited: &mut HashSet<b::ValueIdx>,
    ) -> bool {
        if visited.contains(&idx) {
            return true;
        }
        visited.insert(idx);

        let mut success = true;

        let same_of = {
            self.ctx.lock_modules()[self.mod_idx].values[idx]
                .same_type_of
                .clone()
        };

        if same_of.len() > 0 {
            let mut tys = vec![];
            for i in same_of {
                success = self.validate_value(i, visited) && success;
                if !success {
                    return false;
                };
                tys.push(self.ctx.lock_modules()[self.mod_idx].values[i].ty.clone());
            }

            let mut result_ty = tys[0].clone();
            for ty in &tys[1..] {
                if let Some(ty) = result_ty.union(ty, &self.ctx.lock_modules()) {
                    result_ty = ty;
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::TypeMisatch::new(tys).into(),
                        self.ctx.lock_modules()[self.mod_idx].values[idx].loc,
                    ));
                    return false;
                }
            }

            self.ctx.lock_modules_mut()[self.mod_idx].values[idx].ty = result_ty;

            return success;
        }

        let mut constraints = self.value_constraints[idx].iter().cloned().collect_vec();
        constraints.sort_by(|a, b| b.priority().cmp(&a.priority()));

        for c in constraints {
            let merge_with = match c {
                Constraint::Is(ty) => ty.clone(),
                Constraint::TypeOf(target) => {
                    success = self.validate_value(target, visited) && success;
                    self.ctx.lock_modules()[self.mod_idx].values[target]
                        .ty
                        .clone()
                }
                Constraint::Array(target) => {
                    success = self.validate_value(target, visited) && success;
                    let ty = self.ctx.lock_modules()[self.mod_idx].values[target]
                        .ty
                        .clone();
                    b::Type::new(
                        b::TypeBody::Array(b::ArrayType::new(ty.into(), None)),
                        None,
                    )
                }
                Constraint::Ptr(target) => {
                    success = self.validate_value(target, visited) && success;
                    let ty = self.ctx.lock_modules()[self.mod_idx].values[target]
                        .ty
                        .clone();
                    b::Type::new(b::TypeBody::Ptr(ty.into()), None)
                }
                Constraint::ArrayElemPtr(target) => {
                    success = self.validate_value(target, visited) && success;
                    let item_ty = if let b::TypeBody::Array(arr_ty) =
                        &self.ctx.lock_modules()[self.mod_idx].values[target].ty.body
                    {
                        arr_ty.item.clone()
                    } else {
                        b::Type::unknown(None).into()
                    };
                    b::Type::new(b::TypeBody::Ptr(item_ty), None)
                }
                Constraint::ReturnOf(target) => {
                    success = self.validate_value(target, visited) && success;
                    if let b::TypeBody::Func(func_ty) =
                        &self.ctx.lock_modules()[self.mod_idx].values[target].ty.body
                    {
                        func_ty.ret.clone()
                    } else {
                        b::Type::unknown(None)
                    }
                }
                Constraint::ParameterOf(target, idx) => {
                    success = self.validate_value(target, visited) && success;
                    if let b::TypeBody::Func(func_ty) =
                        &self.ctx.lock_modules()[self.mod_idx].values[target].ty.body
                    {
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
                    success = self.validate_value(target, visited) && success;
                    for entry in
                        self.get_property_deps(target, &key, &self.ctx.lock_modules())
                    {
                        success = self.validate_value(entry, visited) && success;
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
                        success = self.validate_value(*member, visited) && success;
                    }
                    b::Type::new(
                        b::TypeBody::Inferred(b::InferredType {
                            members: members
                                .iter()
                                .map(|(k, v)| {
                                    let value =
                                        &self.ctx.lock_modules()[self.mod_idx].values[*v];
                                    (k.clone(), value.ty.clone())
                                })
                                .collect(),
                            properties: SortedMap::new(),
                        }),
                        None,
                    )
                }
                Constraint::HasProperty(key, target) => {
                    success = self.validate_value(target, visited) && success;
                    let ty = {
                        self.ctx.lock_modules()[self.mod_idx].values[target]
                            .ty
                            .clone()
                    };
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

            {
                let modules = &mut self.ctx.lock_modules_mut();
                if let Some(result_ty) = modules[self.mod_idx].values[idx]
                    .ty
                    .intersection(&merge_with, modules)
                {
                    modules[self.mod_idx].values[idx].ty = result_ty;
                } else {
                    self.ctx.push_error(errors::Error::new(
                        errors::UnexpectedType::new(
                            modules[self.mod_idx].values[idx].ty.to_owned(),
                            merge_with.clone(),
                        )
                        .into(),
                        modules[self.mod_idx].values[idx].loc,
                    ));
                    success = false;
                }
            }
        }

        {
            let value = &self.ctx.lock_modules()[self.mod_idx].values[idx];
            if success
                && matches!(
                    &value.ty.body,
                    b::TypeBody::AnyNumber
                        | b::TypeBody::AnySignedNumber
                        | b::TypeBody::AnyFloat
                        | b::TypeBody::Inferred(_)
                )
            {
                self.ctx.push_error(errors::Error::new(
                    errors::ErrorDetail::TypeNotFinal,
                    value.loc,
                ));
                success = false;
            }
        }

        success
    }

    fn define_property(
        &mut self,
        src_v: b::ValueIdx,
        prop_v: b::ValueIdx,
        prop_name: &str,
        loc: b::Loc,
    ) {
        let same_of = {
            self.ctx.lock_modules()[self.mod_idx].values[src_v]
                .same_type_of
                .clone()
        };

        if same_of.len() >= 1 {
            for v in &same_of {
                self.define_property(*v, prop_v, prop_name, loc);
            }
            return;
        }

        for item in &self.value_constraints[src_v] {
            if let Constraint::HasProperty(prop_name_, prop_v_) = item {
                if prop_name == prop_name_ {
                    self.merge_types(&[*prop_v_, prop_v]);
                    return;
                }
            }
        }

        self.add_constraint(
            src_v,
            Constraint::HasProperty(prop_name.to_string(), prop_v),
        );
        self.add_constraint(prop_v, Constraint::IsProperty(src_v, prop_name.to_string()));
    }

    fn get_property_deps(
        &self,
        v: b::ValueIdx,
        name: &str,
        modules: &[b::Module],
    ) -> Vec<b::ValueIdx> {
        let module = &modules[self.mod_idx];
        let parent = &module.values[v].ty;
        let b::TypeBody::TypeRef(mod_idx, ty_idx) = &parent.body else {
            return vec![];
        };
        let Some(func) = modules
            .get(*mod_idx)
            .and_then(|module| module.typedefs.get(*ty_idx))
            .and_then(|typedef| match &typedef.body {
                b::TypeDefBody::Record(rec) => rec.methods.get(name),
            })
            .and_then(|method| {
                if method.func_ref.0 == self.mod_idx {
                    return module.funcs.get(method.func_ref.1);
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
        v: b::ValueIdx,
        key: &str,
        modules: &[b::Module],
    ) -> Option<b::Type> {
        let module = &modules[self.mod_idx];
        let parent = &module.values[v].ty;
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
                        return Some((module.funcs.get(method.func_ref.1)?, method.loc));
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
            if module.values[*self_param].ty.body != parent.body {
                return None;
            }
            // functions without parameters are just values
            if params.len() == 0 {
                return Some(module.values[func.ret].ty.clone());
            }
            return Some(b::Type::new(
                b::TypeBody::Func(
                    b::FuncType::new(
                        params
                            .iter()
                            .map(|x| module.values[*x].ty.clone())
                            .collect(),
                        module.values[func.ret].ty.clone(),
                    )
                    .into(),
                ),
                Some(loc),
            ));
        }
        return parent.property(key, modules).map(|ty| ty.into_owned());
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn finish_body(
        &self,
        get_body: impl for<'m> Fn(&'m b::Module) -> &'m [b::Instr],
        get_body_mut: impl for<'m> Fn(&'m mut b::Module) -> &'m mut [b::Instr],
    ) {
        let len = {
            let module = &self.ctx.lock_modules()[self.mod_idx];
            get_body(module).len()
        };
        for i in 0..len {
            let results = {
                let module = &self.ctx.lock_modules_mut()[self.mod_idx];
                get_body(module)[i].results.clone()
            };

            let get_property_key = {
                let modules = &self.ctx.lock_modules()[self.mod_idx];
                let instr = &get_body(modules)[i];
                match &instr.body {
                    b::InstrBody::GetProperty(key) => Some(key.clone()),
                    _ => None,
                }
            };

            if let Some(key) = get_property_key {
                let mut prop_v = results[0];
                {
                    let module = &self.ctx.lock_modules_mut()[self.mod_idx];
                    loop {
                        let same_type_of = &module.values[prop_v].same_type_of;
                        if same_type_of.len() == 0 {
                            break;
                        }
                        prop_v = *same_type_of.iter().next().unwrap();
                    }
                }
                let constraints = &self.value_constraints[prop_v];
                let mut parent = None;
                for c in constraints {
                    match c {
                        Constraint::IsProperty(target, _key) if &key == _key => {
                            parent = Some(target);
                            break;
                        }
                        _ => {}
                    }
                }
                if let Some(parent) = parent {
                    let (is_field, is_method) = {
                        let modules = &self.ctx.lock_modules();
                        let parent_ty = &modules[self.mod_idx].values[*parent].ty;
                        (
                            parent_ty.field(&key, &modules).is_some(),
                            parent_ty.method(&key, &modules).is_some(),
                        )
                    };

                    {
                        let modules = &mut self.ctx.lock_modules_mut();
                        let instr = &mut get_body_mut(&mut modules[self.mod_idx])[i];
                        if is_field {
                            instr.body = b::InstrBody::GetField(key.clone());
                        } else if is_method {
                            instr.body = b::InstrBody::GetMethod(key.clone());
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, new)]
struct ScopePayload {
    result: b::ValueIdx,
    #[new(default)]
    loop_args: Vec<b::ValueIdx>,
}
