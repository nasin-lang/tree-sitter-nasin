mod func;
mod globals;
mod types;

use cranelift_shim::{self as cl, Module};
use target_lexicon::Triple;

use self::func::FuncCodegen;
use self::globals::Globals;
use super::traits::Codegen;
use crate::utils::{self, enumerate};
use crate::{bytecode as b, config};

enumerate!(pub FuncNS: u32 {
    User = 0,
    InternalFunc = 1,
});

enumerate!(pub BuiltinFunc: u32 {
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

            this.module = codegen.module;
            this.globals = codegen.globals;
            this.funcs = codegen.funcs;
            this
        })
    }

    fn write_to_file(self, _file: &std::path::Path) {
        todo!()
    }
}
