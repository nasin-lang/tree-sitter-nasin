
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ParserValue {
    Func(usize),
    Global(usize),
    Local(usize),
    MovedLocal,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<ParserValue>),
    Record(Vec<(String, ParserValue)>),
}
