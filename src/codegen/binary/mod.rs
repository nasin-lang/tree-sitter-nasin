mod func;

use std::env;
use std::fs::File;
use std::io::BufWriter;

use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, TrapCode, UserFuncName};
use cranelift_codegen::{isa, settings, Context};
use cranelift_module::{default_libcall_names, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use func::FnCodegen;
use target_lexicon::Triple;

use super::traits::Codegen;
use crate::proto::lex;

struct FnData {
    name: String,
    func_id: FuncId,
    func: Function,
}

pub struct BinaryCodegen {
    pub module: ObjectModule,
    symbols: Vec<FnData>,
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
            symbols: Vec::new(),
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
        let sig = self.module.make_signature();
        let mut func = Function::with_name_signature(UserFuncName::user(2, 0), sig);
        let func_id = self
            .module
            .declare_function("_start", Linkage::Export, &func.signature)
            .unwrap();

        let mut fn_codegen = FnCodegen::new(&mut self.module, &mut func);
        fn_codegen.create_entry_block(&[]);

        //  We're using a main function only because global variables are not implemented. In the
        //  future, main will be a glocal variable of the entry module that will evaluate in a
        //  Action monad that will be run here in the entry function
        let main_return = { fn_codegen.call("main", &[])[0] };

        // We're using the return of main as exit status only because strings and printing are not
        // implemented and I need some way to check if the program is working. When printing is
        // implemented, this will be changed to a fixed success status, as it should be
        fn_codegen.call("exit", &[main_return]);
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
            UserFuncName::user(1, self.symbols.len() as u32)
        };

        let func = Function::with_name_signature(user_func_name, sig);

        let func_id = self
            .module
            .declare_function(&decl.name, Linkage::Export, &func.signature)
            .unwrap();

        self.symbols.push(FnData {
            name: decl.name.clone(),
            func_id,
            func,
        });
    }

    fn define_function(&mut self, decl: &lex::FnDecl) {
        let fn_data = self
            .symbols
            .iter_mut()
            .find(|fn_data| fn_data.name == decl.name)
            .expect("Function not declared");

        FnCodegen::build(&mut self.module, &mut fn_data.func, &decl);
    }

    fn write_to_file(mut self, file: &str) {
        let mut obj_ctx = self.module.make_context();

        self.create_entry_function(&mut obj_ctx);

        // Define all functions in the module
        for fn_data in self.symbols.into_iter() {
            obj_ctx.func = fn_data.func;
            self.module
                .define_function(fn_data.func_id, &mut obj_ctx)
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
