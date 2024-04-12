use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::mir;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VirtualValue {
    Func(u32),
    Global(u32),
    Local(u32),
    Param(u32),
    Number(String),
    Array(Vec<VirtualValue>),
}

impl From<mir::Value> for VirtualValue {
    fn from(value: mir::Value) -> Self {
        match value {
            mir::Value::Local(idx) => VirtualValue::Local(idx),
            mir::Value::Param(idx) => VirtualValue::Param(idx),
        }
    }
}

impl TryFrom<VirtualValue> for mir::GlobalConstValue {
    type Error = ();

    fn try_from(value: VirtualValue) -> Result<Self, Self::Error> {
        match value {
            VirtualValue::Number(n) => Ok(mir::GlobalConstValue::Number(n)),
            VirtualValue::Array(ref values) => {
                let values = values
                    .iter()
                    .map(|v| v.clone().try_into())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(mir::GlobalConstValue::Array(values))
            }
            _ => Err(()),
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
            ty: mir::Type::Unknown,
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

/// Maps identifiers to the last ref (local index, global index, etc) that represents
/// them. Whenever a identifier is shadowed, its id is updated
#[derive(Debug, Clone)]
pub struct IdentMap {
    map: HashMap<String, VirtualValue>,
}

impl IdentMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ident: &str, v_value: VirtualValue) {
        self.map.insert(ident.to_string(), v_value);
    }

    pub fn get(&self, ident: &str) -> Option<VirtualValue> {
        self.map.get(ident).cloned()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn extend(&mut self, other: Self) {
        self.map.extend(other.map);
    }
}

pub trait Registry {
    fn idents(&self) -> &IdentMap;
    fn idents_mut(&mut self) -> &mut IdentMap;
    fn register_local(&mut self, ident: &str, ty: mir::Type, produced_by: ValueTypeDeps) -> u32;
    fn get_locals(&self) -> impl Iterator<Item = mir::Local>;
    fn global_type(&self, idx: u32) -> Option<mir::Type>;
    fn func_type(&self, idx: u32) -> Option<mir::Type>;
    fn param_type(&self, idx: u32) -> Option<mir::Type>;
    fn local_type(&self, idx: u32) -> Option<mir::Type>;
    fn set_value_type(
        &mut self,
        v_value: VirtualValue,
        ty: mir::Type,
        consumed_by: Option<VirtualValue>,
    );

    fn value_type(&self, v_value: &VirtualValue) -> Option<mir::Type> {
        match v_value {
            VirtualValue::Func(idx) => self.func_type(*idx),
            VirtualValue::Global(idx) => self.global_type(*idx),
            VirtualValue::Local(idx) => self.local_type(*idx),
            VirtualValue::Param(idx) => self.param_type(*idx),
            VirtualValue::Number(n) => Some(mir::Type::num_type(n)),
            VirtualValue::Array(refs) => {
                let item_types = refs
                    .iter()
                    .map(|v| self.value_type(v))
                    .collect::<Option<Vec<_>>>()?;
                let item_ty = mir::Type::merge(item_types.iter()).expect(&format!(
                    "Failed to merge types of array items: {}",
                    item_types.iter().map(|t| t.to_string()).join(", ")
                ));
                Some(mir::Type::array_type(item_ty, Some(refs.len())))
            }
        }
    }
}

trait RegistryExt: Registry {
    fn get_mut_value_type(&mut self, v_value: &VirtualValue) -> Option<&mut ValueType>;

    fn constrain_value_type_chain(
        &mut self,
        v_value: VirtualValue,
        args: Option<TypeConstraintArgs>,
    ) {
        let mut stack = vec![(v_value, args)];

        while stack.len() > 0 {
            let (v_value, args) = stack.pop().unwrap();

            if matches!(&v_value, VirtualValue::Number(_)) {
                return;
            }

            let Some(value_type) = self.get_mut_value_type(&v_value) else {
                panic!(
                    "Type of {:?} cannot be refined with {:?} because it does not exist in the registry or is not mutable",
                    v_value, args
                );
            };

            let skip_ref = match args {
                Some(TypeConstraintArgs::Direct { ty }) => {
                    // We we'll not escape the loop in case of a redundant constraint when
                    // the constraint is direct because this might be used only to
                    // constrain the types of the references, like `insert_value_type`
                    // does.

                    value_type.ty = value_type.ty.merge_with(&ty).expect(&format!(
                        "Failed to merge types {} and {}",
                        value_type.ty, ty
                    ));

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| mir::Type::matches([&sig.ret[0], &ty]));

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

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| mir::Type::matches([&producer_ty, &sig.params[arg_idx]]));

                    // Since the new type is a merge of the returned types of the
                    // producing signatures, if it didn't change, we know that nothing
                    // changed and don't need need to propagate the change
                    if sigs_len == value_type.produced_by.sig.len() {
                        continue;
                    }

                    let ty =
                        mir::Type::merge(value_type.produced_by.sig.iter().map(|sig| &sig.ret[0]))
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

                    value_type.ty = value_type.ty.merge_with(&ty).expect(&format!(
                        "Failed to merge types {} and {}",
                        value_type.ty, ty
                    ));

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| mir::Type::matches([&sig.ret[0], &ty]));

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

                let ty = mir::Type::ambig(
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
        }
    }
}

#[derive(Debug)]
pub struct ModuleRegistry {
    pub global_idents: IdentMap,
    pub init_idents: IdentMap,
    funcs: Vec<mir::Type>,
    globals: Vec<ValueType>,
    init_locals: Vec<ValueType>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            global_idents: IdentMap::new(),
            init_idents: IdentMap::new(),
            init_locals: Vec::new(),
            funcs: Vec::new(),
            globals: Vec::new(),
        }
    }

    pub fn register_func<P>(&mut self, ident: &str, params: P) -> u32
    where
        P: IntoIterator<Item = mir::Type>,
    {
        let idx = self.funcs.len();
        let v_value = VirtualValue::Func(idx as u32);
        self.funcs.push(mir::Type::func_type(
            params,
            // Multiple return values are not supported yet
            [mir::Type::Unknown],
        ));
        self.global_idents.insert(ident, v_value.clone());
        self.init_idents.insert(ident, v_value);
        idx as u32
    }

    pub fn register_global(
        &mut self,
        ident: &str,
        ty: mir::Type,
        produced_by: ValueTypeDeps,
    ) -> u32 {
        let idx = self.globals.len();
        let v_value = VirtualValue::Global(idx as u32);
        self.globals.push(ValueType {
            ty,
            produced_by,
            ..Default::default()
        });
        self.global_idents.insert(ident, v_value.clone());
        self.constrain_value_type_chain(v_value.clone(), None);

        idx as u32
    }
}

impl Registry for ModuleRegistry {
    fn idents(&self) -> &IdentMap {
        &self.init_idents
    }

    fn idents_mut(&mut self) -> &mut IdentMap {
        &mut self.init_idents
    }

    fn global_type(&self, idx: u32) -> Option<mir::Type> {
        self.globals
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }
    fn func_type(&self, idx: u32) -> Option<mir::Type> {
        self.funcs.get(idx as usize).cloned()
    }
    fn param_type(&self, _idx: u32) -> Option<mir::Type> {
        None
    }
    fn local_type(&self, idx: u32) -> Option<mir::Type> {
        self.init_locals
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }

    fn set_value_type(
        &mut self,
        v_value: VirtualValue,
        ty: mir::Type,
        consumed_by: Option<VirtualValue>,
    ) {
        if matches!(&v_value, VirtualValue::Number(_)) {
            panic!("{:?} cannot be refined", v_value);
        }

        if let VirtualValue::Func(idx) = &v_value {
            let idx = *idx as usize;

            if idx >= self.funcs.len() {
                panic!("Function {} does not exist", idx);
            }

            if consumed_by.is_some() {
                panic!("Cannot set type of a function indirectly");
            }

            self.funcs[idx] = self.funcs[idx].merge_with(&ty).expect(&format!(
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

    fn register_local(&mut self, ident: &str, ty: mir::Type, produced_by: ValueTypeDeps) -> u32 {
        let idx = self.init_locals.len();
        let v_value = VirtualValue::Local(idx as u32);
        self.init_locals.push(ValueType {
            ty,
            produced_by,
            ..Default::default()
        });
        self.init_idents.insert(ident, v_value.clone());
        self.constrain_value_type_chain(v_value.clone(), None);

        idx as u32
    }

    fn get_locals(&self) -> impl Iterator<Item = mir::Local> {
        self.init_locals.iter().map(|value_type| mir::Local {
            ty: value_type.ty.clone(),
        })
    }
}

impl RegistryExt for ModuleRegistry {
    fn get_mut_value_type(&mut self, v_value: &VirtualValue) -> Option<&mut ValueType> {
        match v_value {
            VirtualValue::Global(idx) => self.globals.get_mut(*idx as usize),
            VirtualValue::Local(idx) => self.init_locals.get_mut(*idx as usize),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct FuncRegistry<'a> {
    pub module_registry: &'a mut ModuleRegistry,
    pub idents: IdentMap,
    params: Vec<ValueType>,
    locals: Vec<ValueType>,
}

impl<'a> FuncRegistry<'a> {
    pub fn new(module_registry: &'a mut ModuleRegistry) -> Self {
        let idents = module_registry.global_idents.clone();
        Self {
            module_registry,
            idents,
            params: Vec::new(),
            locals: Vec::new(),
        }
    }

    pub fn register_param(&mut self, ident: &str, ty: mir::Type) -> u32 {
        let idx = self.params.len();
        self.params.push(ValueType {
            ty,
            ..Default::default()
        });
        self.idents.insert(ident, VirtualValue::Param(idx as u32));
        idx as u32
    }

    pub fn get_params(&self) -> impl Iterator<Item = mir::Param> {
        self.params
            .clone()
            .into_iter()
            .map(|value_type| mir::Param { ty: value_type.ty })
    }
}

impl Registry for FuncRegistry<'_> {
    fn idents(&self) -> &IdentMap {
        &self.idents
    }

    fn idents_mut(&mut self) -> &mut IdentMap {
        &mut self.idents
    }

    fn global_type(&self, idx: u32) -> Option<mir::Type> {
        self.module_registry.global_type(idx)
    }
    fn func_type(&self, idx: u32) -> Option<mir::Type> {
        self.module_registry.func_type(idx)
    }
    fn param_type(&self, idx: u32) -> Option<mir::Type> {
        self.params
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }
    fn local_type(&self, idx: u32) -> Option<mir::Type> {
        self.locals
            .get(idx as usize)
            .map(|value_type| value_type.ty.clone())
    }

    fn set_value_type(
        &mut self,
        v_value: VirtualValue,
        ty: mir::Type,
        consumed_by: Option<VirtualValue>,
    ) {
        if matches!(&v_value, VirtualValue::Number(_)) {
            panic!("{:?} cannot be refined", v_value);
        }

        if matches!(&v_value, VirtualValue::Func(_) | VirtualValue::Global(_)) {
            panic!("{:?} cannot be refined inside a function", v_value);
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

    fn register_local(&mut self, ident: &str, ty: mir::Type, produced_by: ValueTypeDeps) -> u32 {
        let idx = self.locals.len();
        self.locals.push(ValueType {
            ty,
            produced_by,
            ..Default::default()
        });
        self.idents.insert(ident, VirtualValue::Local(idx as u32));
        self.constrain_value_type_chain(VirtualValue::Local(idx as u32), None);

        idx as u32
    }

    fn get_locals(&self) -> impl Iterator<Item = mir::Local> {
        self.locals.iter().map(|value_type| mir::Local {
            ty: value_type.ty.clone(),
        })
    }
}

impl<'a> RegistryExt for FuncRegistry<'a> {
    fn get_mut_value_type(&mut self, v_value: &VirtualValue) -> Option<&mut ValueType> {
        match v_value {
            VirtualValue::Param(idx) => self.params.get_mut(*idx as usize),
            VirtualValue::Local(idx) => self.locals.get_mut(*idx as usize),
            _ => panic!("{:?} cannot be refined inside a function", v_value),
        }
    }
}
