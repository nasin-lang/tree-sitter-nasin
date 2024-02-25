use std::collections::HashMap;
use std::iter::zip;

use cranelift_codegen::ir::{types, Block, Function, InstBuilder, MemFlags, Value};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{FuncOrDataId, Module};
use cranelift_object::ObjectModule;

use super::variable_ref::VariableRef;
use crate::proto::lex;

pub struct FnCodegen<'a> {
    pub module: &'a mut ObjectModule,
    pub builder: FunctionBuilder<'a>,
    // Store the last values of a BodyResult, so they can be accessed later when managing the
    // control flow, manly for `if` and when building data initialization
    pub last_result: Vec<Value>,
    variables: HashMap<String, VariableRef>,
}

impl<'a> FnCodegen<'a> {
    pub fn new(
        module: &'a mut ObjectModule,
        func: &'a mut Function,
        func_ctx: &'a mut FunctionBuilderContext,
    ) -> Self {
        FnCodegen {
            module,
            variables: HashMap::new(),
            last_result: Vec::new(),
            builder: FunctionBuilder::new(func, func_ctx),
        }
    }

    pub fn build(
        module: &'a mut ObjectModule,
        func: &'a mut Function,
        func_ctx: &'a mut FunctionBuilderContext,
        decl: &lex::FnDecl,
    ) {
        let mut this = Self::new(module, func, func_ctx);

        this.create_entry_block(&decl.args);

        for instr in decl.body.iter() {
            if let Some(instr) = instr.instr.as_ref() {
                this.instr(instr);
            }
        }

        println!("<{}>: {}", decl.name, &this.builder.func);

        this.finalize();
    }

    pub fn build_value(&mut self, lex_value: &lex::Value) -> Value {
        match lex_value.value.as_ref() {
            Some(lex::value::Value::Num(num)) => {
                let ptr_ty = self.module.target_config().pointer_type();
                // maybe this parsing should be handled by the lexer
                self.builder
                    .ins()
                    .iconst(ptr_ty, num.parse::<i64>().unwrap())
            }
            Some(lex::value::Value::Ident(ident)) => {
                if let Some(var_ref) = self.variables.get(ident) {
                    return var_ref.get_value(&self.builder);
                }

                // Fall back to a global variable
                match self.module.get_name(ident) {
                    Some(FuncOrDataId::Data(data_id)) => {
                        let data = self
                            .module
                            .declare_data_in_func(data_id, &mut self.builder.func);
                        // FIXME: hardcoded type
                        let ptr_ty = self.module.target_config().pointer_type();
                        let ptr = self.builder.ins().global_value(ptr_ty, data);

                        // It would probably better to hold the pointer and deref it lazily instead
                        self.builder.ins().load(ptr_ty, MemFlags::new(), ptr, 0)
                    }
                    Some(FuncOrDataId::Func(_)) => {
                        todo!();
                    }
                    None => {
                        unreachable!();
                    }
                }
            }
            None => {
                unreachable!();
            }
        }
    }

    pub fn store_value(&mut self, name: &str, _ty: types::Type, value: Value) {
        // FIXME: check type for what kind of VariableRef to use
        let var = VariableRef::ImmPrimitive(value);
        self.variables.insert(name.to_string(), var);
    }

    pub fn create_entry_block(&mut self, args: &[String]) -> Block {
        let entry_block = self.builder.create_block();
        self.builder
            .append_block_params_for_function_params(entry_block);
        self.builder.switch_to_block(entry_block);

        let entry_block_params = self.builder.block_params(entry_block).to_vec();
        for (arg, block_param) in zip(args.iter(), entry_block_params.iter()) {
            // FIXME: hardcoded type
            let ptr_ty = self.module.target_config().pointer_type();
            self.store_value(&arg, ptr_ty, *block_param);
        }

        entry_block
    }

    pub fn instr(&mut self, instr: &lex::instr::Instr) {
        match instr {
            lex::instr::Instr::Assign(assign) => {
                let value = self.build_value(&assign.value);
                // FIXME: hardcoded type
                let ptr_ty = self.module.target_config().pointer_type();
                self.store_value(&assign.name, ptr_ty, value);
            }
            lex::instr::Instr::BinOp(bin_op) => {
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.build_value(&bin_op.left);
                let right = self.build_value(&bin_op.right);

                let tmp = match bin_op.op() {
                    lex::BinOpType::Add => self.builder.ins().iadd(left, right),
                    lex::BinOpType::Sub => self.builder.ins().isub(left, right),
                    lex::BinOpType::Mul => self.builder.ins().imul(left, right),
                    lex::BinOpType::Div => self.builder.ins().sdiv(left, right),
                    lex::BinOpType::Mod => self.builder.ins().srem(left, right),
                    lex::BinOpType::Pow => {
                        // TODO: exponentiation by squaring
                        // https://stackoverflow.com/a/101613
                        self.builder.ins().imul(left, right)
                    }
                };

                // FIXME: hardcoded type
                let ptr_ty = self.module.target_config().pointer_type();
                self.store_value(&bin_op.name, ptr_ty, tmp);
            }
            lex::instr::Instr::FnCall(fn_call) => {
                let mut args = Vec::new();
                for arg in fn_call.args.iter() {
                    args.push(self.build_value(arg));
                }

                let results = self.call(&fn_call.callee, &args);
                if let &[tmp] = results {
                    // FIXME: hardcoded type
                    let ptr_ty = self.module.target_config().pointer_type();
                    self.store_value(&fn_call.name, ptr_ty, tmp);
                }
            }
            lex::instr::Instr::FnReturn(fn_return) => {
                if fn_return.value.is_none() {
                    self.builder.ins().return_(&[]);
                } else {
                    let value = self.build_value(&fn_return);
                    self.builder.ins().return_(&[value]);
                }
            }
            lex::instr::Instr::BodyReturn(body_return) => {
                let value = self.build_value(&body_return);
                self.last_result.clear();
                self.last_result.push(value);
            }
        }
    }

    pub fn call(&mut self, func_name: &str, args: &[Value]) -> &[Value] {
        let Some(FuncOrDataId::Func(func_id)) = self.module.get_name(func_name) else {
            // TODO: better error handling
            panic!("Function {} not found", func_name);
        };

        let func = &mut self.builder.func;
        let func_ref = self.module.declare_func_in_func(func_id, func);

        let instr = self.builder.ins().call(func_ref, args);
        self.builder.inst_results(instr)
    }

    pub fn finalize(mut self) {
        // Sealing the block means that everything before it is done and won't change. Creanelift's
        // documentation recomends sealing each block as soon as possible, but since we're doing a
        // lot of back patching in the function, sealing all the blocks at the end of the function
        // is the only way to go. Maybe the lex typing could be changed to provide all the
        // information we need, like variables names, types and so on, so we can avoid this
        self.builder.seal_all_blocks();

        self.builder.finalize();
    }
}
