use std::collections::HashMap;

use cranelift_codegen::ir::{types, StackSlotData, StackSlotKind};
use cranelift_codegen::ir::{FuncRef, Function, InstBuilder, MemFlags, Value};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{DataId, FuncId, Module};
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
    pub globals: &'a [GlobalBinding],
    pub funcs: &'a [FuncBinding],
    pub params: Vec<LocalBinding>,
    pub locals: Vec<LocalBinding>,
    global_ptrs: HashMap<u32, Value>,
    func_refs: HashMap<u32, FuncRef>,
}

impl<'a> FnCodegen<'a> {
    pub fn new(
        symbol_name: &str,
        module: &'a mut ObjectModule,
        func: &'a mut Function,
        func_ctx: &'a mut FunctionBuilderContext,
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
                native_ty: module.get_type(&param.ty),
            })
            .collect();

        builder.switch_to_block(entry_block);

        let locals = locals
            .iter()
            .map(|local| LocalBinding {
                value: None,
                ty: local.ty.clone(),
                native_ty: module.get_type(&local.ty),
            })
            .collect();

        FnCodegen {
            symbol_name: symbol_name.to_string(),
            module,
            builder,
            globals,
            funcs,
            params,
            locals,
            global_ptrs: HashMap::new(),
            func_refs: HashMap::new(),
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
            mir::Instr::CreateNumber(v) => {
                let local = self
                    .locals
                    .get_mut(v.target_idx as usize)
                    .expect("Local not found");

                let value = self.builder.ins().iconst(
                    local.native_ty,
                    v.number.parse::<i64>().expect("Invalid number"),
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
                    v.string.len() as u32,
                ));
                let ptr = self.builder.ins().stack_addr(local.native_ty, ss, 0);

                for (i, byte) in v.string.bytes().enumerate() {
                    let value = self.builder.ins().iconst(types::I8, byte as i64);
                    self.builder
                        .ins()
                        .store(MemFlags::new(), value, ptr, i as i32);
                }

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
                let item_native_ty = self.module.get_type(&array_ty.item);

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
                            let item_native_ty = self.module.get_type(&array_ty.item);
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
                        let func_ref = self
                            .module
                            .declare_func_in_func(func.func_id.clone(), &mut self.builder.func);
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
        match self.global_ptrs.get(&global_idx) {
            Some(ptr) => ptr.clone(),
            None => {
                let global = self
                    .globals
                    .get(global_idx as usize)
                    .expect("Global not found");

                let data = self
                    .module
                    .declare_data_in_func(global.data_id.clone(), &mut self.builder.func);
                let ptr = self
                    .builder
                    .ins()
                    .global_value(self.module.poiter_type(), data);

                self.global_ptrs.insert(global_idx, ptr.clone());

                ptr
            }
        }
    }
}
