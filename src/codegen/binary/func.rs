use std::collections::HashMap;

use cranelift_codegen::ir::{types, Block, Function, InstBuilder, Value};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::Module;
use cranelift_object::ObjectModule;
use itertools::izip;

use super::{type_gen::TypeGen, variable_ref::VariableRef};
use crate::proto::lex;

pub struct FnCodegen<'a> {
    pub module: &'a mut ObjectModule,
    pub builder: FunctionBuilder<'a>,
    // Store the last values of a BodyResult, so they can be accessed later when managing the
    // control flow, manly for `if` and when building data initialization
    pub last_result: Vec<Value>,
    pub variables: HashMap<String, VariableRef>,
}

impl<'a> FnCodegen<'a> {
    pub fn new(
        module: &'a mut ObjectModule,
        func: &'a mut Function,
        func_ctx: &'a mut FunctionBuilderContext,
        symbols: HashMap<String, VariableRef>,
    ) -> Self {
        FnCodegen {
            module,
            variables: symbols,
            last_result: Vec::new(),
            builder: FunctionBuilder::new(func, func_ctx),
        }
    }

    pub fn build(
        module: &'a mut ObjectModule,
        func: &'a mut Function,
        func_ctx: &'a mut FunctionBuilderContext,
        symbols: HashMap<String, VariableRef>,
        decl: &lex::FnDecl,
    ) {
        let mut this = Self::new(module, func, func_ctx, symbols);

        this.create_entry_block(&decl.args, &decl.r#type.args);

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
                // maybe this parsing should be handled by the lexer
                self.builder
                    .ins()
                    // FIXME: hardcoded type
                    .iconst(types::I32, num.parse::<i64>().unwrap())
            }
            Some(lex::value::Value::Ident(ident)) => {
                let var_ref = self
                    .variables
                    .get(ident)
                    .expect(&format!("Variable {} not found", ident));

                var_ref.get_value(&mut self.module, &mut self.builder)
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

    pub fn create_entry_block(&mut self, args: &[String], args_ty: &[lex::Type]) -> Block {
        let entry_block = self.builder.create_block();
        self.builder
            .append_block_params_for_function_params(entry_block);

        let entry_block_params = self.builder.block_params(entry_block).to_vec();
        for (arg, arg_ty, block_param) in
            izip!(args.iter(), args_ty.iter(), entry_block_params.into_iter())
        {
            let ty = self.module.get_type(arg_ty);
            self.store_value(&arg, ty, block_param);
        }

        self.builder.switch_to_block(entry_block);

        entry_block
    }

    pub fn instr(&mut self, instr: &lex::instr::Instr) {
        match instr {
            lex::instr::Instr::Assign(assign) => {
                let ty = self.module.get_type(&assign.r#type);
                let value = self.build_value(&assign.value);
                self.store_value(&assign.name, ty, value);
            }
            lex::instr::Instr::BinOp(bin_op) => {
                let ty = self.module.get_type(&bin_op.r#type);
                // Different instructions for different types, might want to use some kind of
                // abstraction for this
                let left = self.build_value(&bin_op.left);
                let right = self.build_value(&bin_op.right);

                let tmp = match ty {
                    // FIXME: unsigned types
                    types::I8 | types::I16 | types::I32 | types::I64 => match bin_op.op() {
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
                    },
                    types::F32 | types::F64 => match bin_op.op() {
                        lex::BinOpType::Add => self.builder.ins().fadd(left, right),
                        lex::BinOpType::Sub => self.builder.ins().fsub(left, right),
                        lex::BinOpType::Mul => self.builder.ins().fmul(left, right),
                        lex::BinOpType::Div => self.builder.ins().fdiv(left, right),
                        lex::BinOpType::Mod => {
                            panic!("Modulo is not defined for floating point numbers")
                        }
                        lex::BinOpType::Pow => {
                            todo!()
                        }
                    },
                    _ => {
                        panic!("Type {} not supported", ty);
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
                    let ty = self.module.get_type(&fn_call.r#type);
                    self.store_value(&fn_call.name, ty, tmp);
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
        let var_ref = self
            .variables
            .get(func_name)
            .expect(&format!("Function {} not found", func_name));

        let func_ref = var_ref.get_func_ref(&mut self.module, &mut self.builder);

        let instr = self.builder.ins().call(func_ref, args);
        self.builder.inst_results(instr)
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
}
