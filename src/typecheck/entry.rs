use std::collections::HashSet;

use derive_new::new;

use crate::bytecode as b;

pub type TypeCheckEntryIdx = usize;

#[derive(Debug, Clone, new)]
pub struct TypeCheckEntry {
    pub ty: b::Type,
    pub loc: b::Loc,
    #[new(default)]
    pub constraints: Vec<Constraint>,
    #[new(default)]
    pub same_of: HashSet<TypeCheckEntryIdx>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Is(b::Type),
    TypeOf(TypeCheckEntryIdx),
    Property(String, TypeCheckEntryIdx),
    Array(TypeCheckEntryIdx),
    Ptr(TypeCheckEntryIdx),
}
