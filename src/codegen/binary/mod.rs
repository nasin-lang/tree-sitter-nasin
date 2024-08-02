mod func;
mod globals;
mod types;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use cranelift_shim::{self as cl, InstBuilder, Module};
use itertools::izip;
use target_lexicon::Triple;

use self::func::FuncCodegen;
use self::globals::Globals;
use super::traits::Codegen;
use crate::utils;
use crate::{bytecode as b, config};

utils::enumerate!(pub FuncNS: u32 {
    User = 0,
    SystemFunc = 1,
});

utils::enumerate!(pub SystemFunc: u32 {
    Start = 0,
    Exit = 1,
});

pub struct FuncBinding {
    pub is_extern: bool,
    pub symbol_name: String,
    pub func_id: cl::FuncId,
    pub params: Vec<b::Param>,
    pub ret: b::Type,
}

pub struct BinaryCodegen {
    pub module: cl::ObjectModule,
    module_ctx: cl::Context,
    typedefs: Vec<b::TypeDef>,
    globals: Globals,
    funcs: Vec<FuncBinding>,
    declared_funcs: Vec<cl::Function>,
    dump_clif: bool,
}
impl BinaryCodegen {
    pub fn new(triple: Triple, cfg: &config::BuildConfig) -> Self {
        let settings_builder = cl::settings::builder();
        let flags = cl::settings::Flags::new(settings_builder);
        let isa_target = cl::isa::lookup(triple).unwrap().finish(flags).unwrap();

        let module = cl::ObjectModule::new(
            cl::ObjectBuilder::new(isa_target, "main", cl::default_libcall_names())
                .unwrap(),
        );

        let module_ctx = module.make_context();

        BinaryCodegen {
            module,
            module_ctx,
            typedefs: Vec::new(),
            globals: Globals::new(),
            funcs: Vec::new(),
            declared_funcs: Vec::new(),
            dump_clif: cfg.dump_clif,
        }
    }
    fn build_entry(&mut self) {
        let mut exit_sig = self.module.make_signature();
        exit_sig.params.push(cl::AbiParam::new(cl::types::I32));
        let exit_func = cl::Function::with_name_signature(
            cl::UserFuncName::user(FuncNS::SystemFunc.into(), SystemFunc::Exit.into()),
            exit_sig,
        );
        let exit_func_id = self
            .module
            .declare_function("exit", cl::Linkage::Import, &exit_func.signature)
            .unwrap();

        let mut func = cl::Function::with_name_signature(
            cl::UserFuncName::user(FuncNS::SystemFunc.into(), SystemFunc::Start.into()),
            self.module.make_signature(),
        );
        let func_id = self
            .module
            .declare_function("_start", cl::Linkage::Export, &func.signature)
            .unwrap();

        utils::replace_with(self, |mut this| {
            let mut func_ctx = cl::FunctionBuilderContext::new();
            let func_builder = cl::FunctionBuilder::new(&mut func, &mut func_ctx);
            let mut codegen = FuncCodegen::new(
                Some(func_builder),
                this.module,
                this.globals,
                this.funcs,
                this.typedefs.clone(),
            );
            codegen.create_initial_block(&[]);

            let mut entry = None;

            for global in codegen.globals.globals.clone() {
                if global.entry {
                    entry = Some(global);
                    continue;
                }
                let Some(init) = &global.init else {
                    continue;
                };
                for instr in init {
                    codegen.add_instr(instr);
                }
                let res = codegen.stack.pop();
                codegen.store_global(res, &global);
            }

            let entry = entry.expect("entrypoint should be defined");
            let exit_code = if let Some(init) = &entry.init {
                for instr in init {
                    codegen.add_instr(instr);
                }
                codegen
                    .stack
                    .pop()
                    .add_to_func(&mut codegen.module, codegen.func.as_mut().unwrap())
            } else {
                codegen
                    .func
                    .as_mut()
                    .unwrap()
                    .ins()
                    .iconst(cl::types::I32, 0)
            };
            codegen.call(exit_func_id, &[exit_code]);

            (this.module, this.globals, this.funcs) = codegen.return_never();
            this
        });

        self.funcs.push(FuncBinding {
            symbol_name: "_start".to_string(),
            is_extern: false,
            func_id,
            params: vec![],
            ret: b::Type::unknown(),
        });
        self.declared_funcs.push(func);
    }
}
impl Codegen<'_> for BinaryCodegen {
    fn declare_typedef(&mut self, _idx: usize, decl: &b::TypeDef) {
        self.typedefs.push(decl.clone());
    }

    fn declare_function(&mut self, idx: usize, decl: &b::Func) {
        let mut sig = self.module.make_signature();

        for param in &decl.params {
            sig.params.push(cl::AbiParam::new(types::get_type(
                &param.ty,
                &self.typedefs,
                &self.module,
            )));
        }
        sig.returns.push(cl::AbiParam::new(types::get_type(
            &decl.ret,
            &self.typedefs,
            &self.module,
        )));

        let user_func_name = cl::UserFuncName::user(FuncNS::User.into(), idx as u32);

        let func = cl::Function::with_name_signature(user_func_name, sig);

        let symbol_name = if let Some(b::Extern { name }) = &decl.extn {
            name.clone()
        } else {
            // TODO: improve name mangling
            format!("$func{idx}")
        };

        let linkage = if decl.extn.is_some() {
            if decl.body.is_empty() {
                cl::Linkage::Import
            } else {
                cl::Linkage::Export
            }
        } else {
            cl::Linkage::Local
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

    fn declare_global(&mut self, idx: usize, decl: &b::Global) {
        self.globals
            .insert_global(idx, decl, &mut self.module, self.typedefs.clone());
    }

    fn build_function(&mut self, idx: usize, decl: &b::Func) {
        utils::replace_with(self, |mut this| {
            let mut func_ctx = cl::FunctionBuilderContext::new();
            let func =
                cl::FunctionBuilder::new(&mut this.declared_funcs[idx], &mut func_ctx);
            let mut codegen = FuncCodegen::new(
                Some(func),
                this.module,
                this.globals,
                this.funcs,
                this.typedefs.clone(),
            );
            codegen.create_initial_block(&decl.params);

            for instr in &decl.body {
                codegen.add_instr(instr);
            }

            (this.module, this.globals, this.funcs) = codegen.return_value();
            this
        })
    }

    fn write_to_file(mut self, file: &std::path::Path) {
        self.build_entry();

        if self.globals.data.len() > 0 && self.dump_clif {
            for (data_id, desc) in &self.globals.data {
                let data_init = &desc.init;
                print!("{} = data {}", data_id, data_init.size());
                if let cl::Init::Bytes { contents } = data_init {
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
        let status = std::process::Command::new("ld")
            .arg("-dynamic-linker")
            .arg(dyn_linker)
            .arg("-o")
            .arg(file)
            .arg(&obj_path)
            .arg("-lc")
            .status()
            .expect("failed to link object file");

        if !status.success() {
            panic!("failed to link object file");
        }
    }
}
