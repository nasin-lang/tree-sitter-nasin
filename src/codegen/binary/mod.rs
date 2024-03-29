mod func;
mod type_gen;

use std::env;
use std::fs::File;
use std::io::BufWriter;

use cranelift_codegen::ir::{types, AbiParam, Function, InstBuilder, TrapCode, UserFuncName};
use cranelift_codegen::{isa, settings, Context};
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{default_libcall_names, DataDescription, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use self::func::{FnCodegen, FuncBinding, GlobalBinding};
use self::type_gen::TypeGen;
use super::traits::Codegen;
use crate::proto::m_ir;

const EXIT_FUNC_IDX: usize = 0;

pub struct BinaryCodegen {
    module: ObjectModule,
    module_ctx: Context,
    globals: Vec<GlobalBinding>,
    funcs: Vec<FuncBinding>,
    intrinsic_funcs: Vec<FuncId>,
    declared_funcs: Vec<(FuncId, Function)>,
}

impl BinaryCodegen {
    pub fn new<V: Into<Vec<u8>>>(triple: Triple, name: V) -> Self {
        let settings_builder = settings::builder();
        let flags = settings::Flags::new(settings_builder);
        let isa_target = isa::lookup(triple).unwrap().finish(flags).unwrap();

        let module = ObjectModule::new(
            ObjectBuilder::new(isa_target, name, default_libcall_names()).unwrap(),
        );

        let module_ctx = module.make_context();

        let mut this = BinaryCodegen {
            module,
            module_ctx,
            globals: Vec::new(),
            funcs: Vec::new(),
            intrinsic_funcs: Vec::new(),
            declared_funcs: Vec::new(),
        };

        // Declare libc functions
        // TODO: detect which libc functions are needed and declare only those
        // TODO: use syscalls instead of libc functions, maybe we will have to
        //       implement a wrapper around syscall in C to be able to call it
        //       from Cranelift
        // void exit(int status);
        {
            let mut sig = this.module.make_signature();
            sig.params.push(AbiParam::new(types::I32));
            let func =
                Function::with_name_signature(UserFuncName::user(0, EXIT_FUNC_IDX as u32), sig);
            let func_id = this
                .module
                .declare_function("exit", Linkage::Import, &func.signature)
                .unwrap();

            this.intrinsic_funcs.push(func_id);
        }

        this
    }
}

impl Codegen for BinaryCodegen {
    fn declare_function(&mut self, idx: usize, decl: &m_ir::FnDecl) {
        let mut sig = self.module.make_signature();

        for param in &decl.params {
            sig.params
                .push(AbiParam::new(self.module.get_type(&param.r#type)));
        }
        for ret in &decl.ret {
            sig.returns.push(AbiParam::new(self.module.get_type(ret)));
        }

        let user_func_name = {
            // namespace is 1 because 0 is being used for reserved functions. This number is
            // completely arbitary and may be changed to be more involved in the future
            UserFuncName::user(1, idx as u32)
        };

        let func = Function::with_name_signature(user_func_name, sig);

        let symbol_name = if let Some(m_ir::Export { name }) = &decl.export {
            name.clone()
        } else {
            format!("$func{}", idx)
        };

        let linkage = if decl.export.is_some() {
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
            func_id,
            params: decl.params.clone(),
            ret: decl.ret.clone(),
        });
        self.declared_funcs.push((func_id, func));
    }

    fn build_function(&mut self, idx: usize, decl: &m_ir::FnDecl) {
        let func = &self.funcs[idx];
        let (_, native_func) = &mut self.declared_funcs[idx];

        let mut func_ctx = FunctionBuilderContext::new();

        let mut fn_codegen = FnCodegen::new(
            &func.symbol_name,
            &mut self.module,
            native_func,
            &mut func_ctx,
            &self.globals,
            &self.funcs,
            &decl.params,
            &decl.locals,
        );

        for instr in decl.body.iter() {
            if let Some(instr) = instr.instr.as_ref() {
                fn_codegen.instr(instr);
            }
        }

        println!(
            "<{}>: {}",
            &fn_codegen.symbol_name, &fn_codegen.builder.func
        );

        fn_codegen.finalize();
    }

    fn declare_global(&mut self, idx: usize, decl: &m_ir::DataDecl) {
        let native_ty = self.module.get_type(&decl.r#type);

        let symbol_name = if let Some(m_ir::Export { name }) = &decl.export {
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

        let mut desc = DataDescription::new();
        desc.define_zeroinit(native_ty.bytes() as usize);

        self.module.define_data(data_id, &desc).unwrap();

        self.globals.push(GlobalBinding {
            symbol_name: symbol_name.clone(),
            data_id,
            ty: decl.r#type.clone(),
            native_ty,
        });

        println!(
            "<{}> = global_value (id: {}, size: {})\n",
            symbol_name,
            data_id.as_u32(),
            desc.init.size()
        );
    }

    fn build_module_init(&mut self, init: &m_ir::ModuleInit) {
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
            if let Some(instr) = instr.instr.as_ref() {
                fn_codegen.instr(instr);
            }
        }

        // We're using the return of main as exit status only because strings and printing are
        // not implemented and I need some way to check if the program is working. When printing
        // is implemented, this will be changed to a fixed success status, as it should be. In
        // the future, main will evaluate in a Action monad that will be run in the entry
        // function
        let exit_code = fn_codegen.locals.last().unwrap().value.unwrap();

        let exit_func_ref = fn_codegen.module.declare_func_in_func(
            self.intrinsic_funcs[EXIT_FUNC_IDX].clone(),
            &mut fn_codegen.builder.func,
        );
        fn_codegen.builder.ins().call(exit_func_ref, &[exit_code]);

        fn_codegen
            .builder
            .ins()
            .trap(TrapCode::UnreachableCodeReached);

        fn_codegen.finalize();

        println!("<_start>: {}\n", &func);

        self.module_ctx.func = func;
        self.module
            .define_function(func_id, &mut self.module_ctx)
            .unwrap();
    }

    fn write_to_file(mut self, file: &str) {
        for (func_id, func) in self.declared_funcs {
            self.module_ctx.func = func;
            self.module
                .define_function(func_id, &mut self.module_ctx)
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
