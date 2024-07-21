use crate::utils::SortedMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ParserValue {
    Func(usize),
    Global(usize),
    Local(usize),
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<ParserValue>),
    Record(SortedMap<String, ParserValue>),
    Never,
}

impl ParserValue {
    pub fn is_never(&self) -> bool {
        matches!(self, ParserValue::Never)
    }
}
