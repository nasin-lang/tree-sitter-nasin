use derive_new::new;

use crate::bytecode as b;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ValueBody {
    Func(usize, usize),
    Global(usize, usize),
    Local(usize),
    Bool(bool),
    Number(String),
    Never,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, new)]
pub struct Value {
    pub body: ValueBody,
    pub loc: b::Loc,
}
impl Value {
    pub fn is_never(&self) -> bool {
        matches!(&self.body, ValueBody::Never)
    }
    pub fn with_loc(&self, loc: b::Loc) -> Self {
        let mut new = self.clone();
        new.loc = loc;
        new
    }
}
