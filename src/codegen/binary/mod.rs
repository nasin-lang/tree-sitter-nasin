mod func;
mod type_gen;
mod variable_ref;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;

use cranelift_codegen::ir::{
    types, AbiParam, Function, InstBuilder, MemFlags, TrapCode, UserFuncName,
};
use cranelift_codegen::{isa, settings, Context};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use self::func::FnCodegen;
use self::type_gen::TypeGen;
use self::variable_ref::VariableRef;
use super::traits::Codegen;
use crate::proto::lex;

pub struct BinaryCodegen {
    pub module: ObjectModule,
    last_func_num: u32,
    data_list: Vec<lex::DataDecl>,
    symbols_ref: HashMap<String, VariableRef>,
}

impl BinaryCodegen {
    pub fn new<V: Into<Vec<u8>>>(triple: Triple, name: V) -> Self {
        let settings_builder = settings::builder();
        let flags = settings::Flags::new(settings_builder);
        let isa_target = isa::lookup(triple).unwrap().finish(flags).unwrap();

        let module = ObjectModule::new(
            ObjectBuilder::new(isa_target, name, default_libcall_names()).unwrap(),
        );

        let mut this = BinaryCodegen {
            module,
            last_func_num: 0,
            data_list: Vec::new(),
            symbols_ref: HashMap::new(),
        };

        // Declare libc functions
        // TODO: detect which libc functions are needed and declare only those
        // void exit(int status);
        {
            let mut sig = this.module.make_signature();
            sig.params.push(AbiParam::new(types::I32));
            let func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
            let func_id = this
                .module
                .declare_function("exit", Linkage::Import, &func.signature)
                .unwrap();

            let fn_type = lex::FnType {
                args: vec![lex::Type {
                    r#type: Some(lex::r#type::Type::Primitive(lex::PrimType::I32.into())),
                }],
                ret: vec![],
            };

            this.symbols_ref.insert(
                "exit".to_string(),
                VariableRef::ExternalFunc(func_id, func, fn_type),
            );
        }

        this
    }

    fn create_entry_function(&mut self, ctx: &mut Context) {
        let sig = self.module.make_signature();
        let mut func = Function::with_name_signature(UserFuncName::user(2, 0), sig);
        let func_id = self
            .module
            .declare_function("_start", Linkage::Export, &func.signature)
            .unwrap();

        let mut func_ctx = FunctionBuilderContext::new();
        let mut fn_codegen = FnCodegen::new(
            &mut self.module,
            &mut func,
            &mut func_ctx,
            self.symbols_ref.clone(),
        );
        fn_codegen.create_entry_block(&[], &[]);

        // Global variables need to be implemented inside a function
        for data_decl in self.data_list.iter() {
            for instr in data_decl.body.iter() {
                if let Some(instr) = instr.instr.as_ref() {
                    fn_codegen.instr(instr);
                }
            }

            let result = *fn_codegen.last_result.first().unwrap();

            if let Some(var_ref) = self.symbols_ref.get(&data_decl.name) {
                let data_ptr = var_ref.get_ptr(&mut fn_codegen.module, &mut fn_codegen.builder);

                fn_codegen
                    .builder
                    .ins()
                    .store(MemFlags::new(), result, data_ptr, 0);
            }

            fn_codegen
                .variables
                // FIXME: check type for what kind of VariableRef to use
                .insert(data_decl.name.clone(), VariableRef::ImmPrimitive(result));
        }

        // We're using the return of main as exit status only because strings and printing are
        // not implemented and I need some way to check if the program is working. When printing
        // is implemented, this will be changed to a fixed success status, as it should be. In
        // the future, main will evaluate in a Action monad that will be run in the entry
        // function
        let exit_code = fn_codegen
            .variables
            .get("main")
            .expect("main not found")
            .get_value(&mut fn_codegen.module, &mut fn_codegen.builder);

        fn_codegen.call("exit", &[exit_code]);
        // This code is guaranteed to be unreachable, but Cranelift requires some block ending
        // instruction anyways, and `trap` generated less assembly instructions then `return`
        fn_codegen
            .builder
            .ins()
            .trap(TrapCode::UnreachableCodeReached);

        fn_codegen.finalize();

        println!("<_start>: {}\n", &func);

        ctx.func = func;
        self.module.define_function(func_id, ctx).unwrap();
    }
}

impl Codegen for BinaryCodegen {
    fn declare_function(&mut self, decl: &lex::FnDecl) {
        let mut sig = self.module.make_signature();

        for arg_ty in decl.r#type.args.iter() {
            sig.params
                .push(AbiParam::new(self.module.get_type(&arg_ty)));
        }
        for ret_ty in decl.r#type.ret.iter() {
            sig.returns
                .push(AbiParam::new(self.module.get_type(ret_ty)));
        }

        let user_func_name = {
            // namespace is 1 because 0 is being used for reserved functions. This number is
            // completely arbitary and may be changed to be more involved in the future
            UserFuncName::user(1, {
                let num = self.last_func_num;
                self.last_func_num += 1;
                num
            })
        };

        let func = Function::with_name_signature(user_func_name, sig);

        let func_id = self
            .module
            .declare_function(&decl.name, Linkage::Export, &func.signature)
            .unwrap();

        self.symbols_ref.insert(
            decl.name.clone(),
            VariableRef::GlobalFunc(func_id, func, decl.r#type.clone()),
        );
    }

    fn build_function(&mut self, decl: &lex::FnDecl) {
        let symbols = self.symbols_ref.clone();

        let Some(VariableRef::GlobalFunc(_, func, _)) = self.symbols_ref.get_mut(&decl.name) else {
            panic!("Function {} not declared", decl.name);
        };

        let mut func_ctx = FunctionBuilderContext::new();

        FnCodegen::build(&mut self.module, func, &mut func_ctx, symbols, &decl);
    }

    fn declare_data(&mut self, decl: &lex::DataDecl) {
        if decl.name == "main" {
            self.data_list.push(decl.clone());
            return;
        }

        let ty = self.module.get_type(&decl.r#type);

        let data_id = self
            .module
            .declare_data(&decl.name, Linkage::Local, true, false)
            .unwrap();

        let mut desc = DataDescription::new();
        desc.define_zeroinit(ty.bytes() as usize);

        self.module.define_data(data_id, &desc).unwrap();

        self.data_list.push(decl.clone());
        self.symbols_ref.insert(
            decl.name.clone(),
            VariableRef::GlobalPrimitive(data_id, decl.r#type.clone()),
        );

        println!(
            "<{}> = global_value (id: {}, size: {})\n",
            decl.name,
            data_id.as_u32(),
            desc.init.size()
        );
    }

    fn write_to_file(mut self, file: &str) {
        let mut obj_ctx = self.module.make_context();

        self.create_entry_function(&mut obj_ctx);

        // Define all functions in the module
        for var_ref in self.symbols_ref.into_values() {
            let VariableRef::GlobalFunc(func_id, func, _) = var_ref else {
                continue;
            };

            obj_ctx.func = func;
            self.module.define_function(func_id, &mut obj_ctx).unwrap();
        }

        let obj_product = self.module.finish();

        // FIXME: get file name from some kind of configuration
        let obj_path = env::temp_dir().join(format!("{}.o", file));
        let out_file = File::create(&obj_path).expect("Failed to create object file");

        obj_product
            .object
            .write_stream(BufWriter::new(out_file))
            .unwrap();

        std::process::Command::new("cc")
            .arg("-nostartfiles")
            .arg("-o")
            .arg(file)
            .arg(&obj_path)
            .status()
            .expect("failed to link object file");
    }
}
