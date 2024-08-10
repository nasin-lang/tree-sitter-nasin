use std::collections::HashSet;

use crate::bytecode as b;

pub type TypeCheckEntryIdx = usize;

#[derive(Debug, Clone)]
pub struct TypeCheckEntry {
    pub ty: b::Type,
    pub constraints: Vec<Constraint>,
    pub same_of: HashSet<TypeCheckEntryIdx>,
}

impl TypeCheckEntry {
    pub fn new(ty: b::Type) -> Self {
        Self {
            ty,
            constraints: vec![],
            same_of: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Is(b::Type),
    TypeOf(TypeCheckEntryIdx),
    Property(String, TypeCheckEntryIdx),
    Array(TypeCheckEntryIdx),
}
