mod func;
mod type_gen;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use cranelift_codegen::ir::{types, AbiParam, Function, InstBuilder, TrapCode, UserFuncName};
use cranelift_codegen::{isa, settings, Context};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use itertools::izip;
use target_lexicon::Triple;

use self::func::{FnCodegen, FuncBinding, GlobalBinding};
use self::type_gen::TypeGen;
use super::traits::Codegen;
use crate::config::BuildConfig;
use crate::mir;

const EXIT_FUNC_IDX: usize = 0;

pub struct BinaryCodegen {
    module: ObjectModule,
    module_ctx: Context,
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
            globals: Vec::new(),
            funcs: Vec::new(),
            declared_funcs: Vec::new(),
            dump_clif: cfg.dump_clif,
        }
    }
}

impl Codegen for BinaryCodegen {
    fn declare_function(&mut self, idx: usize, decl: &mir::Func) {
        let mut sig = self.module.make_signature();

        for param in &decl.params {
            sig.params
                .push(AbiParam::new(self.module.get_type(&param.ty)));
        }
        for ret in &decl.ret {
            sig.returns.push(AbiParam::new(self.module.get_type(ret)));
        }

        let user_func_name = UserFuncName::user(0, idx as u32);

        let func = Function::with_name_signature(user_func_name, sig);

        let symbol_name = if let Some(mir::Extern { name }) = &decl.extern_ {
            name.clone()
        } else if let Some(mir::Export { name }) = &decl.export {
            name.clone()
        } else {
            format!("$func{}", idx)
        };

        let linkage = if decl.extern_.is_some() {
            Linkage::Import
        } else if decl.export.is_some() {
            Linkage::Export
        } else {
            Linkage::Local
        };

        let func_id = self
            .module
            .declare_function(&symbol_name, linkage, &func.signature)
            .unwrap();

        self.funcs.push(FuncBinding {
            symbol_name,
            is_extern: decl.extern_.is_some(),
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
            &self.globals,
            &self.funcs,
            &decl.params,
            &decl.locals,
        );

        for instr in decl.body.iter() {
            fn_codegen.instr(instr);
        }

        fn_codegen.finalize();
    }

    fn declare_global(&mut self, idx: usize, decl: &mir::Global) {
        let native_ty = self.module.get_type(&decl.ty);
        let size = self.module.get_size(&decl.ty);

        let symbol_name = if let Some(mir::Export { name }) = &decl.export {
            name.clone()
        } else {
            format!("$global{}", idx)
        };

        let linkage = if decl.export.is_some() {
            Linkage::Export
        } else {
            Linkage::Local
        };

        let data_id = self
            .module
            .declare_data(&symbol_name, linkage, true, false)
            .unwrap();

        let mut data_desc = DataDescription::new();
        match &decl.value {
            Some(v) => {
                data_desc.define(self.module.serialize(&decl.ty, v).into());
            }
            _ => {
                data_desc.define_zeroinit(size);
            }
        }

        self.module.define_data(data_id, &data_desc).unwrap();

        self.globals.push(GlobalBinding {
            symbol_name: symbol_name.clone(),
            data_id,
            data_desc,
            ty: decl.ty.clone(),
            native_ty,
        });
    }

    fn build_module_init(&mut self, init: &mir::ModuleInit) {
        let sig = self.module.make_signature();
        let mut func = Function::with_name_signature(UserFuncName::user(2, 0), sig);
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
            &self.globals,
            &self.funcs,
            &[],
            &init.locals,
        );

        for instr in &init.body {
            fn_codegen.instr(instr);
        }

        // In the future, main will evaluate in a Action monad that will be run in the
        // entry function
        let exit_code = fn_codegen.builder.ins().iconst(types::I32, 0);

        let exit_func_ref = fn_codegen.module.declare_func_in_func(
            self.funcs[EXIT_FUNC_IDX].func_id.clone(),
            &mut fn_codegen.builder.func,
        );
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
                let data_init = &global.data_desc.init;
                print!(
                    "<{}> {} = data {}",
                    global.symbol_name,
                    global.data_id,
                    data_init.size()
                );
                match data_init {
                    cranelift_module::Init::Zeros { size } => {
                        println!(" {{{}}}", size);
                    }
                    cranelift_module::Init::Bytes { contents } => {
                        print!(" {{");
                        for (i, byte) in contents.iter().enumerate() {
                            if i != 0 {
                                print!(", ");
                            }
                            print!("{}", byte);
                        }
                        println!("}}");
                    }
                    _ => {}
                }
            }
            println!();
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
        }

        let obj_product = self.module.finish();

        // FIXME: get file name from some kind of configuration
        let obj_path =
            env::temp_dir().join(format!("{}.o", file.to_string_lossy().replace("/", "__")));
        let out_file = File::create(&obj_path).expect("Failed to create object file");

        obj_product
            .object
            .write_stream(BufWriter::new(out_file))
            .unwrap();

        // TODO: windows support
        let cc = [
            "/usr/bin/clang",
            "/usr/local/bin/clang",
            "/usr/bin/gcc",
            "/usr/local/bin/gcc",
        ]
        .into_iter()
        .find(|path| std::fs::metadata(path).is_ok())
        .expect("Could not find clang or gcc");

        std::process::Command::new(cc)
            .arg("-nostartfiles")
            .arg("-o")
            .arg(file)
            .arg(&obj_path)
            .status()
            .expect("failed to link object file");
    }
}
