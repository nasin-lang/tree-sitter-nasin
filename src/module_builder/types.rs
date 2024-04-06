use std::collections::hash_map::RandomState;
use std::collections::HashSet;

use crate::proto::mir;

macro_rules! primitive {
    ($name:ident) => {
        mir::Ty {
            ty: Some(mir::ty::Ty::Primitive(mir::PrimType::$name.into())),
        }
    };
}
use itertools::Itertools;
pub(crate) use primitive;

/// Returns a instance of the unknown type.
pub fn unknown_type() -> mir::Ty {
    mir::Ty {
        ty: Some(mir::ty::Ty::Unknown(true)),
    }
}

/// Returns an ambiguous type with the given types. If there is only one type, returns that type
/// instead. If no types are given, returns an unknown type.
pub fn ambig<I>(types: I) -> mir::Ty
where
    I: IntoIterator<Item = mir::Ty>,
{
    let types: HashSet<_, RandomState> = types.into_iter().collect();

    if types.len() == 1 {
        return types.into_iter().next().unwrap();
    }

    if types.is_empty() {
        return unknown_type();
    }

    mir::Ty {
        ty: Some(mir::ty::Ty::Ambig(mir::AmbigType {
            types: types.into_iter().collect(),
        })),
    }
}

/// Returns the type of a number literal. Most of the time, this will be a ambiguous type, including
/// all possible types that the number can be parsed as.
pub fn num_type(_num: &str) -> mir::Ty {
    // Simplified for testing
    ambig([primitive!(I32), primitive!(I64)])
    // let is_float = num.contains('.');
    // let is_negative = num.starts_with('-');

    // if is_float {
    //     ambig([primitive!(F32), primitive!(F64)])
    // } else if is_negative {
    //     ambig([
    //         primitive!(I8),
    //         primitive!(I16),
    //         primitive!(I32),
    //         primitive!(I64),
    //         primitive!(F32),
    //         primitive!(F64),
    //     ])
    // } else {
    //     ambig([
    //         primitive!(U8),
    //         primitive!(U16),
    //         primitive!(U32),
    //         primitive!(U64),
    //         primitive!(USize),
    //         primitive!(I8),
    //         primitive!(I16),
    //         primitive!(I32),
    //         primitive!(I64),
    //         primitive!(F32),
    //         primitive!(F64),
    //     ])
    // }
}

/// Returns a function type for a binary operation with the given type. For this to work, the type
/// must be a absolute type, not a ambiguous or unknown type.
pub fn binop_sig(ty: &mir::Ty) -> mir::FuncType {
    mir::FuncType {
        args: vec![ty.clone(), ty.clone()],
        ret: vec![ty.clone()],
    }
}

/// Returns a type for a function. If any of the arguments or the return type is ambiguous, returns
/// an ambigous type for all combinations of the function signature.
pub fn fn_type<A, R>(args: A, ret: R) -> mir::Ty
where
    A: IntoIterator<Item = mir::Ty>,
    R: IntoIterator<Item = mir::Ty>,
{
    let args = args
        .into_iter()
        .map(|ty| into_types_iter(ty))
        .multi_cartesian_product();
    let ret = ret
        .into_iter()
        .map(|ty| into_types_iter(ty))
        .multi_cartesian_product();

    ambig(args.cartesian_product(ret).map(|(args, ret)| mir::Ty {
        ty: Some(mir::ty::Ty::Func(mir::FuncType { args, ret })),
    }))
}

/// Returns true if all the types are the same or are supertype/subtype of each other.
pub fn match_types<'a, I>(types: I) -> bool
where
    I: IntoIterator<Item = &'a mir::Ty>,
{
    merge_types(types).is_some()
}

/// Returns an vector listing all the possible types of a type. If the type is not
/// ambiguous, returns an iterator with only the type itself.
pub fn possible_types<'a>(ty: &'a mir::Ty) -> Vec<&'a mir::Ty> {
    match &ty.ty {
        Some(mir::ty::Ty::Ambig(ambig)) => ambig.types.iter().collect::<Vec<_>>(),
        _ => vec![ty],
    }
}

pub fn into_types_iter(ty: mir::Ty) -> std::vec::IntoIter<mir::Ty> {
    match ty.ty {
        Some(mir::ty::Ty::Ambig(ambig)) => ambig.types.into_iter(),
        _ => vec![ty].into_iter(),
    }
}

/// Merges a list of types into a single type. If the types are incompatible, returns None.
pub fn merge_types<'a, I>(types: I) -> Option<mir::Ty>
where
    I: IntoIterator<Item = &'a mir::Ty>,
{
    let types = types.into_iter().map(possible_types);
    let ambig_types: Vec<_> = types
        .multi_cartesian_product()
        .filter_map(|types| {
            let mut result = vec![unknown_type()];

            for a in types {
                result = result
                    .iter()
                    .filter_map(|b| {
                        if eq_types(a, b) {
                            return Some(vec![a.clone()].into_iter());
                        }

                        if let Some(mir::ty::Ty::Unknown(_)) = a.ty {
                            return Some(vec![b.clone()].into_iter());
                        }

                        if let Some(mir::ty::Ty::Unknown(_)) = b.ty {
                            return Some(vec![a.clone()].into_iter());
                        }

                        if let (Some(mir::ty::Ty::Func(a)), Some(mir::ty::Ty::Func(b))) =
                            (&a.ty, &b.ty)
                        {
                            if a.args.len() != b.args.len() || a.ret.len() != b.ret.len() {
                                return None;
                            }

                            let mut args = Vec::with_capacity(a.args.len());
                            let mut ret = Vec::with_capacity(a.ret.len());

                            for (a, b) in a.args.iter().zip(b.args.iter()) {
                                args.push(merge_types(vec![a, b])?);
                            }

                            for (a, b) in a.ret.iter().zip(b.ret.iter()) {
                                ret.push(merge_types(vec![a, b])?);
                            }

                            return Some(into_types_iter(fn_type(args, ret)));
                        }

                        None
                    })
                    .flatten()
                    .collect();

                if result.is_empty() {
                    return None;
                }
            }

            Some(result.into_iter())
        })
        .flatten()
        .collect();

    if ambig_types.is_empty() {
        return None;
    }

    Some(ambig(ambig_types))
}

/// Checks is two types are equivalent. This only yields true if the types are the same absolute
/// type or if they are ambiguous types with the same types.
/// Obs: This is not Eq only because Eq is already implemented for all protobuf types, so I can't
/// implement it for m_ir::AmbigType so it ignores the order of the types.
pub fn eq_types(a: &mir::Ty, b: &mir::Ty) -> bool {
    if let (Some(mir::ty::Ty::Unknown(_)), Some(mir::ty::Ty::Unknown(_))) = (&a.ty, &b.ty) {
        return true;
    }
    if let Some(mir::ty::Ty::Unknown(_)) = a.ty {
        return false;
    }
    if let Some(mir::ty::Ty::Unknown(_)) = b.ty {
        return false;
    }

    let a_types: HashSet<_, RandomState> = possible_types(a).into_iter().collect();
    let b_types: HashSet<_, RandomState> = possible_types(b).into_iter().collect();

    a_types == b_types
}
