use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;

use cl::InstBuilder;
use cranelift_shim as cl;
use derive_new::new;
use itertools::{izip, Itertools};

use super::globals::{GlobalBinding, Globals};
use super::types::{self, get_size, get_type};
use super::FuncBinding;
use crate::{bytecode as b, utils};

#[derive(new)]
pub struct FuncCodegen<'a, 'b, M: cl::Module> {
    pub modules: &'a [b::Module],
    pub builder: Option<cl::FunctionBuilder<'b>>,
    pub obj_module: M,
    pub globals: Globals<'a>,
    pub funcs: HashMap<(usize, usize), FuncBinding>,
    #[new(value = "utils::ScopeStack::new(ScopePayload::default())")]
    pub scopes: utils::ScopeStack<ScopePayload<'a>>,
    #[new(default)]
    pub values: HashMap<b::ValueIdx, types::RuntimeValue>,
    #[new(default)]
    declared_funcs: HashMap<cl::FuncId, cl::FuncRef>,
}
macro_rules! expect_builder {
    ($self:expr) => {{
        ($self)
            .builder
            .as_mut()
            .expect("function builder should be defined")
    }};
}
impl<'a, M: cl::Module> FuncCodegen<'a, '_, M> {
    pub fn create_initial_block(&mut self, params: &'a [b::ValueIdx], mod_idx: usize) {
        let (block, cl_values) = {
            let func = expect_builder!(self);
            let block = func.create_block();
            func.append_block_params_for_function_params(block);
            (block, func.block_params(block).to_vec())
        };

        for (v, cl_value) in izip!(params, cl_values) {
            let runtime_value = types::RuntimeValue::new(cl_value.into(), mod_idx, *v);
            {
                let this = &mut *self;
                let v = *v;
                let runtime_value = runtime_value.clone();
                this.values.insert(v, runtime_value);
            };
        }

        expect_builder!(self).switch_to_block(block);
        self.scopes.last_mut().block = Some(block);
    }
    #[tracing::instrument(skip(self))]
    pub fn add_instr(&mut self, instr: &'a b::Instr, mod_idx: usize) {
        if self.scopes.last().is_never()
            && !matches!(&instr.body, b::InstrBody::End | b::InstrBody::Else)
        {
            return;
        }

        let mut mark_as_unreachable = instr.results.len() > 0
            && instr
                .results
                .iter()
                .all(|v| self.modules[mod_idx].values[*v].ty.is_never());

        if let Some(value) = self.value_from_instr(instr, mod_idx) {
            self.values.insert(instr.results[0], value);
            return;
        }

        match &instr.body {
            b::InstrBody::Add(a, b)
            | b::InstrBody::Sub(a, b)
            | b::InstrBody::Mul(a, b)
            | b::InstrBody::Div(a, b)
            | b::InstrBody::Mod(a, b)
            | b::InstrBody::Eq(a, b)
            | b::InstrBody::Neq(a, b)
            | b::InstrBody::Gt(a, b)
            | b::InstrBody::Lt(a, b)
            | b::InstrBody::Gte(a, b)
            | b::InstrBody::Lte(a, b) => {
                let lhs = self.use_value(*a);
                let rhs = self.use_value(*b);
                let ty = &self.modules[mod_idx].values[*a].ty;

                let builder = expect_builder!(self);

                let cl_value = match &instr.body {
                    b::InstrBody::Add(..) if ty.is_int() => builder.ins().iadd(lhs, rhs),
                    b::InstrBody::Add(..) if ty.is_float() => {
                        builder.ins().fadd(lhs, rhs)
                    }
                    b::InstrBody::Sub(..) if ty.is_int() => builder.ins().isub(lhs, rhs),
                    b::InstrBody::Sub(..) if ty.is_float() => {
                        builder.ins().fsub(lhs, rhs)
                    }
                    b::InstrBody::Mul(..) if ty.is_int() => builder.ins().imul(lhs, rhs),
                    b::InstrBody::Mul(..) if ty.is_float() => {
                        builder.ins().fmul(lhs, rhs)
                    }
                    b::InstrBody::Div(..) if ty.is_uint() => builder.ins().udiv(lhs, rhs),
                    b::InstrBody::Div(..) if ty.is_sint() => builder.ins().sdiv(lhs, rhs),
                    b::InstrBody::Div(..) if ty.is_float() => {
                        builder.ins().fdiv(lhs, rhs)
                    }
                    b::InstrBody::Mod(..) if ty.is_uint() => builder.ins().urem(lhs, rhs),
                    b::InstrBody::Mod(..) if ty.is_sint() => builder.ins().srem(lhs, rhs),
                    b::InstrBody::Mod(..) if ty.is_float() => {
                        let x = builder.ins().fdiv(lhs, rhs);
                        let x = builder.ins().trunc(x);
                        let y = builder.ins().fneg(rhs);
                        builder.ins().fma(x, y, lhs)
                    }
                    b::InstrBody::Eq(..) if ty.is_int() => {
                        builder.ins().icmp(cl::IntCC::Equal, lhs, rhs)
                    }
                    b::InstrBody::Eq(..) if ty.is_float() => {
                        builder.ins().fcmp(cl::FloatCC::Equal, lhs, rhs)
                    }
                    b::InstrBody::Neq(..) if ty.is_int() => {
                        builder.ins().icmp(cl::IntCC::NotEqual, lhs, rhs)
                    }
                    b::InstrBody::Neq(..) if ty.is_float() => {
                        builder.ins().fcmp(cl::FloatCC::NotEqual, lhs, rhs)
                    }
                    b::InstrBody::Lt(..) if ty.is_sint() => {
                        builder.ins().icmp(cl::IntCC::SignedLessThan, lhs, rhs)
                    }
                    b::InstrBody::Lt(..) if ty.is_uint() => {
                        builder.ins().icmp(cl::IntCC::UnsignedLessThan, lhs, rhs)
                    }
                    b::InstrBody::Lt(..) if ty.is_float() => {
                        builder.ins().fcmp(cl::FloatCC::LessThan, lhs, rhs)
                    }
                    b::InstrBody::Gt(..) if ty.is_sint() => {
                        builder.ins().icmp(cl::IntCC::SignedGreaterThan, lhs, rhs)
                    }
                    b::InstrBody::Gt(..) if ty.is_uint() => {
                        builder.ins().icmp(cl::IntCC::UnsignedGreaterThan, lhs, rhs)
                    }
                    b::InstrBody::Gt(..) if ty.is_float() => {
                        builder.ins().fcmp(cl::FloatCC::GreaterThan, lhs, rhs)
                    }
                    b::InstrBody::Lte(..) if ty.is_sint() => {
                        builder
                            .ins()
                            .icmp(cl::IntCC::SignedLessThanOrEqual, lhs, rhs)
                    }
                    b::InstrBody::Lte(..) if ty.is_uint() => {
                        builder
                            .ins()
                            .icmp(cl::IntCC::UnsignedLessThanOrEqual, lhs, rhs)
                    }
                    b::InstrBody::Lte(..) if ty.is_float() => {
                        builder.ins().fcmp(cl::FloatCC::LessThanOrEqual, lhs, rhs)
                    }
                    b::InstrBody::Gte(..) if ty.is_sint() => {
                        builder
                            .ins()
                            .icmp(cl::IntCC::SignedGreaterThanOrEqual, lhs, rhs)
                    }
                    b::InstrBody::Gte(..) if ty.is_uint() => builder.ins().icmp(
                        cl::IntCC::UnsignedGreaterThanOrEqual,
                        lhs,
                        rhs,
                    ),
                    b::InstrBody::Gte(..) if ty.is_float() => {
                        builder
                            .ins()
                            .fcmp(cl::FloatCC::GreaterThanOrEqual, lhs, rhs)
                    }
                    _ => unreachable!(),
                };

                self.values.insert(
                    instr.results[0],
                    types::RuntimeValue::new(cl_value.into(), mod_idx, instr.results[0]),
                );
            }
            b::InstrBody::Not(cond) => {
                let cond = self.use_value(*cond);
                let builder = expect_builder!(self);

                let v_false = builder.ins().iconst(cl::types::I8, 0);
                let cl_value = builder.ins().icmp(cl::IntCC::Equal, cond, v_false);

                self.values.insert(
                    instr.results[0],
                    types::RuntimeValue::new(cl_value.into(), mod_idx, instr.results[0]),
                );
            }
            b::InstrBody::If(cond_v, target_v) => {
                let builder = expect_builder!(self);
                let cond = self.values[cond_v].add_to_func(&mut self.obj_module, builder);

                let then_block = builder.create_block();
                let else_block = builder.create_block();

                builder.ins().brif(cond, then_block, &[], else_block, &[]);
                builder.switch_to_block(then_block);

                let module = &self.modules[mod_idx];
                let ty = &module.values[*target_v].ty;

                let next_block = builder.create_block();

                if !ty.is_never() {
                    builder.append_block_param(
                        next_block,
                        get_type(ty, self.modules, &self.obj_module),
                    );
                }

                self.scopes.last_mut().block = Some(next_block);

                self.scopes.begin(ScopePayload {
                    start_block: Some(then_block),
                    block: Some(then_block),
                    branches: vec![else_block],
                    next_block: Some(next_block),
                    result_value: if ty.is_never() { None } else { Some(*target_v) },
                    ty: Some(Cow::Borrowed(ty)),
                });
            }
            b::InstrBody::Else => {
                let builder = expect_builder!(self);

                let is_never = self.scopes.last().is_never();

                let scope = self.scopes.branch();
                let else_block = scope.payload.branches.pop().unwrap();
                scope.payload.start_block = Some(else_block);
                scope.payload.block = Some(else_block);

                if !is_never {
                    let value = self.values[&scope.result_value.unwrap()]
                        .add_to_func(&self.obj_module, builder);
                    builder
                        .ins()
                        .jump(scope.payload.next_block.unwrap(), &[value]);
                }

                builder.switch_to_block(else_block);
            }
            b::InstrBody::Loop(acc_vs, target_v) => {
                todo!();
                //let builder = expect_builder!(self);
                //let args = self.stack.pop_many(*n);
                //let mut args_values = vec![];
                //let mut loop_params = vec![];
                //
                //let loop_block = builder.create_block();
                //for arg in &args {
                //    let value = builder.append_block_param(
                //        loop_block,
                //        get_type(&arg.ty, self.modules, &self.obj_module),
                //    );
                //    args_values.push(arg.add_to_func(&mut self.obj_module, builder));
                //    loop_params
                //        .push(types::RuntimeValue::new(arg.ty.clone(), value.into()))
                //}
                //
                //builder.ins().jump(loop_block, &args_values);
                //builder.switch_to_block(loop_block);
                //
                //let next_block = builder.create_block();
                //builder.append_block_param(
                //    next_block,
                //    get_type(ty, self.modules, &self.obj_module),
                //);
                //
                //self.stack.get_scope_mut().payload.block = Some(next_block);
                //
                //let scope = self.stack.create_scope(ScopePayload {
                //    start_block: Some(loop_block),
                //    block: Some(loop_block),
                //    next_block: Some(next_block),
                //    ty: Some(Cow::Borrowed(ty)),
                //    branches: vec![],
                //});
                //scope.is_loop = true;
                //scope.loop_arity = *n;
                //
                //self.stack.extend(loop_params);
            }
            b::InstrBody::End => {
                let builder = expect_builder!(self);

                let scope = self.scopes.end();
                let next_block = scope.payload.next_block.unwrap();

                if !scope.is_never() {
                    let value = self.values[&scope.result_value.unwrap()]
                        .add_to_func(&self.obj_module, builder);
                    builder.ins().jump(next_block, &[value]);
                }

                builder.switch_to_block(next_block);

                let block_params = builder.block_params(next_block);
                assert!(block_params.len() <= 1);
                assert!(block_params.is_empty() == scope.payload.ty.is_none());

                if let [value] = block_params {
                    let runtime_value = (*value).into();
                    let result_v = scope.payload.result_value.unwrap();
                    self.values.insert(
                        scope.payload.result_value.unwrap(),
                        types::RuntimeValue::new(runtime_value, mod_idx, result_v),
                    );
                }
            }
            b::InstrBody::Continue(vs) => {
                let builder = expect_builder!(self);
                let (block, arity) = {
                    let scope = self
                        .scopes
                        .last_loop()
                        .expect("continue instruction should be called in a loop");
                    (scope.payload.start_block.unwrap(), scope.loop_arity)
                };

                let values = vs
                    .into_iter()
                    .map(|v| self.values[v].add_to_func(&mut self.obj_module, builder))
                    .collect_vec();

                builder.ins().jump(block, &values);
                self.scopes.last_mut().mark_as_never();
            }
            b::InstrBody::Call(func_mod_idx, func_idx, vs) => {
                let func_id =
                    self.funcs.get(&(*func_mod_idx, *func_idx)).unwrap().func_id;
                let func = &self.modules[*func_mod_idx].funcs[*func_idx];
                let builder = expect_builder!(self);

                let args = vs
                    .into_iter()
                    .map(|v| self.values[v].add_to_func(&mut self.obj_module, builder))
                    .collect_vec();

                if let Some(value) = self.call(func_id, &args) {
                    self.values.insert(
                        instr.results[0],
                        types::RuntimeValue::new(value.into(), mod_idx, instr.results[0]),
                    );
                } else if self.modules[*func_mod_idx].values[func.ret].ty.is_never() {
                    mark_as_unreachable = true;
                }
            }
            b::InstrBody::IndirectCall(func_v, vs) => {
                let builder = expect_builder!(self);

                let func = self.values[func_v].clone();

                let mut args = vs
                    .into_iter()
                    .map(|v| self.values[v].add_to_func(&mut self.obj_module, builder))
                    .collect_vec();

                match &func.src {
                    types::ValueSource::AppliedMethod(
                        self_value,
                        (func_mod_idx, func_idx),
                    ) => {
                        let func_id =
                            self.funcs.get(&(*func_mod_idx, *func_idx)).unwrap().func_id;
                        let func = &self.modules[*func_mod_idx].funcs[*func_idx];

                        args.push(*self_value);

                        if let Some(value) = self.call(func_id, &args) {
                            self.values.insert(
                                instr.results[0],
                                types::RuntimeValue::new(
                                    value.into(),
                                    mod_idx,
                                    instr.results[0],
                                ),
                            );
                        } else if self.modules[*func_mod_idx].values[func.ret]
                            .ty
                            .is_never()
                        {
                            mark_as_unreachable = true;
                        }
                    }
                    _ => todo!("function as value"),
                }
            }
            b::InstrBody::GetField(source_v, name) => {
                let builder = expect_builder!(self);

                let source_ty = &self.modules[mod_idx].values[*source_v].ty;
                let source = &self.values[source_v];
                let b::Type {
                    body: b::TypeBody::TypeRef(ty_mod_idx, ty_idx),
                    ..
                } = source_ty
                else {
                    panic!("type should be a typeref");
                };
                let b::TypeDefBody::Record(rec) =
                    &self.modules[*ty_mod_idx].typedefs[*ty_idx].body
                else {
                    panic!("type should be a record type");
                };

                let mut offset = 0;
                for (k, v) in &rec.fields {
                    if k == name {
                        break;
                    }
                    offset += get_type(&v.ty, self.modules, &self.obj_module).bytes();
                }

                let source_value = source.add_to_func(&mut self.obj_module, builder);
                let value = builder.ins().load(
                    source.native_type(self.modules, &self.obj_module).clone(),
                    cl::MemFlags::new(),
                    source_value,
                    offset as i32,
                );
                self.values.insert(
                    instr.results[0],
                    types::RuntimeValue::new(value.into(), mod_idx, instr.results[0]),
                );
            }
            b::InstrBody::GetMethod(source_v, name) => {
                let builder = expect_builder!(self);

                let source_ty = &self.modules[mod_idx].values[*source_v].ty;
                let source = &self.values[source_v];
                let b::Type {
                    body: b::TypeBody::TypeRef(ty_mod_idx, ty_idx),
                    ..
                } = source_ty
                else {
                    panic!("type should be a typeref");
                };

                let method = match &self.modules[*ty_mod_idx].typedefs[*ty_idx].body {
                    b::TypeDefBody::Record(rec) => rec
                        .methods
                        .iter()
                        .find(|(k, _)| k == name)
                        .map(|(_, v)| v)
                        .expect("method should be present in record"),
                };

                let value = source.add_to_func(&self.obj_module, builder);
                self.values.insert(
                    instr.results[0],
                    types::RuntimeValue::new(
                        types::ValueSource::AppliedMethod(value, method.func_ref),
                        mod_idx,
                        instr.results[0],
                    ),
                );
            }
            b::InstrBody::ArrayLen(source_v) | b::InstrBody::StrLen(source_v) => {
                let builder = expect_builder!(self);

                let source = self.values[source_v].add_to_func(&self.obj_module, builder);
                let value = builder.ins().load(
                    self.obj_module.isa().pointer_type(),
                    cl::MemFlags::new(),
                    source,
                    0,
                );
                self.values.insert(
                    instr.results[0],
                    types::RuntimeValue::new(
                        types::ValueSource::Value(value),
                        mod_idx,
                        instr.results[0],
                    ),
                );
            }
            b::InstrBody::ArrayPtr(source_v, idx)
            | b::InstrBody::StrPtr(source_v, idx) => {
                let source_ty = &self.modules[mod_idx].values[*source_v].ty;
                let source = self.values[source_v].clone();
                let cl_source =
                    source.add_to_func(&mut self.obj_module, expect_builder!(self));

                let (item_size, len) = match &source_ty.body {
                    b::TypeBody::Array(array_ty) => (
                        get_size(&array_ty.item, &self.modules, &self.obj_module),
                        array_ty.len,
                    ),
                    b::TypeBody::String(str_ty) => (1, str_ty.len),
                    _ => panic!("type should be string or array"),
                };

                if let Some(len) = len {
                    assert!(*idx < len as u64);
                } else {
                    // Check length at runtime
                    let builder = expect_builder!(self);

                    let idx_value = builder
                        .ins()
                        .iconst(self.obj_module.isa().pointer_type(), unsafe {
                            mem::transmute::<_, i64>(*idx)
                        });
                    let len = builder.ins().load(
                        self.obj_module.isa().pointer_type(),
                        cl::MemFlags::new(),
                        cl_source,
                        0,
                    );
                    let cond =
                        builder
                            .ins()
                            .icmp(cl::IntCC::UnsignedLessThan, idx_value, len);
                    self.add_assert(cond, cl::TrapCode::NullReference);
                }

                let builder = expect_builder!(self);

                let offset =
                    self.obj_module.isa().pointer_bytes() as u64 + idx * item_size as u64;
                let offset_value = builder
                    .ins()
                    .iconst(self.obj_module.isa().pointer_type(), unsafe {
                        mem::transmute::<_, i64>(offset)
                    });
                let value = builder.ins().iadd(cl_source, offset_value);

                self.values.insert(
                    instr.results[0],
                    types::RuntimeValue::new(value.into(), mod_idx, instr.results[0]),
                );
            }
            b::InstrBody::Type(..) => {}
            b::InstrBody::GetProperty(..) | b::InstrBody::CompileError => {
                panic!("never should try to compile '{}'", &instr)
            }
            b::InstrBody::CreateNumber(..)
            | b::InstrBody::CreateBool(..)
            | b::InstrBody::CreateString(..)
            | b::InstrBody::CreateArray(..)
            | b::InstrBody::CreateRecord(..)
            | b::InstrBody::GetGlobal(..) => unreachable!(),
        }

        if mark_as_unreachable {
            self.scopes.last_mut().mark_as_never();
            let builder = expect_builder!(self);
            builder.ins().trap(cl::TrapCode::UnreachableCodeReached);
        }
    }
    pub fn return_value(
        mut self,
        v: b::ValueIdx,
    ) -> (M, Globals<'a>, HashMap<(usize, usize), FuncBinding>) {
        let builder = expect_builder!(self);

        let value = self.values[&v].add_to_func(&mut self.obj_module, builder);
        builder.ins().return_(&[value]);

        (self.obj_module, self.globals, self.funcs)
    }
    pub fn return_never(
        mut self,
    ) -> (M, Globals<'a>, HashMap<(usize, usize), FuncBinding>) {
        let func = expect_builder!(self);

        func.ins().trap(cl::TrapCode::UnreachableCodeReached);

        (self.obj_module, self.globals, self.funcs)
    }
    pub fn value_from_instr(
        &mut self,
        instr: &'a b::Instr,
        mod_idx: usize,
    ) -> Option<types::RuntimeValue> {
        utils::replace_with(self, |mut this| {
            let v = 'match_b: {
                match &instr.body {
                    b::InstrBody::CreateNumber(n) => {
                        let module = &self.modules[mod_idx];
                        let ty = &module.values[instr.results[0]].ty;

                        macro_rules! parse_num {
                            ($ty:ty, $variant:ident) => {{
                                let value: $ty = n.parse().unwrap();
                                let src = types::ValueSource::$variant(unsafe {
                                    mem::transmute(value)
                                });
                                Some(types::RuntimeValue::new(
                                    src,
                                    mod_idx,
                                    instr.results[0],
                                ))
                            }};
                        }

                        match &ty.body {
                            b::TypeBody::I8 => parse_num!(i8, I8),
                            b::TypeBody::I16 => parse_num!(i16, I16),
                            b::TypeBody::I32 => parse_num!(i32, I32),
                            b::TypeBody::I64 => parse_num!(i64, I64),
                            b::TypeBody::U8 => parse_num!(u8, I8),
                            b::TypeBody::U16 => parse_num!(u16, I16),
                            b::TypeBody::U32 => parse_num!(u32, I32),
                            b::TypeBody::U64 => parse_num!(u64, I64),
                            b::TypeBody::USize => {
                                match this.obj_module.isa().pointer_bytes() {
                                    1 => parse_num!(u8, I8),
                                    2 => parse_num!(u16, I16),
                                    4 => parse_num!(u32, I32),
                                    8 => parse_num!(u64, I64),
                                    _ => unreachable!(),
                                }
                            }
                            b::TypeBody::F32 => parse_num!(f32, F32),
                            b::TypeBody::F64 => parse_num!(f64, F64),
                            b::TypeBody::Void
                            | b::TypeBody::Never
                            | b::TypeBody::Bool
                            | b::TypeBody::String(_)
                            | b::TypeBody::TypeRef(_, _)
                            | b::TypeBody::Array(_)
                            | b::TypeBody::Ptr(_)
                            | b::TypeBody::Inferred(_)
                            | b::TypeBody::AnyOpaque
                            | b::TypeBody::AnyNumber
                            | b::TypeBody::AnySignedNumber
                            | b::TypeBody::AnyFloat
                            | b::TypeBody::Func(_) => panic!("Cannot parse {n} as {ty}"),
                        }
                    }
                    b::InstrBody::CreateBool(b) => Some(types::RuntimeValue::new(
                        (*b as u8).into(),
                        mod_idx,
                        instr.results[0],
                    )),
                    b::InstrBody::CreateString(s) => {
                        let (data, module) =
                            this.globals.data_for_string(s, this.obj_module);
                        this.obj_module = module;
                        Some(types::RuntimeValue::new(
                            data.into(),
                            mod_idx,
                            instr.results[0],
                        ))
                    }
                    b::InstrBody::CreateArray(vs) => {
                        let (data, module) = this.globals.data_for_array(
                            vs.iter().map(|v| this.values[v].src).collect_vec(),
                            this.obj_module,
                        );
                        this.obj_module = module;
                        let src = if let Some(data) = data {
                            data.into()
                        } else if this.builder.is_some() {
                            this.create_stack_slot(
                                vs.iter().map(|v| this.values[v]).collect_vec(),
                            )
                            .into()
                        } else {
                            break 'match_b None;
                        };
                        Some(types::RuntimeValue::new(src, mod_idx, instr.results[0]))
                    }
                    b::InstrBody::CreateRecord(fields) => {
                        let module = &self.modules[mod_idx];
                        let ty = &module.values[instr.results[0]].ty;

                        let values = types::tuple_from_record(
                            fields
                                .iter()
                                .map(|(name, v)| (name, this.values[v].clone()))
                                .collect_vec(),
                            ty,
                            this.modules,
                        );
                        let (data, module) = this.globals.data_for_tuple(
                            values.iter().map(|value| value.src).collect_vec(),
                            this.obj_module,
                        );
                        this.obj_module = module;
                        let src = if let Some(data) = data {
                            data.into()
                        } else if this.builder.is_some() {
                            this.create_stack_slot(values).into()
                        } else {
                            break 'match_b None;
                        };
                        Some(types::RuntimeValue::new(src, mod_idx, instr.results[0]))
                    }
                    b::InstrBody::GetGlobal(mod_idx, global_idx) => Some(
                        this.globals
                            .get_global(*mod_idx, *global_idx)
                            .expect("global idx out of range")
                            .value
                            .clone(),
                    ),
                    _ => None,
                }
            };

            (this, v)
        })
    }
    pub fn store_global(&mut self, value: types::RuntimeValue, global: &GlobalBinding) {
        let types::ValueSource::Data(data_id) = &global.value.src else {
            panic!("should never try to store a global that is a const");
        };

        let builder = expect_builder!(self);

        let ty = value.native_type(self.modules, &self.obj_module);
        let value = value.add_to_func(&mut self.obj_module, builder);
        let gv = self
            .obj_module
            .declare_data_in_func(*data_id, &mut builder.func);
        let ptr = builder.ins().global_value(ty, gv);
        builder.ins().store(cl::MemFlags::new(), value, ptr, 0);
    }
    pub fn call(&mut self, func_id: cl::FuncId, args: &[cl::Value]) -> Option<cl::Value> {
        let builder = expect_builder!(self);

        let func_ref = match self.declared_funcs.get(&func_id) {
            Some(func_ref) => *func_ref,
            None => {
                let func_ref =
                    self.obj_module.declare_func_in_func(func_id, builder.func);
                self.declared_funcs.insert(func_id, func_ref);
                func_ref
            }
        };

        let instr = builder.ins().call(func_ref, &args);
        let results = builder.inst_results(instr);
        assert!(results.len() <= 1);

        if results.is_empty() {
            None
        } else {
            Some(results[0])
        }
    }

    fn use_value(&mut self, v: b::ValueIdx) -> cl::Value {
        let Some(runtime_value) = self.values.get(&v) else {
            panic!("value should be present in scope: {v}");
        };
        let cl_value =
            runtime_value.add_to_func(&mut self.obj_module, expect_builder!(self));
        cl_value
    }
    fn create_stack_slot(
        &mut self,
        values: impl IntoIterator<Item = types::RuntimeValue>,
    ) -> cl::StackSlot {
        let Some(func) = &mut self.builder else {
            panic!("cannot add stack slot without a function");
        };

        let mut size = 0;
        let values = values
            .into_iter()
            .map(|v| {
                let offset = size;
                size += v.native_type(self.modules, &self.obj_module).bytes();
                (offset, v.add_to_func(&self.obj_module, func))
            })
            .collect_vec();

        let ss_data = cl::StackSlotData::new(cl::StackSlotKind::ExplicitSlot, size);
        let ss = func.create_sized_stack_slot(ss_data);
        for (offset, value) in values {
            func.ins().stack_store(value, ss, offset as i32);
        }

        ss
    }
    fn add_assert(&mut self, cond: cl::Value, code: cl::TrapCode) {
        let builder = expect_builder!(self);
        builder.ins().trapz(cond, code);
    }
}

#[derive(Debug, Default)]
pub struct ScopePayload<'a> {
    pub start_block: Option<cl::Block>,
    pub block: Option<cl::Block>,
    pub next_block: Option<cl::Block>,
    pub branches: Vec<cl::Block>,
    pub result_value: Option<b::ValueIdx>,
    pub ty: Option<Cow<'a, b::Type>>,
}
