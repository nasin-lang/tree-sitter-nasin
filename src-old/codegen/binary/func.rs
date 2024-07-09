use std::collections::HashMap;

use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{types, Block, GlobalValue, StackSlotData, StackSlotKind};
use cranelift_codegen::ir::{FuncRef, Function, InstBuilder, MemFlags, Value};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataDescription, DataId, FuncId, Module};
use cranelift_object::ObjectModule;
use itertools::izip;

use super::type_gen::TypeGen;
use crate::mir;

// Cranelift's variables are for mutable primitives, immutable primitive can just use values.
// Aggregate values can be either stack slots, if they have known length and are never moved, or
// heap allocated if else, regardless of mutability

pub struct GlobalBinding {
    pub symbol_name: String,
    pub data_id: DataId,
    pub data: DataDescription,
    pub ty: mir::Type,
    pub native_ty: types::Type,
}

pub struct FuncBinding {
    pub is_extern: bool,
    pub symbol_name: String,
    pub func_id: FuncId,
    pub params: Vec<mir::Param>,
    pub ret: Vec<mir::Type>,
}

pub struct LocalBinding {
    pub value: Option<Value>,
    pub ty: mir::Type,
    pub native_ty: types::Type,
}

pub struct FnCodegen<'a> {
    pub symbol_name: String,
    pub module: &'a mut ObjectModule,
    pub builder: FunctionBuilder<'a>,
    pub typedefs: &'a [mir::TypeDef],
    pub globals: &'a [GlobalBinding],
    pub funcs: &'a [FuncBinding],
    pub params: Vec<LocalBinding>,
    pub locals: Vec<LocalBinding>,
    global_values: HashMap<u32, GlobalValue>,
    func_refs: HashMap<u32, FuncRef>,
    jump_stack: Vec<Block>,
    loop_jump_stack: Vec<Block>,
}

impl<'a> FnCodegen<'a> {
    pub fn new(
        symbol_name: &str,
        module: &'a mut ObjectModule,
        func: &'a mut Function,
        func_ctx: &'a mut FunctionBuilderContext,
        typedefs: &'a [mir::TypeDef],
        globals: &'a [GlobalBinding],
        funcs: &'a [FuncBinding],
        params: &'a [mir::Param],
        locals: &'a [mir::Local],
    ) -> Self {
        let mut builder = FunctionBuilder::new(func, func_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);

        let params = izip!(params, builder.block_params(entry_block))
            .map(|(param, value)| LocalBinding {
                value: Some(value.clone()),
                ty: param.ty.clone(),
                native_ty: module.get_type(&param.ty, typedefs),
            })
            .collect();

        builder.switch_to_block(entry_block);

        let locals = locals
            .iter()
            .map(|local| LocalBinding {
                value: None,
                ty: local.ty.clone(),
                native_ty: module.get_type(&local.ty, typedefs),
            })
            .collect();

        FnCodegen {
            symbol_name: symbol_name.to_string(),
            module,
            builder,
            typedefs,
            globals,
            funcs,
            params,
            locals,
            global_values: HashMap::new(),
            func_refs: HashMap::new(),
            jump_stack: vec![],
            loop_jump_stack: vec![],
        }
    }

    pub fn get_value(&self, mir_value: &mir::Value) -> Value {
        let local = match &mir_value {
            mir::Value::Local(idx) => self.locals.get(*idx as usize),
            mir::Value::Param(idx) => self.params.get(*idx as usize),
        }
        .expect(&format!("{:?} not found", mir_value));

        if let Some(value) = &local.value {
            return value.clone();
        }

        panic!("Value {:?} not defined", mir_value);
    }

    pub fn instr(&mut self, instr: &mir::Instr) {
        match instr {
            mir::Instr::Bind(v) => {
                let value = self.get_value(&v.value);
                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                local.value = Some(value);
            }
            mir::Instr::CreateBool(v) => {
                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = self.builder.ins().iconst(local.native_ty, v.value as i64);

                local.value = Some(value);
            }
            mir::Instr::CreateNumber(v) => {
                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = self.builder.ins().iconst(
                    local.native_ty,
                    v.value.parse::<i64>().expect("Invalid number"),
                );

                local.value = Some(value);
            }
            mir::Instr::CreateString(v) => {
                let local = self
                    .locals
                    .get(v.target_idx as usize)
                    .expect("Local not found");

                let ss = self.builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    v.value.len() as u32 + 1,
                ));
                let ptr = self.builder.ins().stack_addr(local.native_ty, ss, 0);

                for (i, byte) in v.value.bytes().enumerate() {
                    let value = self.builder.ins().iconst(types::I8, byte as i64);
                    self.builder
                        .ins()
                        .store(MemFlags::new(), value, ptr, i as i32);
                }

                // Append a null terminator to avoid problems if used as a C string
                let value = self.builder.ins().iconst(types::I8, 0);
                self.builder.ins().store(
                    MemFlags::new(),
                    value,
                    ptr,
                    v.value.len() as i32,
                );

                let local = self.locals.get_mut(v.target_idx as usize).unwrap();
                local.value = Some(ptr);
            }
            mir::Instr::CreateData(v) => {
                let local = self
                    .locals
                    .get(v.target_idx as usize)
                    .expect("Local not found");

                let mir::Type::Array(array_ty) = &local.ty else {
                    panic!("Invalid type for array")
                };
                let item_native_ty = self.module.get_type(&array_ty.item, self.typedefs);

                let ss = self.builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    item_native_ty.bytes() * v.items.len() as u32,
                ));
                let ptr = self.builder.ins().stack_addr(local.native_ty, ss, 0);

                for (i, item) in v.items.iter().enumerate() {
                    let value = self.get_value(item);
                    self.builder.ins().store(
                        MemFlags::new(),
                        value,
                        ptr,
                        (i as u32 * item_native_ty.bytes()) as i32,
                    );
                }

                let local = self.locals.get_mut(v.target_idx as usize).unwrap();
                local.value = Some(ptr);
            }
            mir::Instr::LoadGlobal(v) => {
                let global = self
                    .globals
                    .get(v.global_idx as usize)
                    .expect("Global not found");

                let ptr = self.get_global_ptr(v.global_idx);

                let value = if global.ty.is_primitive() {
                    self.builder
                        .ins()
                        .load(global.native_ty, MemFlags::new(), ptr, 0)
                } else {
                    ptr
                };

                let local = self.locals.get_mut(v.target_idx as usize).unwrap();
                local.value = Some(value);
            }
            mir::Instr::StoreGlobal(v) => {
                let global = self
                    .globals
                    .get(v.global_idx as usize)
                    .expect("Global not found");

                let ptr = self.get_global_ptr(v.global_idx);

                let value = self.get_value(&v.value);

                let offset = match v.field_idx {
                    Some(idx) => match &global.ty {
                        mir::Type::Array(array_ty) => {
                            let item_native_ty =
                                self.module.get_type(&array_ty.item, &self.typedefs);
                            (idx * item_native_ty.bytes()) as i32
                        }
                        _ => panic!("Cannot store field in type {}", &global.ty),
                    },
                    None => 0,
                };

                self.builder
                    .ins()
                    .store(MemFlags::new(), value, ptr, offset);
            }
            mir::Instr::LoadField(v) => {
                let source = match &v.source {
                    mir::Value::Local(idx) => &self.locals[*idx as usize],
                    mir::Value::Param(idx) => &self.params[*idx as usize],
                };

                let offset = match &source.ty {
                    mir::Type::Array(array_ty) => {
                        // FIXME: check length
                        let item_size =
                            self.module.get_type(&array_ty.item, &self.typedefs).bytes();
                        (item_size * v.field_idx) as i32
                    }
                    mir::Type::TypeRef(idx) => {
                        let mir::TypeDefBody::Record(rec_def) =
                            &self.typedefs[*idx as usize].body
                        else {
                            panic!("Expected record type");
                        };

                        let field_idx = v.field_idx as usize;
                        assert!(rec_def.fields.len() > field_idx);

                        let offset = rec_def
                            .fields
                            .iter()
                            .take(field_idx)
                            .map(|f| {
                                self.module.get_type(&f.ty, &self.typedefs).bytes() as i32
                            })
                            .sum();

                        offset
                    }
                    _ => panic!("Cannot get field {} of type {}", v.field_idx, source.ty),
                };

                let value = self.builder.ins().load(
                    source.native_ty.clone(),
                    MemFlags::new(),
                    source.value.expect("source value is not defined yet"),
                    offset,
                );

                self.locals[v.target_idx as usize].value = Some(value);
            }
            mir::Instr::Add(v) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = match local.native_ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => {
                        self.builder.ins().iadd(left, right)
                    }
                    types::F32 | types::F64 => self.builder.ins().fadd(left, right),
                    _ => {
                        panic!("Type {} not supported", &local.ty);
                    }
                };

                local.value = Some(value);
            }
            mir::Instr::Sub(v) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = match local.native_ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => {
                        self.builder.ins().isub(left, right)
                    }
                    types::F32 | types::F64 => self.builder.ins().fsub(left, right),
                    _ => {
                        panic!("Type {} not supported", &local.ty);
                    }
                };

                local.value = Some(value);
            }
            mir::Instr::Mul(v) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = match local.native_ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => {
                        self.builder.ins().imul(left, right)
                    }
                    types::F32 | types::F64 => self.builder.ins().fmul(left, right),
                    _ => {
                        panic!("Type {} not supported", &local.ty);
                    }
                };

                local.value = Some(value);
            }
            mir::Instr::Div(v) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = match local.native_ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => {
                        self.builder.ins().sdiv(left, right)
                    }
                    types::F32 | types::F64 => self.builder.ins().fdiv(left, right),
                    _ => {
                        panic!("Type {} not supported", &local.ty);
                    }
                };

                local.value = Some(value);
            }
            mir::Instr::Mod(v) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = match local.native_ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => {
                        self.builder.ins().srem(left, right)
                    }
                    types::F32 | types::F64 => {
                        panic!("Modulo is not defined for floating point numbers")
                    }
                    _ => {
                        panic!("Type {} not supported", &local.ty);
                    }
                };

                local.value = Some(value);
            }
            mir::Instr::Pow(v) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = match local.native_ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => {
                        // TODO: exponentiation by squaring
                        // https://stackoverflow.com/a/101613
                        self.builder.ins().imul(left, right)
                    }
                    types::F32 | types::F64 => {
                        todo!()
                    }
                    _ => {
                        panic!("Type {} not supported", &local.ty);
                    }
                };

                local.value = Some(value);
            }
            mir::Instr::Call(v) => {
                let func_ref = match self.func_refs.get(&v.func_idx) {
                    Some(func_ref) => *func_ref,
                    None => {
                        let func = self
                            .funcs
                            .get(v.func_idx as usize)
                            .expect("Function not found");
                        let func_ref = self.module.declare_func_in_func(
                            func.func_id.clone(),
                            &mut self.builder.func,
                        );
                        self.func_refs.insert(v.func_idx, func_ref);
                        func_ref
                    }
                };

                let args = v.args.iter().map(|a| self.get_value(a)).collect::<Vec<_>>();

                let instr = self.builder.ins().call(func_ref, &args);
                let results = self.builder.inst_results(instr);

                if let &[value] = results {
                    let local = self
                        .locals
                        .get_mut(v.target_idx as usize)
                        .expect("Local not found");

                    local.value = Some(value);
                }
            }
            mir::Instr::Eq(v)
            | mir::Instr::Neq(v)
            | mir::Instr::Gt(v)
            | mir::Instr::Lt(v)
            | mir::Instr::Gte(v)
            | mir::Instr::Lte(v) => {
                let left = self.get_value(&v.left);
                let right = self.get_value(&v.right);

                // I'm assuming that left and right have the same type
                let input_ty = {
                    match &v.left {
                        mir::Value::Local(i) => self.locals.get(*i as usize),
                        mir::Value::Param(i) => self.params.get(*i as usize),
                    }
                    .expect("Local not found")
                    .ty
                    .clone()
                };

                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let signed = if input_ty.is_signed_int() {
                    true
                } else if input_ty.is_unsigned_int() {
                    false
                } else {
                    panic!("Unhandled type: {input_ty}")
                };

                let cond = match (instr, signed) {
                    (mir::Instr::Eq(..), _) => IntCC::Equal,
                    (mir::Instr::Neq(..), _) => IntCC::NotEqual,
                    (mir::Instr::Gt(..), true) => IntCC::SignedGreaterThan,
                    (mir::Instr::Gt(..), false) => IntCC::UnsignedGreaterThan,
                    (mir::Instr::Lt(..), true) => IntCC::SignedLessThan,
                    (mir::Instr::Lt(..), false) => IntCC::UnsignedLessThan,
                    (mir::Instr::Gte(..), true) => IntCC::SignedGreaterThanOrEqual,
                    (mir::Instr::Gte(..), false) => IntCC::UnsignedGreaterThanOrEqual,
                    (mir::Instr::Lte(..), true) => IntCC::SignedLessThanOrEqual,
                    (mir::Instr::Lte(..), false) => IntCC::UnsignedLessThanOrEqual,
                    _ => unreachable!(),
                };

                let value = self.builder.ins().icmp(cond, left, right);
                local.value = Some(value);
            }
            mir::Instr::If(v) => {
                let cond = self.get_value(&v.cond);

                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();

                self.builder
                    .ins()
                    .brif(cond, then_block, &[], else_block, &[]);

                if !instr.returns() {
                    let next_block = self.builder.create_block();
                    for idx in &v.target_idx_list {
                        let local =
                            self.locals.get_mut(*idx as usize).expect("Local not found");
                        let value = self
                            .builder
                            .append_block_param(next_block, local.native_ty.clone());
                        local.value = Some(value);
                    }

                    self.jump_stack.push(next_block);
                };

                self.builder.switch_to_block(then_block);
                self.instrs(&v.then_body, true);

                self.builder.switch_to_block(else_block);
                self.instrs(&v.else_body, true);

                if instr.returns() {
                    return;
                }

                if let Some(block) = self.jump_stack.pop() {
                    self.builder.switch_to_block(block);
                }
            }
            mir::Instr::Loop(v) => {
                let loop_block = self.builder.create_block();
                self.loop_jump_stack.push(loop_block.clone());

                let mut block_args = vec![];
                for idx in &v.updating_idx_list {
                    let local =
                        self.locals.get_mut(*idx as usize).expect("Local not found");
                    let value =
                        self.builder.append_block_param(loop_block, local.native_ty);

                    block_args
                        .push(local.value.expect("loop value should be defined yet"));

                    local.value = Some(value);
                }
                self.builder.ins().jump(loop_block, &block_args);

                if !instr.returns() {
                    let next_block = self.builder.create_block();
                    for idx in &v.target_idx_list {
                        let local =
                            self.locals.get_mut(*idx as usize).expect("Local not found");
                        let value = self
                            .builder
                            .append_block_param(next_block, local.native_ty.clone());
                        local.value = Some(value);
                    }

                    self.jump_stack.push(next_block);
                };

                self.builder.switch_to_block(loop_block);
                self.instrs(&v.body, true);

                if instr.returns() {
                    return;
                }

                if let Some(block) = self.jump_stack.pop() {
                    self.builder.switch_to_block(block);
                }
            }
            mir::Instr::Return(v) => {
                if v.value.is_none() {
                    self.builder.ins().return_(&[]);
                } else {
                    let values = v
                        .value
                        .as_ref()
                        .map_or(vec![], |value| vec![self.get_value(value)]);
                    self.builder.ins().return_(&values);
                }
            }
            mir::Instr::Break(v) => {
                let jump_args: Vec<_> =
                    v.values.iter().map(|x| self.get_value(x)).collect();

                let Some(block) = self
                    .jump_stack
                    .get(self.jump_stack.len() - v.count as usize)
                else {
                    panic!("Break count is too big");
                };

                self.builder.ins().jump(block.clone(), &jump_args);
            }
            mir::Instr::Continue(v) => {
                let jump_args: Vec<_> =
                    v.values.iter().map(|x| self.get_value(x)).collect();

                let Some(block) = self
                    .loop_jump_stack
                    .get(self.loop_jump_stack.len() - v.count as usize)
                else {
                    dbg!(&v.count, &self.loop_jump_stack);
                    panic!("Continue count is too big");
                };

                self.builder.ins().jump(block.clone(), &jump_args);
            }
        }
    }

    pub fn instrs(&mut self, instrs: &[mir::Instr], can_return: bool) {
        for instr in instrs {
            if instr.returns() && !can_return {
                return;
            }

            self.instr(instr);

            if instr.jumps() {
                return;
            }
        }
    }

    pub fn finalize(mut self) {
        // Sealing the block means that everything before it is done and won't change. Creanelift's
        // documentation recommends sealing each block as soon as possible, but since we're doing a
        // lot of back patching in the function, sealing all the blocks at the end of the function
        // is the only way to go. Maybe the lex typing could be changed to provide all the
        // information we need, like variables names, types and so on, so we can avoid this
        self.builder.seal_all_blocks();

        self.builder.finalize();
    }

    fn get_global_ptr(&mut self, global_idx: u32) -> Value {
        let data = match self.global_values.get(&global_idx) {
            Some(gv) => gv.clone(),
            None => {
                let global = self
                    .globals
                    .get(global_idx as usize)
                    .expect("Global not found");

                self.module
                    .declare_data_in_func(global.data_id.clone(), &mut self.builder.func)
            }
        };

        self.global_values.insert(global_idx, data);

        let ptr = self
            .builder
            .ins()
            .global_value(self.module.poiter_type(), data);

        ptr
    }
}
