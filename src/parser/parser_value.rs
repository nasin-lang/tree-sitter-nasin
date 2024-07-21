#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ParserValue {
    Func(usize),
    Global(usize),
    Local(usize),
    Bool(bool),
    Number(String),
    Never,
}

impl ParserValue {
    pub fn is_never(&self) -> bool {
        matches!(self, ParserValue::Never)
    }
}
