mod func;
mod type_gen;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use cranelift_codegen::ir::{
    types, AbiParam, Function, InstBuilder, TrapCode, UserFuncName,
};
use cranelift_codegen::{isa, settings, Context};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use itertools::izip;
use target_lexicon::Triple;

use self::func::{FnCodegen, FuncBinding, GlobalBinding};
use self::type_gen::TypeGen;
use super::traits::Codegen;
use crate::config::BuildConfig;
use crate::mir;

const USER_FUNC_NS: u32 = 0;
const INTERNAL_FUNC_NS: u32 = 1;

const START_FUNC_IDX: u32 = 0;
const EXIT_FUNC_IDX: u32 = 1;

pub struct BinaryCodegen {
    module: ObjectModule,
    module_ctx: Context,
    typedefs: Vec<mir::TypeDef>,
    globals: Vec<GlobalBinding>,
    funcs: Vec<FuncBinding>,
    declared_funcs: Vec<Function>,
    dump_clif: bool,
}

impl BinaryCodegen {
    pub fn new<V: Into<Vec<u8>>>(triple: Triple, name: V, cfg: &BuildConfig) -> Self {
        let settings_builder = settings::builder();
        let flags = settings::Flags::new(settings_builder);
        let isa_target = isa::lookup(triple).unwrap().finish(flags).unwrap();

        let module = ObjectModule::new(
            ObjectBuilder::new(isa_target, name, default_libcall_names()).unwrap(),
        );

        let module_ctx = module.make_context();

        BinaryCodegen {
            module,
            module_ctx,
            typedefs: Vec::new(),
            globals: Vec::new(),
            funcs: Vec::new(),
            declared_funcs: Vec::new(),
            dump_clif: cfg.dump_clif,
        }
    }
}

impl Codegen for BinaryCodegen {
    fn declare_typedef(&mut self, _idx: usize, decl: &mir::TypeDef) {
        self.typedefs.push(decl.clone());
    }

    fn declare_function(&mut self, idx: usize, decl: &mir::Func) {
        let mut sig = self.module.make_signature();

        for param in &decl.params {
            sig.params.push(AbiParam::new(
                self.module.get_type(&param.ty, &self.typedefs),
            ));
        }
        for ret in &decl.ret {
            sig.returns
                .push(AbiParam::new(self.module.get_type(ret, &self.typedefs)));
        }

        let user_func_name = UserFuncName::user(USER_FUNC_NS, idx as u32);

        let func = Function::with_name_signature(user_func_name, sig);

        let symbol_name = if let Some(mir::Extern { name }) = &decl.extn {
            name.clone()
        } else if let Some(mir::Export { name }) = &decl.export {
            format!("$func_{name}")
        } else {
            format!("$func{idx}")
        };

        let linkage = if decl.extn.is_some() {
            if decl.body.is_empty() {
                Linkage::Import
            } else {
                Linkage::Export
            }
        } else {
            Linkage::Local
        };

        let func_id = self
            .module
            .declare_function(&symbol_name, linkage, &func.signature)
            .unwrap();

        self.funcs.push(FuncBinding {
            symbol_name,
            is_extern: decl.extn.is_some(),
            func_id,
            params: decl.params.clone(),
            ret: decl.ret.clone(),
        });
        self.declared_funcs.push(func);
    }

    fn build_function(&mut self, idx: usize, decl: &mir::Func) {
        let func_binding = &self.funcs[idx];
        let func = &mut self.declared_funcs[idx];

        let mut func_ctx = FunctionBuilderContext::new();

        let mut fn_codegen = FnCodegen::new(
            &func_binding.symbol_name,
            &mut self.module,
            func,
            &mut func_ctx,
            &self.typedefs,
            &self.globals,
            &self.funcs,
            &decl.params,
            &decl.locals,
        );

        fn_codegen.instrs(&decl.body, true);

        fn_codegen.finalize();
    }

    fn declare_global(&mut self, idx: usize, decl: &mir::Global) {
        let symbol_name = if let Some(mir::Export { name }) = &decl.export {
            format!("$global_{name}")
        } else {
            format!("$global{idx}")
        };

        let (data_id, data) = self.module.create_global_data(
            Some(&symbol_name),
            decl.value.as_ref(),
            &decl.ty,
            &self.typedefs,
        );

        self.globals.push(GlobalBinding {
            symbol_name: symbol_name.clone(),
            data_id,
            data,
            ty: decl.ty.clone(),
            native_ty: self.module.get_type(&decl.ty, &self.typedefs),
        });
    }

    fn build_module_init(&mut self, init: &mir::ModuleInit) {
        let mut exit_sig = self.module.make_signature();
        exit_sig.params.push(AbiParam::new(types::I32));
        let exit_func = Function::with_name_signature(
            UserFuncName::user(INTERNAL_FUNC_NS, EXIT_FUNC_IDX),
            exit_sig,
        );
        let exit_func_id = self
            .module
            .declare_function("exit", Linkage::Import, &exit_func.signature)
            .unwrap();

        let sig = self.module.make_signature();
        let mut func = Function::with_name_signature(
            UserFuncName::user(INTERNAL_FUNC_NS, START_FUNC_IDX),
            sig,
        );
        let func_id = self
            .module
            .declare_function("_start", Linkage::Export, &func.signature)
            .unwrap();

        let mut func_ctx = FunctionBuilderContext::new();
        let mut fn_codegen = FnCodegen::new(
            "_start",
            &mut self.module,
            &mut func,
            &mut func_ctx,
            &self.typedefs,
            &self.globals,
            &self.funcs,
            &[],
            &init.locals,
        );

        fn_codegen.instrs(&init.body, false);

        // In the future, main will evaluate in a Action monad that will be run in the
        // entry function
        let exit_code = fn_codegen.builder.ins().iconst(types::I32, 0);

        let exit_func_ref = fn_codegen
            .module
            .declare_func_in_func(exit_func_id, &mut fn_codegen.builder.func);
        fn_codegen.builder.ins().call(exit_func_ref, &[exit_code]);

        fn_codegen
            .builder
            .ins()
            .trap(TrapCode::UnreachableCodeReached);

        fn_codegen.finalize();

        self.funcs.push(FuncBinding {
            symbol_name: "_start".to_string(),
            is_extern: false,
            func_id,
            params: vec![],
            ret: vec![],
        });
        self.declared_funcs.push(func);
    }

    fn write_to_file(mut self, file: &Path) {
        if self.globals.len() > 0 && self.dump_clif {
            for global in &self.globals {
                let data_init = &global.data.init;
                print!(
                    "<{}> {} = data {}",
                    global.symbol_name,
                    global.data_id,
                    data_init.size()
                );
                if let cranelift_module::Init::Bytes { contents } = data_init {
                    print!(" {{");
                    for (i, byte) in contents.iter().enumerate() {
                        if i != 0 {
                            print!(" ");
                        }
                        print!("{}", byte);
                    }
                    print!("}}");
                }
                println!("\n");
            }
        }

        for (func_binding, func) in izip!(self.funcs, self.declared_funcs) {
            if self.dump_clif {
                println!("<{}> {}", func_binding.symbol_name, func);
            }

            if func_binding.is_extern {
                continue;
            }

            self.module_ctx.func = func;
            self.module
                .define_function(func_binding.func_id, &mut self.module_ctx)
                .unwrap();
            self.module.clear_context(&mut self.module_ctx)
        }

        let obj_product = self.module.finish();

        // FIXME: get file name from some kind of configuration
        let obj_path = env::temp_dir()
            .join(format!("{}.o", file.to_string_lossy().replace("/", "__")));
        let out_file = File::create(&obj_path).expect("Failed to create object file");

        obj_product
            .object
            .write_stream(BufWriter::new(out_file))
            .unwrap();

        let dyn_linker = [
            "/lib/ld64.so.2",
            "/lib/ld64.so.1",
            "/lib64/ld-linux-x86-64.so.2",
            "/lib/ld-linux-x86-64.so.2",
        ]
        .into_iter()
        .find(|path| Path::new(path).is_file())
        .expect("libc.a not found");

        // TODO: windows support
        std::process::Command::new("ld")
            .arg("-dynamic-linker")
            .arg(dyn_linker)
            .arg("-o")
            .arg(file)
            .arg(&obj_path)
            .arg("-lc")
            .status()
            .expect("failed to link object file");
    }
}
