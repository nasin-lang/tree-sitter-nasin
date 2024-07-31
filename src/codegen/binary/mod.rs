mod func;
mod globals;
mod types;

use cranelift_shim::{self as cl, Module};
use target_lexicon::Triple;

use self::func::FuncCodegen;
use self::globals::Globals;
use super::traits::Codegen;
use crate::utils::enumerate;
use crate::{bytecode as b, config};

enumerate!(pub FuncNS: u32 {
    User = 0,
    InternalFunc = 1,
});

enumerate!(pub BuiltinFunc: u32 {
    Start = 0,
    Exit = 1,
});

pub struct FuncBinding<'a> {
    pub is_extern: bool,
    pub symbol_name: String,
    pub func_id: cl::FuncId,
    pub params: &'a [b::Param],
    pub ret: &'a b::Type,
}

pub struct BinaryCodegen<'a> {
    pub module: cl::ObjectModule,
    module_ctx: cl::Context,
    typedefs: Vec<&'a b::TypeDef>,
    globals: Globals<'a>,
    funcs: Vec<FuncBinding<'a>>,
    declared_funcs: Vec<cl::Function>,
    dump_clif: bool,
}

impl<'a> BinaryCodegen<'a> {
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

impl<'a> Codegen<'a> for BinaryCodegen<'a> {
    fn declare_typedef(&mut self, _idx: usize, decl: &'a b::TypeDef) {
        self.typedefs.push(decl);
    }

    fn declare_function(&mut self, idx: usize, decl: &'a b::Func) {
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
            params: &decl.params,
            ret: &decl.ret,
        });
        self.declared_funcs.push(func);
    }

    fn declare_global(&mut self, idx: usize, decl: &'a b::Global) {
        self.globals
            .insert_global(idx, decl, &mut self.module, &self.typedefs);
    }

    fn build_function(&mut self, idx: usize, decl: &'a b::Func) {
        let mut func_ctx = cl::FunctionBuilderContext::new();
        let mut func =
            cl::FunctionBuilder::new(&mut self.declared_funcs[idx], &mut func_ctx);
        let mut codegen = FuncCodegen::new(
            Some(&mut func),
            &mut self.module,
            &mut self.globals,
            &self.funcs,
            &self.typedefs,
        );
        codegen.create_initial_block(&decl.params);

        for instr in &decl.body {
            codegen.add_instr(instr);
        }
    }

    fn write_to_file(self, file: &std::path::Path) {
        todo!()
    }
}
