use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;

use cl::{InstBuilder, Module};
use cranelift_shim as cl;
use derive_new::new;
use itertools::{izip, Itertools};

use super::globals::Globals;
use super::types::{self, get_type};
use super::FuncBinding;
use crate::{bytecode as b, utils};

#[derive(new)]
pub struct FuncCodegen<'a: 'b, 'b> {
    pub func: Option<&'b mut cl::FunctionBuilder<'b>>,
    pub module: &'b mut cl::ObjectModule,
    pub globals: &'b mut Globals<'a>,
    pub funcs: &'b [FuncBinding<'a>],
    pub typedefs: &'b [&'a b::TypeDef],
    #[new(value = "utils::ValueStack::new(ScopePayload::default())")]
    pub stack: utils::ValueStack<types::RuntimeValue<'a>, ScopePayload<'a>>,
    #[new(default)]
    func_refs: HashMap<b::FuncIdx, cl::FuncRef>,
}
impl<'a: 'b, 'b> FuncCodegen<'a, 'b> {
    pub fn create_initial_block(&mut self, params: &'a [b::Param]) {
        let Some(func) = &mut self.func else {
            panic!("cannot add stack slot without a function");
        };

        let block = func.create_block();
        func.append_block_params_for_function_params(block);
        for (param, value) in izip!(params, func.block_params(block)) {
            self.stack.push(types::RuntimeValue::new(
                Cow::Borrowed(&param.ty as &_),
                types::ValueSource::Value(*value),
            ));
        }

        func.switch_to_block(block);
        let scope = self.stack.get_scope_mut().payload.block = Some(block);
    }

    pub fn add_instr(&mut self, instr: &'a b::Instr) {
        if self.stack.get_scope().is_never()
            && !matches!(instr, b::Instr::End | b::Instr::Else)
        {
            return;
        }

        if let Some(value) = self.value_from_instr(instr) {
            self.stack.push(value);
            return;
        }

        match instr {
            b::Instr::Add => {
                self.push_bin_op(|func, lhs, rhs, ty| {
                    if ty.is_int() {
                        func.ins().iadd(lhs, rhs)
                    } else if ty.is_float() {
                        func.ins().fadd(lhs, rhs)
                    } else {
                        unreachable!()
                    }
                });
            }
            b::Instr::Sub => {
                self.push_bin_op(|func, lhs, rhs, ty| {
                    if ty.is_int() {
                        func.ins().isub(lhs, rhs)
                    } else if ty.is_float() {
                        func.ins().fsub(lhs, rhs)
                    } else {
                        unreachable!()
                    }
                });
            }
            b::Instr::Mul => {
                self.push_bin_op(|func, lhs, rhs, ty| {
                    if ty.is_int() {
                        func.ins().imul(lhs, rhs)
                    } else if ty.is_float() {
                        func.ins().fmul(lhs, rhs)
                    } else {
                        unreachable!()
                    }
                });
            }
            b::Instr::Div => {
                self.push_bin_op(|func, lhs, rhs, ty| {
                    if ty.is_uint() {
                        func.ins().udiv(lhs, rhs)
                    } else if ty.is_sint() {
                        func.ins().sdiv(lhs, rhs)
                    } else if ty.is_float() {
                        func.ins().fdiv(lhs, rhs)
                    } else {
                        unreachable!()
                    }
                });
            }
            b::Instr::Mod => {
                self.push_bin_op(|func, lhs, rhs, ty| {
                    if ty.is_uint() {
                        func.ins().urem(lhs, rhs)
                    } else if ty.is_sint() {
                        func.ins().srem(lhs, rhs)
                    } else if ty.is_float() {
                        func.ins().fma(
                            func.ins().trunc(func.ins().fdiv(lhs, rhs)),
                            func.ins().fneg(rhs),
                            lhs,
                        )
                    } else {
                        unreachable!()
                    }
                });
            }
            b::Instr::Eq
            | b::Instr::Neq
            | b::Instr::Gt
            | b::Instr::Lt
            | b::Instr::Gte
            | b::Instr::Lte => {
                self.push_bin_op(|func, lhs, rhs, ty| {
                    if ty.is_int() {
                        let cond = match (instr, ty.is_sint()) {
                            (b::Instr::Eq, _) => cl::IntCC::Equal,
                            (b::Instr::Neq, _) => cl::IntCC::NotEqual,
                            (b::Instr::Gt, true) => cl::IntCC::SignedGreaterThan,
                            (b::Instr::Gt, false) => cl::IntCC::UnsignedGreaterThan,
                            (b::Instr::Lt, true) => cl::IntCC::SignedLessThan,
                            (b::Instr::Lt, false) => cl::IntCC::UnsignedLessThan,
                            (b::Instr::Gte, true) => cl::IntCC::SignedGreaterThanOrEqual,
                            (b::Instr::Gte, false) => {
                                cl::IntCC::UnsignedGreaterThanOrEqual
                            }
                            (b::Instr::Lte, true) => cl::IntCC::SignedLessThanOrEqual,
                            (b::Instr::Lte, false) => cl::IntCC::UnsignedLessThanOrEqual,
                            _ => unreachable!(),
                        };
                        func.ins().icmp(cond, lhs, rhs)
                    } else if ty.is_float() {
                        let cond = match (instr, ty.is_sint()) {
                            (b::Instr::Eq, _) => cl::FloatCC::Equal,
                            (b::Instr::Neq, _) => cl::FloatCC::NotEqual,
                            (b::Instr::Gt, true) => cl::FloatCC::GreaterThan,
                            (b::Instr::Lt, true) => cl::FloatCC::LessThan,
                            (b::Instr::Gte, true) => cl::FloatCC::GreaterThanOrEqual,
                            (b::Instr::Lte, true) => cl::FloatCC::LessThanOrEqual,
                            _ => unreachable!(),
                        };
                        func.ins().fcmp(cond, lhs, rhs)
                    } else {
                        unreachable!()
                    }
                });
            }
            b::Instr::If(ty) => {
                let func = self.expect_func();
                let cond = self.stack.pop().add_to_func(&mut self.module, func);

                let then_block = func.create_block();
                let else_block = func.create_block();

                func.ins().brif(cond, then_block, &[], else_block, &[]);
                func.switch_to_block(then_block);

                let next_block = func.create_block();
                func.append_block_param(
                    next_block,
                    get_type(ty, &self.typedefs, &self.module),
                );

                self.stack.get_scope_mut().payload.block = Some(next_block);

                self.stack.create_scope(ScopePayload {
                    block: Some(then_block),
                    branches: vec![else_block],
                    next: Some(next_block),
                    ty: Some(ty),
                });
            }
            b::Instr::Else => {
                let func = self.expect_func();

                let (scope, values) = self.stack.branch_scope();
                let else_block = scope.payload.branches.pop().unwrap();
                scope.payload.block = Some(else_block);

                if !scope.is_never() {
                    let value = values.last().unwrap().add_to_func(&self.module, func);
                    func.ins().jump(scope.payload.next.unwrap(), &[value]);
                }

                func.switch_to_block(else_block);
            }
            b::Instr::Loop(ty, n) => {
                let func = self.expect_func();
                let args = self.stack.pop_many(*n as usize);
                let mut args_values = vec![];

                let loop_block = func.create_block();
                for arg in args {
                    func.append_block_param(
                        loop_block,
                        get_type(&arg.ty, &self.typedefs, &self.module),
                    );
                    args_values.push(arg.add_to_func(&mut self.module, func));
                }

                func.ins().jump(loop_block, &args_values);
                func.switch_to_block(loop_block);

                let next_block = func.create_block();
                func.append_block_param(
                    next_block,
                    get_type(ty, &self.typedefs, &self.module),
                );

                self.stack.get_scope_mut().payload.block = Some(next_block);

                let scope = self.stack.create_scope(ScopePayload {
                    block: Some(loop_block),
                    next: Some(next_block),
                    ty: Some(ty),
                    branches: vec![],
                });
                scope.is_loop = true;
                scope.loop_arity = *n;
            }
            b::Instr::End => {
                let func = self.expect_func();

                let (scope, values) = self.stack.end_scope();
                let next_block = scope.payload.next.unwrap();

                if !scope.is_never() {
                    let value = values.last().unwrap().add_to_func(&self.module, func);
                    func.ins().jump(next_block, &[value]);
                }

                func.switch_to_block(next_block);
            }
            b::Instr::Continue => {
                let func = self.expect_func();
                let scope = self
                    .stack
                    .get_loop_scope()
                    .expect("continue instruction should be called in a loop");

                let loop_block = scope.payload.block.unwrap();
                let values = self
                    .stack
                    .pop_many(scope.loop_arity as usize)
                    .iter()
                    .map(|arg| arg.add_to_func(&mut self.module, func))
                    .collect_vec();

                func.ins().jump(loop_block, &values);
                scope.mark_as_never();
            }
            b::Instr::Call(idx) => {
                let func = self.expect_func();
                let func_binding = self.funcs[*idx as usize];

                let func_ref = match self.func_refs.get(idx) {
                    Some(func_ref) => *func_ref,
                    None => {
                        let func_ref = self.module.declare_func_in_func(
                            func_binding.func_id.clone(),
                            self.expect_func().func,
                        );
                        self.func_refs.insert(*idx, func_ref);
                        func_ref
                    }
                };

                let args = self.stack.pop_many(func_binding.params.len());
                let instr = func.ins().call(
                    func_ref,
                    &args
                        .iter()
                        .map(|arg| arg.add_to_func(&mut self.module, func))
                        .collect_vec(),
                );
                let results = func.inst_results(instr);
                assert!(results.len() == 1);

                self.stack.push(types::RuntimeValue::new(
                    Cow::Borrowed(func_binding.ret),
                    types::ValueSource::Value(results[0]),
                ));
            }
            b::Instr::GetField(name) => {
                let func = self.expect_func();

                let source = self.stack.pop();
                let b::Type::TypeRef(type_ref) = source.ty.as_ref() else {
                    panic!("type should be a record type");
                };
                let b::TypeDefBody::Record(rec) = &self.typedefs[*type_ref as usize].body
                else {
                    panic!("type should be a record type");
                };

                let field = rec
                    .fields
                    .get(name)
                    .expect("field should be present in record");

                let mut offset = 0;
                for (field_name, field) in &rec.fields {
                    if field_name == name {
                        break;
                    }

                    offset += get_type(&field.ty, &self.typedefs, &self.module).bytes();
                }

                let value = func.ins().load(
                    source.native_type(&self.typedefs, self.module).clone(),
                    cl::MemFlags::new(),
                    source.add_to_func(&mut self.module, func),
                    offset as i32,
                );
                self.stack.push(types::RuntimeValue::new(
                    Cow::Borrowed(&field.ty),
                    types::ValueSource::Value(value),
                ));
            }
            b::Instr::CompileError => panic!("never should try to compile CompileError"),
            b::Instr::Dup(..)
            | b::Instr::CreateNumber(..)
            | b::Instr::CreateBool(..)
            | b::Instr::CreateString(..)
            | b::Instr::CreateArray(..)
            | b::Instr::CreateRecord(..)
            | b::Instr::GetGlobal(..) => unreachable!(),
        }
    }

    pub fn value_from_instr(
        &mut self,
        instr: &'a b::Instr,
    ) -> Option<types::RuntimeValue> {
        match instr {
            b::Instr::Dup(n) => Some(*self.stack.get(*n).unwrap()),
            b::Instr::CreateNumber(ty, n) => {
                macro_rules! parse_num {
                    ($ty:ty, $variant:ident) => {{
                        let value: $ty = n.parse().unwrap();
                        let src = types::ValueSource::$variant(unsafe {
                            mem::transmute(value)
                        });
                        Some(types::RuntimeValue::new(Cow::Borrowed(ty), src))
                    }};
                }

                match ty {
                    b::Type::I8 => parse_num!(i8, I8),
                    b::Type::I16 => parse_num!(i16, I16),
                    b::Type::I32 => parse_num!(i32, I32),
                    b::Type::I64 => parse_num!(i64, I64),
                    b::Type::U8 => parse_num!(u8, I8),
                    b::Type::U16 => parse_num!(u16, I16),
                    b::Type::U32 => parse_num!(u32, I32),
                    b::Type::U64 => parse_num!(u64, I64),
                    b::Type::USize => match self.module.isa().pointer_bytes() {
                        1 => parse_num!(u8, I8),
                        2 => parse_num!(u16, I16),
                        4 => parse_num!(u32, I32),
                        8 => parse_num!(u64, I64),
                        _ => unreachable!(),
                    },
                    b::Type::F32 => parse_num!(f32, F32),
                    b::Type::F64 => parse_num!(f64, F64),
                    b::Type::Bool
                    | b::Type::String(_)
                    | b::Type::TypeRef(_)
                    | b::Type::Array(_)
                    | b::Type::Infer(_)
                    | b::Type::AnyNumber
                    | b::Type::AnySignedNumber
                    | b::Type::AnyFloat => panic!("Cannot parse {n} as {ty}"),
                }
            }
            b::Instr::CreateBool(b) => Some(types::RuntimeValue::new(
                Cow::Owned(b::Type::Bool),
                types::ValueSource::I8(*b as u8),
            )),
            b::Instr::CreateString(s) => Some(types::RuntimeValue::new(
                Cow::Owned(b::Type::String(b::StringType::new(Some(s.len())))),
                types::ValueSource::Data(self.globals.data_for_string(s, self.module)),
            )),
            b::Instr::CreateArray(ty, n) => {
                let values = self.stack.pop_many(*n as usize);
                let src = if let Some(data) =
                    self.globals.data_for_tuple(values.clone(), self.module)
                {
                    types::ValueSource::Data(data)
                } else if let Some(func) = self.func {
                    types::ValueSource::StackSlot(self.create_stack_slot(&values))
                } else {
                    return None;
                };
                Some(types::RuntimeValue::new(Cow::Borrowed(ty), src))
            }
            b::Instr::CreateRecord(ty, fields) => {
                let values = types::tuple_from_record(
                    izip!(fields, self.stack.pop_many(fields.len())),
                    ty,
                    self.typedefs,
                );
                let src = if let Some(data) =
                    self.globals.data_for_tuple(values.clone(), self.module)
                {
                    types::ValueSource::Data(data)
                } else if let Some(func) = self.func {
                    types::ValueSource::StackSlot(self.create_stack_slot(&values))
                } else {
                    return None;
                };
                Some(types::RuntimeValue::new(Cow::Borrowed(ty), src))
            }
            b::Instr::GetGlobal(idx) => Some(
                self.globals
                    .get_global(*idx as usize)
                    .expect("global idx out of range")
                    .value
                    .clone(),
            ),
            _ => None,
        }
    }

    fn create_stack_slot(&mut self, values: &[types::RuntimeValue]) -> cl::StackSlot {
        let Some(func) = &mut self.func else {
            panic!("cannot add stack slot without a function");
        };

        let mut size = 0;
        let values = values
            .iter()
            .map(|v| {
                let offset = size;
                size += v.native_type(&self.typedefs, &self.module).bytes();
                (offset, v.add_to_func(self.module, func))
            })
            .collect_vec();

        let ss_data = cl::StackSlotData::new(cl::StackSlotKind::ExplicitSlot, size);
        let ss = func.create_sized_stack_slot(ss_data);
        for (offset, value) in values {
            func.ins().stack_store(value, ss, offset as i32);
        }

        ss
    }

    fn push_bin_op(
        &mut self,
        f: impl FnOnce(&mut cl::FunctionBuilder, cl::Value, cl::Value, &b::Type) -> cl::Value,
    ) {
        let operands = self.stack.pop_many(2);
        assert!(operands[0].ty == operands[1].ty);

        let lhs = operands[0].add_to_func(&mut self.module, self.expect_func());
        let rhs = operands[1].add_to_func(&mut self.module, self.expect_func());

        let value = f(self.expect_func(), lhs, rhs, &operands[0].ty);
        self.stack.push(types::RuntimeValue::new(
            operands[0].ty,
            types::ValueSource::Value(value),
        ));
    }

    fn expect_func(&mut self) -> &mut cl::FunctionBuilder {
        self.func
            .as_mut()
            .expect("function builder should be defined")
    }
}

#[derive(Debug, Default)]
struct ScopePayload<'a> {
    block: Option<cl::Block>,
    next: Option<cl::Block>,
    branches: Vec<cl::Block>,
    ty: Option<&'a b::Type>,
}
