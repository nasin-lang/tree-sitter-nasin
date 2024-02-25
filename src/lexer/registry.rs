use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

use crate::proto::lex;

/// Handles name dedup, anonymous variable naming and non-target specific name mangling
/// Stores known types and handles type checking and inference
#[derive(Debug, Clone)]
pub struct Registry {
    /// Maps original names to the internal ones. Whenever a name is shadowed, it's added to the
    /// Vec at the same entry, and the last one will be the one used when the name is referenced
    /// again
    name_map: HashMap<String, Vec<String>>,
    /// Maps the types of values to their identifiers
    value_types: HashMap<String, lex::Type>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            name_map: HashMap::new(),
            value_types: HashMap::new(),
        }
    }

    pub fn with_parent(parent: &Registry) -> Self {
        Self {
            name_map: parent.name_map.clone(),
            value_types: parent.value_types.clone(),
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

    /// Append the names from the given registry to this one
    pub fn append_names(&mut self, other: &Registry) {
        for (key, value) in other.name_map.iter() {
            let mut value = value.clone();
            self.name_map
                .entry(key.clone())
                .and_modify(|current| current.append(&mut value))
                .or_insert(value);
        }
    }

    pub fn value_type(&self, value: &lex::Value) -> Option<lex::Type> {
        match &value.value {
            Some(lex::value::Value::Ident(ident)) => self.value_types.get(ident).cloned(),
            Some(lex::value::Value::Num(num)) => Some(Self::num_type(num)),
            _ => None,
        }
    }

    pub fn set_value_type(&mut self, ident: String, ty: lex::Type) {
        self.value_types.insert(ident, ty);
    }

    /// Merge two types into a single type and register the constraint for weak types
    /// TODO: constraint weak types
    pub fn merge_types(&mut self, a: &lex::Type, b: &lex::Type) -> lex::Type {
        macro_rules! types_iter {
            ($ty:expr) => {{
                let ty = $ty;
                if let Some(lex::r#type::Type::Ambig(ambig)) = &ty.r#type {
                    ambig.types.clone().into_iter()
                } else {
                    vec![ty.clone()].into_iter()
                }
            }};
        }

        let a_types = HashSet::<_, RandomState>::from_iter(types_iter!(a));
        let b_types = HashSet::<_, RandomState>::from_iter(types_iter!(b));

        let types: Vec<_> = a_types.intersection(&b_types).collect();

        if types.is_empty() {
            // FIXME: better error handling
            panic!("No common types between {} and {}", a, b);
        }

        if types.len() == 1 {
            return (types[0]).clone();
        }

        lex::Type {
            r#type: Some(lex::r#type::Type::Ambig(lex::AmbigType {
                types: types.into_iter().cloned().collect(),
            })),
        }
    }

    pub fn num_type(num: &str) -> lex::Type {
        let is_float = num.contains('.');
        let is_negative = num.starts_with('-');

        let types = if is_float {
            vec![lex::PrimType::F32, lex::PrimType::F64]
        } else if is_negative {
            vec![
                lex::PrimType::I8,
                lex::PrimType::I16,
                lex::PrimType::I32,
                lex::PrimType::I64,
                lex::PrimType::F32,
                lex::PrimType::F64,
            ]
        } else {
            vec![
                lex::PrimType::U8,
                lex::PrimType::U16,
                lex::PrimType::U32,
                lex::PrimType::U64,
                lex::PrimType::USize,
                lex::PrimType::I8,
                lex::PrimType::I16,
                lex::PrimType::I32,
                lex::PrimType::I64,
                lex::PrimType::F32,
                lex::PrimType::F64,
            ]
        };

        let types = types
            .into_iter()
            .map(|t| lex::Type {
                r#type: Some(lex::r#type::Type::Primitive(t.into())),
            })
            .collect();

        lex::Type {
            r#type: Some(lex::r#type::Type::Ambig(lex::AmbigType { types })),
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
