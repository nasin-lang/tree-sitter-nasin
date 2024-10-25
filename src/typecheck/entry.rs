use std::collections::HashSet;

use derive_new::new;

use crate::bytecode as b;
use crate::utils::{number_enum, SortedMap};

pub type TypeCheckEntryIdx = usize;

#[derive(Debug, Clone, new)]
pub struct TypeCheckEntry {
    pub ty: b::Type,
    pub loc: b::Loc,
    #[new(default)]
    pub constraints: HashSet<Constraint>,
    #[new(default)]
    pub same_of: HashSet<TypeCheckEntryIdx>,
}

number_enum!(pub ConstraintPriority: u8 {
    NoType = 0,
    DerivedInferredType = 1,
    DerivedDefinedType = 2,
    DefinedType = 3,
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {
    Is(b::Type),
    TypeOf(TypeCheckEntryIdx),
    Array(TypeCheckEntryIdx),
    Ptr(TypeCheckEntryIdx),
    ReturnOf(TypeCheckEntryIdx),
    ParameterOf(TypeCheckEntryIdx, usize),
    IsProperty(TypeCheckEntryIdx, String),
    Members(SortedMap<String, TypeCheckEntryIdx>),
    HasProperty(String, TypeCheckEntryIdx),
    Func(usize),
}
impl Constraint {
    pub fn priority(&self) -> ConstraintPriority {
        match self {
            Self::Is(..) => ConstraintPriority::DefinedType,
            Self::TypeOf(..)
            | Self::Array(..)
            | Self::Ptr(..)
            | Self::ReturnOf(..)
            | Self::ParameterOf(..)
            | Self::IsProperty(..) => ConstraintPriority::DerivedDefinedType,
            Self::Members(..) | Self::HasProperty(..) | Self::Func(..) => {
                ConstraintPriority::DerivedInferredType
            }
        }
    }
}
