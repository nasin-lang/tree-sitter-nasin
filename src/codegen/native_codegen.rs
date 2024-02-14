use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::iter::zip;

use cranelift_codegen::entity::EntityRef;
use cranelift_codegen::ir::{types, AbiParam, Function, InstBuilder, Signature, UserFuncName};
use cranelift_codegen::{isa, settings};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{default_libcall_names, FuncId, FuncOrDataId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::{PointerWidth, Triple};

use crate::proto::lex;

struct FnData<'a> {
    decl: &'a lex::FnDecl,
    func_id: FuncId,
    func: Function,
}

// This is really messy code, I'll need to refactor this later, but I'm not down to fight the borrow
// checker right now
pub fn compile_program(program: &lex::Module) {
    let shared_builder = settings::builder();
    let shared_flags = settings::Flags::new(shared_builder);
    // TODO: get the target from some kind of configuration
    let isa_target = isa::lookup(Triple::host())
        .unwrap()
        .finish(shared_flags)
        .unwrap();

    let isize_ty = match isa_target.triple().pointer_width() {
        Ok(PointerWidth::U16) => types::I16,
        Ok(PointerWidth::U32) => types::I32,
        Ok(PointerWidth::U64) => types::I64,
        _ => panic!("Unsupported pointer width"),
    };

    let mut obj_module = ObjectModule::new(
        ObjectBuilder::new(isa_target, program.name.clone(), default_libcall_names()).unwrap(),
    );

    // Declare libc functions
    // TODO: detect which libc functions are needed and declare only those
    // void exit(int status);
    {
        let mut sig = obj_module.make_signature();
        sig.params.push(AbiParam::new(isize_ty));
        let func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
        obj_module
            .declare_function("exit", Linkage::Import, &func.signature)
            .unwrap();
    }

    let mut fn_decls = program
        .body
        .iter()
        .filter_map(|instr| match instr.instr.as_ref() {
            Some(lex::instr::Instr::FnDecl(fn_decl)) => Some(fn_decl),
            _ => None,
        })
        .collect::<Vec<_>>();

    let entry_fn = lex::FnDecl {
        name: "_start".to_string(),
        args: Vec::new(),
        body: vec![
            lex::Instr {
                instr: Some(lex::instr::Instr::FnCall(lex::FnCall {
                    name: "v0".to_string(),
                    callee: "main".to_string(),
                    args: Vec::new(),
                })),
            },
            lex::Instr {
                instr: Some(lex::instr::Instr::FnCall(lex::FnCall {
                    name: "v1".to_string(),
                    callee: "exit".to_string(),
                    args: vec![lex::Value {
                        value: Some(lex::value::Value::Ident("v0".to_string())),
                    }],
                })),
            },
            lex::Instr {
                instr: Some(lex::instr::Instr::FnReturn(lex::FnReturn {
                    value: lex::Value { value: None },
                })),
            },
        ],
    };
    fn_decls.push(&entry_fn);

    let mut fn_names = Vec::<FnData>::new();

    for (i, fn_decl) in fn_decls.iter().enumerate() {
        let mut sig = obj_module.make_signature();

        // Type checker is not implemented, so type is hardcoded for now
        if fn_decl.name != "_start" {
            sig.returns.push(AbiParam::new(isize_ty));
        }
        for _args in fn_decl.args.iter() {
            sig.params.push(AbiParam::new(isize_ty));
        }

        let func = Function::with_name_signature(UserFuncName::user(1, i as u32), sig);

        let func_id = obj_module
            .declare_function(&fn_decl.name, Linkage::Export, &func.signature)
            .unwrap();

        fn_names.push(FnData {
            decl: fn_decl,
            func,
            func_id,
        });
    }

    let mut fn_ctx = FunctionBuilderContext::new();

    for fn_data in fn_names.iter_mut() {
        let mut fn_builder = FunctionBuilder::new(&mut fn_data.func, &mut fn_ctx);

        let entry_block = fn_builder.create_block();
        fn_builder.append_block_params_for_function_params(entry_block);
        fn_builder.switch_to_block(entry_block);

        let mut names = HashMap::<String, Variable>::new();

        // macros instead of functions because the borrow checker hates me
        macro_rules! create_var {
            ($name:expr, $ty:expr, $value:expr) => {{
                let key: &str = $name;
                names.insert(key.to_string(), Variable::new(names.len()));
                let var = names.get(key).unwrap();
                fn_builder.declare_var(*var, $ty);
                fn_builder.def_var(*var, $value);
            }};
        }

        macro_rules! use_value {
            ($lex_value:expr) => {{
                let lex_value: &lex::Value = $lex_value;
                match lex_value.value.as_ref() {
                    Some(lex::value::Value::Num(num)) => {
                        // maybe this parsing should be handled by the lexer
                        fn_builder
                            .ins()
                            .iconst(isize_ty, num.value.parse::<i64>().unwrap())
                    }
                    Some(lex::value::Value::Ident(ident)) => {
                        let var = names
                            .get(ident)
                            .expect(&format!("Variable {} not found", ident));
                        fn_builder.use_var(*var)
                    }
                    None => {
                        unreachable!();
                    }
                }
            }};
        }

        let entry_block_params = fn_builder.block_params(entry_block).to_vec();
        for (arg, block_param) in zip(fn_data.decl.args.iter(), entry_block_params.iter()) {
            // FIXME: hardcoded type
            create_var!(&arg, isize_ty, *block_param);
        }

        for instr in fn_data.decl.body.iter() {
            match instr.instr.as_ref() {
                Some(lex::instr::Instr::Assign(assign)) => {
                    let value = use_value!(&assign.value);
                    // FIXME: hardcoded type
                    create_var!(&assign.name, isize_ty, value);
                }
                Some(lex::instr::Instr::BinOp(bin_op)) => {
                    // Different instructions for different types, might want to use some kind of
                    // abstraction for this
                    let left = use_value!(&bin_op.left);
                    let right = use_value!(&bin_op.right);

                    let tmp = match bin_op.op() {
                        lex::BinOpType::Add => fn_builder.ins().iadd(left, right),
                        lex::BinOpType::Sub => fn_builder.ins().isub(left, right),
                        lex::BinOpType::Mul => fn_builder.ins().imul(left, right),
                        lex::BinOpType::Div => fn_builder.ins().sdiv(left, right),
                        lex::BinOpType::Mod => fn_builder.ins().srem(left, right),
                        lex::BinOpType::Pow => {
                            // TODO: exponentiation by squaring
                            // https://stackoverflow.com/a/101613
                            fn_builder.ins().imul(left, right)
                        }
                    };

                    // FIXME: hardcoded type
                    create_var!(&bin_op.name, isize_ty, tmp);
                }
                Some(lex::instr::Instr::FnCall(fn_call)) => {
                    let Some(FuncOrDataId::Func(callee_id)) = obj_module.get_name(&fn_call.callee) else {
                        // TODO: better error handling
                        panic!("Function {} not found", fn_call.name);
                    };

                    let callee_ref = obj_module.declare_func_in_func(callee_id, fn_builder.func);

                    let mut args = Vec::new();
                    for arg in fn_call.args.iter() {
                        args.push(use_value!(arg));
                    }

                    let inst = fn_builder.ins().call(callee_ref, &args);
                    if let &[tmp] = fn_builder.inst_results(inst) {
                        // FIXME: hardcoded type
                        create_var!(&fn_call.name, isize_ty, tmp);
                    }
                }
                Some(lex::instr::Instr::FnReturn(fn_return)) => {
                    if fn_return.value.value.is_none() {
                        fn_builder.ins().return_(&[]);
                    } else {
                        let value = use_value!(&fn_return.value);
                        fn_builder.ins().return_(&[value]);
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }

        // Sealing the block means that everything before it is done and won't change. Creanelift's
        // documentation recomends sealing each block as soon as possible, but since we're doing a
        // lot of back patching in the function, sealing all the blocks at the end of the function
        // is the only way to go. Maybe the lex typing could be changed to provide all the
        // information we need, like variables names, types and so on, so we can avoid this
        fn_builder.seal_all_blocks();

        println!("<{}>: {}", &fn_data.decl.name, &fn_builder.func);

        fn_builder.finalize();
    }

    let mut obj_ctx = obj_module.make_context();

    // Define all functions in the module
    for fn_data in fn_names.into_iter() {
        obj_ctx.func = fn_data.func;
        obj_module
            .define_function(fn_data.func_id, &mut obj_ctx)
            .unwrap();
    }

    let obj_product = obj_module.finish();

    // FIXME: get file name from some kind of configuration
    let obj_path = env::temp_dir().join(format!("{}.o", program.name));
    let out_file = File::create(&obj_path).expect("Failed to create object file");

    obj_product
        .object
        .write_stream(BufWriter::new(out_file))
        .unwrap();

    std::process::Command::new("clang")
        .arg("-nostartfiles")
        .arg("-o")
        .arg(program.name.clone())
        .arg(&obj_path)
        .status()
        .expect("failed to link object file");

    println!("Compiled program to {}", program.name);
}
