mod func;
mod variable_ref;

use std::env;
use std::fs::File;
use std::io::BufWriter;

use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, MemFlags, TrapCode, UserFuncName};
use cranelift_codegen::{isa, settings, Context};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use func::FnCodegen;
use target_lexicon::Triple;

use super::traits::Codegen;
use crate::proto::lex;

#[derive(Debug)]
struct FnInfo {
    name: String,
    func_id: FuncId,
    func: Function,
}

#[derive(Debug)]
struct DataInfo {
    decl: lex::DataDecl,
    data_id: DataId,
}

pub struct BinaryCodegen {
    pub module: ObjectModule,
    func_list: Vec<FnInfo>,
    data_list: Vec<DataInfo>,
}

impl BinaryCodegen {
    pub fn new<V: Into<Vec<u8>>>(triple: Triple, name: V) -> Self {
        let settings_builder = settings::builder();
        // What is this?
        let flags = settings::Flags::new(settings_builder);
        let isa_target = isa::lookup(triple).unwrap().finish(flags).unwrap();

        let module = ObjectModule::new(
            ObjectBuilder::new(isa_target, name, default_libcall_names()).unwrap(),
        );

        let mut this = BinaryCodegen {
            module,
            func_list: Vec::new(),
            data_list: Vec::new(),
        };

        // Declare libc functions
        // TODO: detect which libc functions are needed and declare only those
        // void exit(int status);
        {
            let mut sig = this.module.make_signature();
            sig.params
                .push(AbiParam::new(this.module.target_config().pointer_type()));
            let func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
            this.module
                .declare_function("exit", Linkage::Import, &func.signature)
                .unwrap();
        }

        this
    }

    fn create_entry_function(&mut self, ctx: &mut Context) {
        let ptr_ty = self.module.target_config().pointer_type();

        // Define a initialization function for each

        let sig = self.module.make_signature();
        let mut func = Function::with_name_signature(UserFuncName::user(2, 0), sig);
        let func_id = self
            .module
            .declare_function("_start", Linkage::Export, &func.signature)
            .unwrap();

        let mut func_ctx = FunctionBuilderContext::new();
        let mut fn_codegen = FnCodegen::new(&mut self.module, &mut func, &mut func_ctx);
        fn_codegen.create_entry_block(&[]);

        let mut exit_code = None;

        // Global variables need to be implemented inside a function
        for data_info in self.data_list.iter() {
            let data_id = data_info.data_id;
            let data_decl = &data_info.decl;

            for instr in data_decl.body.iter() {
                if let Some(instr) = instr.instr.as_ref() {
                    fn_codegen.instr(instr);
                }
            }

            let result = *fn_codegen.last_result.first().unwrap();

            // We're using the return of main as exit status only because strings and printing are
            // not implemented and I need some way to check if the program is working. When printing
            // is implemented, this will be changed to a fixed success status, as it should be. In
            // the future, main will evaluate in a Action monad that will be run in the entry
            // function
            if data_decl.name == "main" {
                exit_code = Some(result);
            } else {
                let data = fn_codegen
                    .module
                    .declare_data_in_func(data_id, &mut fn_codegen.builder.func);

                // FIXME: hardcoded type
                let data_ptr = fn_codegen.builder.ins().global_value(ptr_ty, data);

                fn_codegen
                    .builder
                    .ins()
                    .store(MemFlags::new(), result, data_ptr, 0);
            }
        }

        let exit_code = exit_code.unwrap_or_else(|| fn_codegen.builder.ins().iconst(ptr_ty, 0));
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

        // Type checker is not implemented, so type is hardcoded for now
        let ptr_ty = self.module.target_config().pointer_type();
        sig.returns.push(AbiParam::new(ptr_ty));
        for _args in decl.args.iter() {
            sig.params.push(AbiParam::new(ptr_ty));
        }

        let user_func_name = {
            // namespace is 1 because 0 is being used for reserved functions. This number is
            // completely arbitary and may be changed to be more involved in the future
            UserFuncName::user(1, self.func_list.len() as u32)
        };

        let func = Function::with_name_signature(user_func_name, sig);

        let func_id = self
            .module
            .declare_function(&decl.name, Linkage::Export, &func.signature)
            .unwrap();

        self.func_list.push(FnInfo {
            name: decl.name.clone(),
            func_id,
            func,
        });
    }

    fn build_function(&mut self, decl: &lex::FnDecl) {
        let fn_info = self
            .func_list
            .iter_mut()
            .find(|fn_info| fn_info.name == decl.name)
            .expect("Function not declared");

        let mut func_ctx = FunctionBuilderContext::new();

        FnCodegen::build(&mut self.module, &mut fn_info.func, &mut func_ctx, &decl);
    }

    fn declare_data(&mut self, decl: &lex::DataDecl) {
        // FIXME: hardcoded type
        let len = self.module.target_config().pointer_type().bytes() as usize;

        let data_id = self
            .module
            .declare_data(&decl.name, Linkage::Local, true, false)
            .unwrap();

        let mut desc = DataDescription::new();
        desc.define_zeroinit(len);

        self.module.define_data(data_id, &desc).unwrap();

        self.data_list.push(DataInfo {
            decl: decl.clone(),
            data_id,
        });
    }

    fn write_to_file(mut self, file: &str) {
        let mut obj_ctx = self.module.make_context();

        self.create_entry_function(&mut obj_ctx);

        // Define all functions in the module
        for fn_info in self.func_list.into_iter() {
            obj_ctx.func = fn_info.func;
            self.module
                .define_function(fn_info.func_id, &mut obj_ctx)
                .unwrap();
        }

        let obj_product = self.module.finish();

        // FIXME: get file name from some kind of configuration
        let obj_path = env::temp_dir().join(format!("{}.o", file));
        let out_file = File::create(&obj_path).expect("Failed to create object file");

        obj_product
            .object
            .write_stream(BufWriter::new(out_file))
            .unwrap();

        std::process::Command::new("clang")
            .arg("-nostartfiles")
            .arg("-o")
            .arg(file)
            .arg(&obj_path)
            .status()
            .expect("failed to link object file");
    }
}
