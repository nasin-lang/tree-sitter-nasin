mod func;
mod globals;
mod types;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use cranelift_shim::{self as cl, Module};
use itertools::Itertools;
use target_lexicon::Triple;

use self::func::FuncCodegen;
use self::globals::Globals;
use crate::{bytecode as b, config, utils};

utils::number_enum!(pub FuncNS: u32 {
    User = 0,
    SystemFunc = 1,
});

utils::number_enum!(pub SystemFunc: u32 {
    Start = 0,
    Exit = 1,
});

#[derive(Debug)]
pub struct FuncBinding {
    pub is_extern: bool,
    pub symbol_name: String,
    pub func_id: cl::FuncId,
}

pub struct BinaryCodegen<'a> {
    pub modules: &'a [b::Module],
    pub cfg: &'a config::BuildConfig,
    pub obj_module: cl::ObjectModule,
    module_ctx: cl::Context,
    globals: Globals<'a>,
    funcs: HashMap<(usize, usize), FuncBinding>,
    declared_funcs: HashMap<(usize, usize), cl::Function>,
    next_func_id: u32,
}
impl<'a> BinaryCodegen<'a> {
    pub fn new(modules: &'a [b::Module], cfg: &'a config::BuildConfig) -> Self {
        let triple = Triple::host();

        let settings_builder = cl::settings::builder();
        let flags = cl::settings::Flags::new(settings_builder);
        let isa_target = cl::isa::lookup(triple).unwrap().finish(flags).unwrap();

        let obj_module = cl::ObjectModule::new(
            cl::ObjectBuilder::new(isa_target, "main", cl::default_libcall_names())
                .unwrap(),
        );

        let module_ctx = obj_module.make_context();

        BinaryCodegen {
            modules,
            cfg,
            obj_module,
            module_ctx,
            globals: Globals::new(modules),
            funcs: HashMap::new(),
            declared_funcs: HashMap::new(),
            next_func_id: 0,
        }
    }
}
impl BinaryCodegen<'_> {
    pub fn write(mut self) {
        for mod_idx in 0..self.modules.len() {
            for idx in 0..self.modules[mod_idx].globals.len() {
                self.declare_global(mod_idx, idx);
            }

            for idx in 0..self.modules[mod_idx].funcs.len() {
                self.declare_function(mod_idx, idx);
            }
        }

        for mod_idx in 0..self.modules.len() {
            for idx in 0..self.modules[mod_idx].funcs.len() {
                if self.modules[mod_idx].funcs[idx].body.is_empty() {
                    continue;
                }
                self.build_function(mod_idx, idx);
            }
        }

        self.write_to_file();
    }

    fn build_entry(&mut self) {
        let mut exit_sig = self.obj_module.make_signature();
        exit_sig.params.push(cl::AbiParam::new(cl::types::I32));
        let exit_func = cl::Function::with_name_signature(
            cl::UserFuncName::user(FuncNS::SystemFunc.into(), SystemFunc::Exit.into()),
            exit_sig,
        );
        let exit_func_id = self
            .obj_module
            .declare_function("exit", cl::Linkage::Import, &exit_func.signature)
            .unwrap();

        let mut func = cl::Function::with_name_signature(
            cl::UserFuncName::user(FuncNS::SystemFunc.into(), SystemFunc::Start.into()),
            self.obj_module.make_signature(),
        );
        let func_id = self
            .obj_module
            .declare_function("_start", cl::Linkage::Export, &func.signature)
            .unwrap();

        utils::replace_with(self, |mut this| {
            let mut func_ctx = cl::FunctionBuilderContext::new();
            let func_builder = cl::FunctionBuilder::new(&mut func, &mut func_ctx);
            let mut codegen = FuncCodegen::new(
                this.modules,
                Some(func_builder),
                this.obj_module,
                this.globals,
                this.funcs,
            );
            codegen.create_initial_block(&[]);

            let mut entry_point = None;

            for ((i, j), global) in codegen.globals.globals.clone() {
                if global.is_entry_point {
                    entry_point = Some((i, j));
                    continue;
                }
                if global.is_const {
                    continue;
                };
                for instr in &self.modules[i].globals[j].body {
                    codegen.add_instr(instr);
                }
                let res = codegen.stack.pop();
                codegen.store_global(res, &global);
            }

            let entry_point = entry_point.expect("entrypoint should be defined");
            for instr in &self.modules[entry_point.0].globals[entry_point.1].body {
                codegen.add_instr(instr);
            }
            let exit_code = codegen
                .stack
                .pop()
                .add_to_func(&mut codegen.obj_module, codegen.builder.as_mut().unwrap());
            codegen.call(exit_func_id, &[exit_code]);

            (this.obj_module, this.globals, this.funcs) = codegen.return_never();
            this
        });

        if self.cfg.dump_clif {
            println!("<_start> {func}");
        }

        self.module_ctx.func = func;
        self.obj_module
            .define_function(func_id, &mut self.module_ctx)
            .unwrap();
        self.obj_module.clear_context(&mut self.module_ctx)
    }
    fn declare_function(&mut self, mod_idx: usize, idx: usize) {
        let decl = &self.modules[mod_idx].funcs[idx];
        let mut sig = self.obj_module.make_signature();

        for param in &decl.params {
            sig.params.push(cl::AbiParam::new(types::get_type(
                &param.ty,
                self.modules,
                &self.obj_module,
            )));
        }
        sig.returns.push(cl::AbiParam::new(types::get_type(
            &decl.ret,
            self.modules,
            &self.obj_module,
        )));

        let user_func_name =
            cl::UserFuncName::user(FuncNS::User.into(), self.next_func_id);
        self.next_func_id += 1;

        let func = cl::Function::with_name_signature(user_func_name, sig);

        let symbol_name = if let Some(b::Extern { name }) = &decl.extn {
            name.clone()
        } else {
            // TODO: improve name mangling
            format!("$func_{mod_idx}_{idx}")
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
            .obj_module
            .declare_function(&symbol_name, linkage, &func.signature)
            .unwrap();

        self.funcs.insert(
            (mod_idx, idx),
            FuncBinding {
                symbol_name,
                is_extern: decl.extn.is_some(),
                func_id,
            },
        );
        self.declared_funcs.insert((mod_idx, idx), func);
    }
    fn declare_global(&mut self, mod_idx: usize, idx: usize) {
        self.globals
            .insert_global(mod_idx, idx, &mut self.obj_module);
    }
    fn build_function(&mut self, mod_idx: usize, idx: usize) {
        let decl = &self.modules[mod_idx].funcs[idx];
        utils::replace_with(self, |mut this| {
            let mut func_ctx = cl::FunctionBuilderContext::new();
            let func = this.declared_funcs.get_mut(&(mod_idx, idx)).unwrap();
            let func = cl::FunctionBuilder::new(func, &mut func_ctx);
            let mut codegen = FuncCodegen::new(
                this.modules,
                Some(func),
                this.obj_module,
                this.globals,
                this.funcs,
            );
            codegen.create_initial_block(&decl.params);

            for instr in &decl.body {
                codegen.add_instr(instr);
            }

            (this.obj_module, this.globals, this.funcs) = codegen.return_value();
            this
        })
    }
    fn write_to_file(mut self) {
        self.build_entry();

        if self.globals.data.len() > 0 && self.cfg.dump_clif {
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

        for key in self.funcs.keys().cloned().collect_vec() {
            let func_binding = self.funcs.remove(&key).unwrap();
            let func = self.declared_funcs.remove(&key).unwrap();

            if self.cfg.dump_clif {
                println!("<{}> {}", func_binding.symbol_name, func);
            }

            if func_binding.is_extern {
                continue;
            }

            self.module_ctx.func = func;
            self.obj_module
                .define_function(func_binding.func_id, &mut self.module_ctx)
                .unwrap();
            self.obj_module.clear_context(&mut self.module_ctx)
        }

        let obj_product = self.obj_module.finish();

        // FIXME: get file name from some kind of configuration
        let obj_path = env::temp_dir().join(format!(
            "{}.o",
            self.cfg.out.to_string_lossy().replace("/", "__")
        ));
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
            .arg(&self.cfg.out)
            .arg(&obj_path)
            .arg("-lc")
            .status()
            .expect("failed to link object file");

        if !status.success() {
            panic!("failed to link object file");
        }
    }
}
