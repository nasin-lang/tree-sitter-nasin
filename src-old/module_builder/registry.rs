use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::builtin_types::builtin_types;
use crate::mir;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VirtualValue {
    Never,
    Func(u32),
    Global(u32),
    Local(u32),
    Param(u32),
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<VirtualValue>),
    Record(Vec<(String, VirtualValue)>),
}

impl From<mir::Value> for VirtualValue {
    fn from(value: mir::Value) -> Self {
        match value {
            mir::Value::Local(idx) => VirtualValue::Local(idx),
            mir::Value::Param(idx) => VirtualValue::Param(idx),
        }
    }
}

#[derive(Debug, Clone)]
struct ValueType {
    ty: mir::Type,
    produced_by: ValueTypeDeps,
    consumed_by: HashSet<VirtualValue>,
}

impl Default for ValueType {
    fn default() -> Self {
        Self {
            ty: mir::Type::unknown(),
            produced_by: ValueTypeDeps::default(),
            consumed_by: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ValueTypeDeps {
    pub sig: Vec<mir::FuncType>,
    pub refs: Vec<VirtualValue>,
}

/// Arguments used to constrain a type, is used to avoid deep recursion in the type
/// inference by having a vector of constraints that are yet to be resolved
#[derive(Debug, PartialEq, Eq)]
enum TypeConstraintArgs {
    Direct {
        ty: mir::Type,
    },
    TopDown {
        produced_by: VirtualValue,
        producer_ty: mir::Type,
    },
    BottomUp {
        ty: mir::Type,
        consumed_by: VirtualValue,
    },
}

#[derive(Debug)]
pub enum RegistryScope {
    Module,
    Func(Box<Registry>),
}

impl RegistryScope {
    pub fn unwrap(self) -> Box<Registry> {
        match self {
            Self::Func(v) => v,
            Self::Module => panic!("Tried to unwrap RegistryScope::Module"),
        }
    }
}

#[derive(Debug)]
pub struct Registry {
    pub scope: RegistryScope,
    pub idents: HashMap<String, VirtualValue>,
    pub type_idents: HashMap<String, mir::Type>,
    funcs: Vec<mir::Type>,
    globals: Vec<ValueType>,
    types: Vec<mir::TypeDef>,
    params: Vec<ValueType>,
    locals: Vec<ValueType>,
}

impl Registry {
    pub fn new(scope: RegistryScope) -> Self {
        let (idents, type_idents) = match &scope {
            RegistryScope::Module => (HashMap::new(), builtin_types()),
            RegistryScope::Func(parent) => {
                (parent.idents.clone(), parent.type_idents.clone())
            }
        };
        Self {
            scope,
            idents,
            type_idents,
            funcs: vec![],
            globals: vec![],
            types: vec![],
            params: vec![],
            locals: vec![],
        }
    }

    pub fn register_func(
        &mut self,
        ident: &str,
        params: impl IntoIterator<Item = mir::Type>,
    ) -> u32 {
        let idx;
        let v_value;
        {
            let module_reg = self.module_registry_mut();

            idx = module_reg.funcs.len() as u32;
            v_value = VirtualValue::Func(idx);

            module_reg.funcs.push(module_reg.create_func_type(
                params,
                // Multiple return values are not supported yet
                [mir::Type::unknown()],
            ));
            module_reg.idents.insert(ident.to_string(), v_value.clone());
        }

        self.idents.insert(ident.to_string(), v_value);
        idx
    }

    pub fn register_global(
        &mut self,
        ident: &str,
        ty: mir::Type,
        produced_by: ValueTypeDeps,
    ) -> u32 {
        let idx;
        let v_value;
        {
            let module_reg = self.module_registry_mut();

            idx = module_reg.globals.len();
            v_value = VirtualValue::Global(idx as u32);

            module_reg.globals.push(ValueType {
                ty,
                produced_by,
                ..Default::default()
            });
            module_reg.idents.insert(ident.to_string(), v_value.clone());
            module_reg.constrain_value_type_chain(v_value.clone(), None);
        }

        self.idents.insert(ident.to_string(), v_value.clone());
        idx as u32
    }

    pub fn register_type(&mut self, ident: &str, typedef: mir::TypeDef) -> u32 {
        let idx;
        let ty;
        {
            let module_reg = self.module_registry_mut();

            idx = module_reg.globals.len() as u32;
            ty = mir::Type::TypeRef(idx);

            module_reg.types.push(typedef);
        };

        self.type_idents.insert(ident.to_string(), ty);
        return idx;
    }

    pub fn register_local(
        &mut self,
        ident: &str,
        ty: mir::Type,
        produced_by: ValueTypeDeps,
    ) -> u32 {
        let idx = self.locals.len() as u32;
        self.locals.push(ValueType {
            ty,
            produced_by,
            ..Default::default()
        });
        self.idents
            .insert(ident.to_string(), VirtualValue::Local(idx));
        self.constrain_value_type_chain(VirtualValue::Local(idx), None);
        idx as u32
    }

    pub fn register_param(&mut self, ident: &str, ty: mir::Type) -> u32 {
        assert!(matches!(self.scope, RegistryScope::Func(_)));

        let idx = self.params.len();
        self.params.push(ValueType {
            ty,
            ..Default::default()
        });
        self.idents
            .insert(ident.to_string(), VirtualValue::Param(idx as u32));
        idx as u32
    }

    pub fn global_type(&self, idx: u32) -> Option<mir::Type> {
        self.module_registry()
            .globals
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }

    pub fn func_type(&self, idx: u32) -> Option<mir::Type> {
        self.module_registry().funcs.get(idx as usize).cloned()
    }

    pub fn param_type(&self, idx: u32) -> Option<mir::Type> {
        self.params
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }

    pub fn local_type(&self, idx: u32) -> Option<mir::Type> {
        self.locals
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }

    pub fn typedef(&self, idx: u32) -> Option<&mir::TypeDef> {
        self.types.get(idx as usize)
    }

    pub fn get_types(&self) -> impl Iterator<Item = mir::TypeDef> + '_ {
        self.types.iter().cloned()
    }

    pub fn get_params(&self) -> impl Iterator<Item = mir::Param> + '_ {
        self.params.iter().map(|value_type| mir::Param {
            ty: value_type.ty.clone(),
        })
    }

    pub fn get_locals(&self) -> impl Iterator<Item = mir::Local> + '_ {
        self.locals.iter().map(|value_type| mir::Local {
            ty: value_type.ty.clone(),
        })
    }

    pub fn value_type(&self, v_value: &VirtualValue) -> Option<mir::Type> {
        match v_value {
            VirtualValue::Func(idx) => self.func_type(*idx),
            VirtualValue::Global(idx) => self.global_type(*idx),
            VirtualValue::Local(idx) => self.local_type(*idx),
            VirtualValue::Param(idx) => self.param_type(*idx),
            VirtualValue::Bool(_) => Some(mir::Type::Bool),
            VirtualValue::Number(n) => Some(self.create_num_type(n)),
            VirtualValue::String(s) => {
                Some(mir::Type::String(mir::StringType { len: Some(s.len()) }))
            }
            VirtualValue::Array(refs) => {
                let item_types = refs
                    .iter()
                    .map(|v| self.value_type(v))
                    .collect::<Option<Vec<_>>>()?;
                let item_ty = self.merge_types(item_types.iter()).expect(&format!(
                    "Failed to merge types of array items: {}",
                    item_types.iter().map(|t| t.to_string()).join(", ")
                ));
                Some(self.create_array_type(item_ty, Some(refs.len())))
            }
            VirtualValue::Record(v) => {
                let properties = v
                    .iter()
                    .map(|(key, vv)| Some((key.clone(), self.value_type(vv)?)))
                    .collect::<Option<Vec<_>>>()?;
                Some(self.create_object_type(properties))
            }
            VirtualValue::Never => panic!("VirtualValue::Never does not have type"),
        }
    }

    pub fn set_value_type(
        &mut self,
        v_value: VirtualValue,
        ty: mir::Type,
        consumed_by: Option<VirtualValue>,
    ) {
        if matches!(&v_value, VirtualValue::Number(_)) {
            panic!("{:?} cannot be refined", v_value);
        }

        if matches!(self.scope, RegistryScope::Func(_))
            && matches!(&v_value, VirtualValue::Func(_) | VirtualValue::Global(_))
        {
            panic!("{:?} cannot be refined inside a function", v_value);
        }

        if let VirtualValue::Func(idx) = &v_value {
            let idx = *idx as usize;

            if idx >= self.funcs.len() {
                panic!("Function {} does not exist", idx);
            }

            if consumed_by.is_some() {
                panic!("Cannot set type of a function indirectly");
            }

            self.funcs[idx] = self.merge_types([&self.funcs[idx], &ty]).expect(&format!(
                "Failed to merge types {} and {}",
                self.funcs[idx], ty
            ));

            return;
        }

        self.constrain_value_type_chain(
            v_value,
            if let Some(consumed_by) = consumed_by {
                Some(TypeConstraintArgs::BottomUp { ty, consumed_by })
            } else {
                Some(TypeConstraintArgs::Direct { ty })
            },
        );
    }

    pub fn module_registry(&self) -> &Self {
        match &self.scope {
            RegistryScope::Module => self,
            RegistryScope::Func(parent) => parent,
        }
    }

    pub fn module_registry_mut(&mut self) -> &mut Self {
        if matches!(&self.scope, RegistryScope::Module) {
            return self;
        }
        match &mut self.scope {
            RegistryScope::Func(parent) => parent,
            _ => unreachable!(),
        }
    }

    fn value_type_ref(&self, v_value: &VirtualValue) -> Option<&ValueType> {
        match v_value {
            VirtualValue::Global(idx) => {
                self.module_registry().globals.get(*idx as usize)
            }
            VirtualValue::Param(idx) => self.params.get(*idx as usize),
            VirtualValue::Local(idx) => self.locals.get(*idx as usize),
            _ => None,
        }
    }

    fn value_type_mut(&mut self, v_value: &VirtualValue) -> Option<&mut ValueType> {
        match v_value {
            VirtualValue::Global(idx) => {
                self.module_registry_mut().globals.get_mut(*idx as usize)
            }
            VirtualValue::Param(idx) => self.params.get_mut(*idx as usize),
            VirtualValue::Local(idx) => self.locals.get_mut(*idx as usize),
            _ => None,
        }
    }

    fn constrain_value_type_chain(
        &mut self,
        v_value: VirtualValue,
        args: Option<TypeConstraintArgs>,
    ) {
        let mut stack = vec![(v_value, args)];

        while stack.len() > 0 {
            let (v_value, args) = stack.pop().unwrap();

            if matches!(
                &v_value,
                VirtualValue::Bool(_) | VirtualValue::Number(_) | VirtualValue::String(_)
            ) {
                continue;
            }

            let Some(value_type) = self.value_type_ref(&v_value) else {
                dbg!(self);
                panic!(
                    "Type of {:?} cannot be refined with {:?} because it does not exist in the registry or is not mutable",
                    &v_value, &args
                );
            };
            let mut value_type = value_type.clone();

            let skip_ref = match args {
                Some(TypeConstraintArgs::Direct { ty }) => {
                    // We we'll not escape the loop in case of a redundant constraint when
                    // the constraint is direct because this might be used only to
                    // constrain the types of the references, like `insert_value_type`
                    // does.

                    value_type.ty = self.merge_types([&value_type.ty, &ty]).expect(
                        &format!("Failed to merge types {} and {}", value_type.ty, ty),
                    );

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| self.match_types([&sig.ret[0], &ty]));

                    None
                }
                Some(TypeConstraintArgs::TopDown {
                    produced_by,
                    producer_ty,
                }) => {
                    let arg_idx = value_type
                        .produced_by
                        .refs
                        .iter()
                        .position(|r| r == &produced_by)
                        .expect(&format!(
                            "Failed to constrain type: {:?} does not depend on {:?}",
                            &v_value, &produced_by
                        ));

                    let sigs_len = value_type.produced_by.sig.len();

                    value_type.produced_by.sig.retain(|sig| {
                        self.match_types([&producer_ty, &sig.params[arg_idx]])
                    });

                    // Since the new type is a merge of the returned types of the
                    // producing signatures, if it didn't change, we know that nothing
                    // changed and don't need need to propagate the change
                    if sigs_len == value_type.produced_by.sig.len() {
                        continue;
                    }

                    let ty = self
                        .merge_types(
                            value_type.produced_by.sig.iter().map(|sig| &sig.ret[0]),
                        )
                        .expect("Failed to merge types");

                    if value_type.ty == ty {
                        continue;
                    }

                    value_type.ty = ty;

                    Some(produced_by)
                }
                Some(TypeConstraintArgs::BottomUp { ty, consumed_by }) => {
                    value_type.consumed_by.insert(consumed_by.clone());

                    if value_type.ty == ty {
                        continue;
                    }

                    value_type.ty = self.merge_types([&value_type.ty, &ty]).expect(
                        &format!("Failed to merge types {} and {}", value_type.ty, ty),
                    );

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| self.match_types([&sig.ret[0], &ty]));

                    Some(consumed_by)
                }
                // If there are no constraints, we don't need to change the value type,
                // but we should still propagate it
                None => None,
            };

            // constrain the types that this depends on
            for (i, ref_value) in value_type.produced_by.refs.iter().enumerate() {
                if Some(ref_value) == skip_ref.as_ref() {
                    continue;
                }

                let ty = self.create_ambig_type(
                    value_type
                        .produced_by
                        .sig
                        .iter()
                        .map(|item| item.params[i].clone()),
                );

                stack.push((
                    ref_value.clone(),
                    Some(TypeConstraintArgs::BottomUp {
                        ty: ty.clone(),
                        consumed_by: v_value.clone(),
                    }),
                ))
            }

            // constrain the types that depend on this
            for consumed_by in value_type.consumed_by.iter() {
                if Some(consumed_by) == skip_ref.as_ref() {
                    continue;
                }

                stack.push((
                    consumed_by.clone(),
                    Some(TypeConstraintArgs::TopDown {
                        produced_by: v_value.clone(),
                        producer_ty: value_type.ty.clone(),
                    }),
                ))
            }

            let Some(value_type_mut) = self.value_type_mut(&v_value) else {
                panic!(
                    "Type of {:?} cannot be refined because it does not exist in the registry or is not mutable",
                    &v_value
                );
            };
            *value_type_mut = value_type;
        }
    }
}
