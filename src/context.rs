use std::fs;
use std::sync::{Mutex, RwLock};

use derive_more::{Deref, DerefMut};
use derive_new::new;
use tree_sitter as ts;

use crate::{bytecode as b, codegen, config, errors, parser, sources, typecheck};

#[derive(Debug, Deref, DerefMut, new)]
pub struct BuildContext<'a> {
    pub cfg: config::BuildConfig,
    #[new(default)]
    #[deref]
    #[deref_mut]
    pub source_manager: sources::SourceManager,
    #[new(default)]
    pub errors: Mutex<Vec<errors::Error<'a>>>,
    #[new(default)]
    modules: RwLock<Vec<b::Module>>,
}
impl<'a> BuildContext<'a> {
    pub fn lock_modules(&self) -> impl Deref<Target = Vec<b::Module>> + '_ {
        self.modules.read().unwrap()
    }
    pub fn lock_modules_mut(&self) -> impl DerefMut<Target = Vec<b::Module>> + '_ {
        self.modules.write().unwrap()
    }
    pub fn parse(&'a self, src_idx: usize) -> usize {
        let mut ts_parser = ts::Parser::new();
        ts_parser
            .set_language(tree_sitter_torvo::language())
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
        module_parser.add_root(root_node);

        module_parser.finish();
        let err = typecheck::TypeChecker::new(self, mod_idx).check();

        if self.cfg.dump_bytecode {
            println!("{}", &self.lock_modules()[mod_idx]);
        }

        {
            self.errors.lock().unwrap().extend(err);
        }

        mod_idx
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
