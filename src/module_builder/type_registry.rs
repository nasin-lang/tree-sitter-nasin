use std::collections::HashSet;

use itertools::{izip, Itertools};

use super::registry::Registry;
use crate::mir;

impl Registry {
    /// Returns the type of a number literal. Most of the time, this will be a ambiguous
    /// type, including all possible types that the number can be parsed as.
    pub fn create_num_type(&self, num: &str) -> mir::Type {
        let is_float = num.contains('.');
        let is_negative = num.starts_with('-');

        if is_float {
            self.create_ambig_type([mir::Type::F32, mir::Type::F64])
        } else if is_negative {
            self.create_ambig_type([
                mir::Type::I8,
                mir::Type::I16,
                mir::Type::I32,
                mir::Type::I64,
                mir::Type::F32,
                mir::Type::F64,
            ])
        } else {
            self.create_ambig_type([
                mir::Type::U8,
                mir::Type::U16,
                mir::Type::U32,
                mir::Type::U64,
                mir::Type::USize,
                mir::Type::I8,
                mir::Type::I16,
                mir::Type::I32,
                mir::Type::I64,
                mir::Type::F32,
                mir::Type::F64,
            ])
        }
    }

    /// Returns an ambiguous type with the given types. If there is only one type, returns
    /// that type instead. If no types are given, returns an unknown type.
    pub fn create_ambig_type(
        &self,
        types: impl IntoIterator<Item = mir::Type>,
    ) -> mir::Type {
        let types: HashSet<_> = types
            .into_iter()
            .flat_map(|ty| self.into_possible_types(ty))
            .collect();

        if types.len() == 1 {
            types.into_iter().next().unwrap()
        } else {
            mir::Type::Infer(mir::InferType::new(types, []))
        }
    }

    /// Returns an type for a inferred value with the given properties.
    pub fn create_object_type(
        &self,
        props: impl IntoIterator<Item = (String, mir::Type)>,
    ) -> mir::Type {
        let types: Vec<_> = props
            .into_iter()
            .map(|(name, ty)| {
                self.into_possible_types(ty)
                    .into_iter()
                    .map(move |ty| (name.clone(), ty))
            })
            .multi_cartesian_product()
            .map(|props| mir::Type::Infer(mir::InferType::new([], props)))
            .collect();

        if types.len() == 1 {
            types.into_iter().next().unwrap()
        } else {
            mir::Type::Infer(mir::InferType::new(types, []))
        }
    }

    /// Returns a type for a function. If any of the arguments or the return type is
    /// ambiguous, returns an ambiguous type for all combinations of the function
    /// signature.
    pub fn create_func_type(
        &self,
        args: impl IntoIterator<Item = mir::Type>,
        ret: impl IntoIterator<Item = mir::Type>,
    ) -> mir::Type {
        let args = args
            .into_iter()
            .map(|ty| self.into_possible_types(ty))
            .multi_cartesian_product();
        let ret = ret
            .into_iter()
            .map(|ty| self.into_possible_types(ty))
            .multi_cartesian_product();

        self.create_ambig_type(
            args.cartesian_product(ret)
                .map(|(args, ret)| mir::Type::Func(mir::FuncType::new(args, ret))),
        )
    }

    /// Returns a type for an array. If the item type is ambiguous, returns an ambiguous
    /// type where the item type is concrete.
    pub fn create_array_type(
        &self,
        item_type: mir::Type,
        len: Option<usize>,
    ) -> mir::Type {
        let types = self.into_possible_types(item_type);
        if types.len() == 1 {
            mir::Type::Array(mir::ArrayType::new(types.into_iter().next().unwrap(), len))
        } else {
            self.create_ambig_type(
                types
                    .into_iter()
                    .map(|ty| mir::Type::Array(mir::ArrayType::new(ty, len))),
            )
        }
    }

    /// Returns an vector listing all the possible types of a type. If the type is not
    /// ambiguous, returns an iterator with only the type itself.
    pub fn possible_types<'a>(&self, ty: &'a mir::Type) -> Vec<&'a mir::Type> {
        if let mir::Type::Infer(v) = &ty {
            if v.types.is_empty() {
                return vec![ty];
            }
        }
        match ty {
            mir::Type::Infer(v) => v.types.iter().collect(),
            _ => vec![ty],
        }
    }

    /// Returns an vector listing all the possible types of a type. If the type is not
    /// ambiguous, returns an iterator with only the type itself.
    pub fn into_possible_types(&self, ty: mir::Type) -> Vec<mir::Type> {
        if let mir::Type::Infer(v) = &ty {
            if v.types.is_empty() {
                return vec![ty];
            }
        }
        match ty {
            mir::Type::Infer(v) => v.types,
            _ => vec![ty],
        }
    }

    /// Merges a list of types into a single type. If the types are incompatible, returns
    /// None.
    pub fn merge_types<'a>(
        &self,
        types: impl IntoIterator<Item = &'a mir::Type> + std::fmt::Debug,
    ) -> Option<mir::Type> {
        let mut iter = types.into_iter();
        let mut res_type = iter.next()?.clone();

        for ty in iter {
            if &res_type == ty || ty.is_unknown() {
                continue;
            }

            res_type = match (&res_type, ty) {
                (a @ mir::Type::Infer(_), b) | (a, b @ mir::Type::Infer(_)) => {
                    let mut props = match a {
                        mir::Type::Infer(v) => v.properties.clone(),
                        _ => vec![],
                    };
                    props.extend(match b {
                        mir::Type::Infer(v) => v.properties.clone(),
                        _ => vec![],
                    });

                    let has_props = |ty: &mir::Type| {
                        props.iter().all(|(prop_name, prop_type)| {
                            self.type_has_prop(ty, prop_name, prop_type)
                        })
                    };

                    let a_types = match a {
                        mir::Type::Infer(v) => v.types.iter().collect(),
                        _ => vec![a],
                    };
                    let b_types = match b {
                        mir::Type::Infer(v) => v.types.iter().collect(),
                        _ => vec![b],
                    };

                    let types: HashSet<_> = if a_types.is_empty() && b_types.is_empty() {
                        [mir::Type::Infer(mir::InferType::new([], props.clone()))].into()
                    } else if a_types.is_empty() {
                        b_types.into_iter().cloned().filter(has_props).collect()
                    } else if b_types.is_empty() {
                        a_types.into_iter().cloned().filter(has_props).collect()
                    } else {
                        a_types
                            .iter()
                            .flat_map(|a| {
                                b_types.iter().filter_map(|b| self.merge_types([*a, *b]))
                            })
                            .filter(has_props)
                            .collect()
                    };

                    if types.is_empty() {
                        return None;
                    }

                    if types.len() == 1 {
                        types.into_iter().next().unwrap()
                    } else {
                        mir::Type::Infer(mir::InferType::new(types, props))
                    }
                }
                (mir::Type::String(a), mir::Type::String(b)) => {
                    let len = match (a.len, b.len) {
                        (Some(a_len), Some(b_len)) => {
                            if a_len != b_len {
                                return None;
                            }
                            Some(a_len)
                        }
                        (Some(len), None) | (None, Some(len)) => Some(len),
                        (None, None) => None,
                    };

                    mir::Type::String(mir::StringType { len })
                }
                (mir::Type::Array(a), mir::Type::Array(b)) => {
                    let len = match (a.len, b.len) {
                        (Some(a_len), Some(b_len)) => {
                            if a_len != b_len {
                                return None;
                            }
                            Some(a_len)
                        }
                        (Some(len), None) | (None, Some(len)) => Some(len),
                        (None, None) => None,
                    };

                    let item = self.merge_types([a.item.as_ref(), b.item.as_ref()])?;

                    self.create_array_type(item, len)
                }
                (mir::Type::Func(a), mir::Type::Func(b)) => {
                    if a.params.len() != b.params.len() || a.ret.len() != b.ret.len() {
                        return None;
                    }

                    let mut params = Vec::with_capacity(a.params.len());
                    let mut ret = Vec::with_capacity(a.ret.len());

                    for (a, b) in izip!(&a.params, &b.params) {
                        params.push(self.merge_types([a, b])?);
                    }

                    for (a, b) in izip!(&a.ret, &b.ret) {
                        ret.push(self.merge_types([a, b])?);
                    }

                    self.create_func_type(params, ret)
                }
                _ => {
                    return None;
                }
            };
        }

        Some(res_type)
    }

    /// Returns true if all the types are the same or are supertype/subtype of each other.
    pub fn match_types<'a>(
        &self,
        types: impl IntoIterator<Item = &'a mir::Type> + std::fmt::Debug,
    ) -> bool {
        self.merge_types(types).is_some()
    }

    /// Returns true if the type is known to have a certain property.
    pub fn type_has_prop(
        &self,
        ty: &mir::Type,
        prop_name: &str,
        prop_type: &mir::Type,
    ) -> bool {
        match self.get_prop_type(ty, prop_name) {
            Some(t) => self.match_types([&t, prop_type]),
            None => false,
        }
    }

    /// Returns the type of a property of a type.
    pub fn get_prop_type(&self, ty: &mir::Type, prop_name: &str) -> Option<mir::Type> {
        match ty {
            mir::Type::Infer(v) => {
                for (name, ty) in &v.properties {
                    if prop_name == name {
                        return Some(ty.clone());
                    }
                }
                let types: HashSet<_> = v
                    .types
                    .iter()
                    .filter_map(|t| self.get_prop_type(t, prop_name))
                    .flat_map(|t| {
                        self.possible_types(&t)
                            .into_iter()
                            .cloned()
                            .collect::<Vec<_>>()
                    })
                    .collect();
                if types.len() == 0 {
                    None
                } else if types.len() == 1 {
                    types.into_iter().next()
                } else {
                    Some(mir::Type::Infer(mir::InferType::new(types, [])))
                }
            }
            _ => {
                let (_, field_ty) = self.get_type_field(ty, prop_name)?;
                Some(field_ty)
            }
        }
    }

    /// Returns the type and index of a field of a type. If the field index cannot be
    /// precisely known yet, returns None.
    pub fn get_type_field(
        &self,
        ty: &mir::Type,
        field_name: &str,
    ) -> Option<(u32, mir::Type)> {
        match ty {
            mir::Type::TypeRef(v) => {
                let Some(typedef) = self.typedef(*v) else {
                    return None;
                };
                let mir::TypeDefBody::Record(rec_type) = &typedef.body else {
                    return None;
                };
                for (i, field) in rec_type.fields.iter().enumerate() {
                    if field_name == &field.name {
                        return Some((i as u32, field.ty.clone()));
                    }
                }
                None
            }
            _ => None,
        }
    }
}
