use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, RwLock};

use derive_more::{Deref, DerefMut};
use derive_new::new;
use tree_sitter as ts;

use crate::utils::indented;
use crate::{bytecode as b, codegen, config, errors, parser, sources, typecheck};

#[derive(Debug, Deref, DerefMut, new)]
pub struct BuildContext {
    pub cfg: config::BuildConfig,
    #[new(default)]
    #[deref]
    #[deref_mut]
    pub source_manager: sources::SourceManager,
    #[new(default)]
    pub errors: Mutex<Vec<errors::Error>>,
    #[new(default)]
    modules: RwLock<Vec<b::Module>>,
    #[new(default)]
    core_mod_idx: Option<usize>,
}
impl BuildContext {
    pub fn lock_modules(&self) -> impl Deref<Target = Vec<b::Module>> + '_ {
        self.modules.read().unwrap()
    }
    pub fn lock_modules_mut(&self) -> impl DerefMut<Target = Vec<b::Module>> + '_ {
        self.modules.write().unwrap()
    }
    pub fn push_error(&self, value: errors::Error) {
        self.errors.lock().unwrap().push(value);
    }
    pub fn parse(&self, src_idx: usize) -> usize {
        let mut ts_parser = ts::Parser::new();
        ts_parser
            .set_language(&tree_sitter_nasin::LANGUAGE.into())
            .unwrap();
        let tree = ts_parser
            .parse(&self.source_manager.source(src_idx).content().text, None)
            .expect("Could not parse this file");
        let root_node = tree.root_node();

        if self.cfg.dump_ast {
            println!("{}", root_node.to_sexp());
        }

        let mod_idx = {
            let mut modules = self.lock_modules_mut();
            let sources = self
                .source_manager
                .sources
                .iter()
                .map(|s| s.into())
                .collect();
            modules.push(b::Module::new(sources));
            modules.len() - 1
        };

        let mut module_parser = parser::ModuleParser::new(self, src_idx, mod_idx);
        if let Some(core_mod_idx) = self.core_mod_idx {
            module_parser.open_module(core_mod_idx);
        }
        module_parser.add_root(root_node);
        module_parser.finish();
        typecheck::TypeChecker::new(self, mod_idx).check();

        if self.cfg.dump_bytecode {
            println!(
                "module {mod_idx}:\n{}",
                indented(4, [&self.lock_modules()[mod_idx]])
            );
        }

        mod_idx
    }
    pub fn parse_library(&mut self) {
        let lib_dir = PathBuf::from(
            option_env!("LIB_DIR").expect("env LIB_DIR should be provided"),
        );
        let core_src_idx = self
            .source_manager
            .preload(lib_dir.join("core.nsn"))
            .expect("should be able to locate core.nsn");
        self.core_mod_idx = Some(self.parse(core_src_idx));
    }
    pub fn compile(&self) {
        let modules = self.lock_modules();

        fs::create_dir_all(self.cfg.out.parent().unwrap()).unwrap();

        let codegen = codegen::BinaryCodegen::new(&modules, &self.cfg);
        codegen.write();

        if !self.cfg.silent {
            println!("Compiled program to {}", self.cfg.out.to_string_lossy());
        }
    }
}
