use std::collections::{HashMap, HashSet};

use super::types::{ambig, eq_types, match_types, merge_types, num_type};
use crate::proto::m_ir;

#[derive(Debug, Clone)]
struct ValueType {
    ty: m_ir::Type,
    produced_by: ValueTypeDeps,
    consumed_by: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ValueTypeDeps {
    pub sig: Vec<m_ir::FnType>,
    pub refs: Vec<m_ir::Value>,
}

/// Arguments used to constrain a type, is used to avoid deep recursion in the type inference by
/// having a vector of constraints that are yet to be resolved
#[derive(Debug, PartialEq, Eq)]
enum TypeConstraintArgs {
    Direct {
        ty: m_ir::Type,
    },
    TopDown {
        produced_by: String,
        producer_ty: m_ir::Type,
    },
    BottomUp {
        ty: m_ir::Type,
        consumed_by: String,
    },
}

/// Handles name dedup, anonymous variable naming and non-target specific name mangling
/// Stores known types and handles type checking and inference
#[derive(Debug, Clone)]
pub struct Registry {
    /// Maps original names to the internal ones. Whenever a name is shadowed, it's added to the
    /// Vec at the same entry, and the last one will be the one used when the name is referenced
    /// again
    name_map: HashMap<String, Vec<String>>,
    /// Maps the types of values to their identifiers
    value_types: HashMap<String, ValueType>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            name_map: HashMap::new(),
            value_types: HashMap::new(),
        }
    }

    /// Get a new internal name for the given original name. If the original name is already in use,
    /// it will be suffixed with a number. If the original name is None, a new temporary name will
    /// be generated
    pub fn use_name(&mut self, original_name: Option<&str>) -> String {
        let original_name = original_name.unwrap_or("");
        let count = self.count_ident(original_name);

        let internal_name = if original_name == "" {
            format!("v{}", count + 1)
        } else {
            if count > 0 {
                format!("{}_{}", original_name, count)
            } else {
                original_name.to_string()
            }
        };

        if !self.name_map.contains_key(original_name) {
            self.name_map.insert(original_name.to_string(), Vec::new());
        }

        let internal_names = self.name_map.get_mut(original_name).unwrap();
        internal_names.push(internal_name.clone());

        internal_name
    }

    /// Get the last internal name for the given original name
    pub fn get_internal_name(&self, original_name: &str) -> Option<&str> {
        self.name_map
            .get(original_name)
            .map(|names| names.last().unwrap().as_str())
    }

    pub fn value_type(&self, value: &m_ir::Value) -> m_ir::Type {
        match &value.value {
            Some(m_ir::value::Value::Ident(ident)) => match self.value_types.get(ident) {
                Some(value_type) => value_type.ty.clone(),
                None => m_ir::Type {
                    r#type: Some(m_ir::r#type::Type::Unknown(true)),
                },
            },
            Some(m_ir::value::Value::Num(num)) => num_type(num),
            None => m_ir::Type {
                r#type: Some(m_ir::r#type::Type::Unknown(true)),
            },
        }
    }

    pub fn insert_value_type(&mut self, ident: &str, ty: m_ir::Type, produced_by: ValueTypeDeps) {
        self.value_types.insert(
            ident.to_string(),
            ValueType {
                ty: ty.clone(),
                produced_by,
                consumed_by: HashSet::new(),
            },
        );
        self.constrain_value_type_chain(ident, None);
    }

    pub fn set_value_type(&mut self, ident: &str, ty: m_ir::Type, consumed_by: Option<&str>) {
        self.constrain_value_type_chain(
            ident,
            if let Some(consumed_by) = consumed_by {
                Some(TypeConstraintArgs::BottomUp {
                    ty,
                    consumed_by: consumed_by.to_string(),
                })
            } else {
                Some(TypeConstraintArgs::Direct { ty })
            },
        );
    }

    fn constrain_value_type_chain(&mut self, ident: &str, args: Option<TypeConstraintArgs>) {
        let mut stack = vec![(ident.to_string(), args)];

        while stack.len() > 0 {
            let (ident, args) = stack.pop().unwrap();

            let Some(value_type) = self.value_types.get_mut(&ident) else {
                continue;
            };

            let skip_ident = match args {
                Some(TypeConstraintArgs::Direct { ty }) => {
                    // We we'll not escape the loop in case of a redundant constraint when the
                    // constraint is direct because this might be used only to constrain the types
                    // of the references, like `insert_value_type` does.

                    value_type.ty = merge_types([&value_type.ty, &ty]).expect(&format!(
                        "Failed to merge types {} and {}",
                        value_type.ty, ty
                    ));

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| match_types([&sig.ret[0], &ty]));

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
                        .position(|ref_value| {
                            let Some(m_ir::value::Value::Ident(ref_ident)) =
                                &ref_value.value.as_ref()
                            else {
                                return false;
                            };
                            ref_ident == &produced_by
                        })
                        .expect(&format!(
                            "Failed to constrain type: {} does not depend on {}",
                            &ident, &produced_by
                        ));

                    let sigs_len = value_type.produced_by.sig.len();

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| match_types([&producer_ty, &sig.args[arg_idx]]));

                    // Since the new type is a merge of the returned types of the producing
                    // signatures, if it didn't change, we know that nothing changed and don't need
                    // need to propagate the change
                    if sigs_len == value_type.produced_by.sig.len() {
                        continue;
                    }

                    let ty = merge_types(value_type.produced_by.sig.iter().map(|sig| &sig.ret[0]))
                        .expect("Failed to merge types");

                    if eq_types(&value_type.ty, &ty) {
                        continue;
                    }

                    value_type.ty = ty;

                    Some(produced_by)
                }
                Some(TypeConstraintArgs::BottomUp { ty, consumed_by }) => {
                    value_type.consumed_by.insert(consumed_by.clone());

                    if eq_types(&value_type.ty, &ty) {
                        continue;
                    }

                    value_type.ty = merge_types([&value_type.ty, &ty]).expect(&format!(
                        "Failed to merge types {} and {}",
                        value_type.ty, ty
                    ));

                    value_type
                        .produced_by
                        .sig
                        .retain(|sig| match_types([&sig.ret[0], &ty]));

                    Some(consumed_by)
                }
                // If there are no constraints, we don't need to change the value type, but we
                // should still propagate it
                None => None,
            };

            // constrain the types that this depends on
            for (i, ref_value) in value_type.produced_by.refs.iter().enumerate() {
                let Some(m_ir::value::Value::Ident(ref_ident)) = &ref_value.value.as_ref() else {
                    continue;
                };

                if Some(ref_ident) == skip_ident.as_ref() {
                    continue;
                }

                let ty = ambig(
                    value_type
                        .produced_by
                        .sig
                        .iter()
                        .map(|item| item.args[i].clone()),
                );

                stack.push((
                    ref_ident.clone(),
                    Some(TypeConstraintArgs::BottomUp {
                        ty: ty.clone(),
                        consumed_by: ident.to_string(),
                    }),
                ))
            }

            // constrain the types that depend on this
            for consumed_by in value_type.consumed_by.iter() {
                if Some(consumed_by) == skip_ident.as_ref() {
                    continue;
                }

                stack.push((
                    consumed_by.clone(),
                    Some(TypeConstraintArgs::TopDown {
                        produced_by: ident.to_string(),
                        producer_ty: value_type.ty.clone(),
                    }),
                ))
            }
        }
    }

    fn count_ident(&self, ident: &str) -> usize {
        let count = self
            .name_map
            .get(ident)
            .map(|names| names.len())
            .unwrap_or(0);

        count
    }
}
