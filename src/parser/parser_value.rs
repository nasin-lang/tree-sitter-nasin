use crate::bytecode as b;

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

impl ParserValue {
    pub fn primitive_value(&self) -> Option<b::PrimitiveValue> {
        let prim_value = match self {
            ParserValue::Bool(v) => b::PrimitiveValue::Bool(*v),
            ParserValue::Number(v) => b::PrimitiveValue::Number(v.clone()),
            _ => {
                return None;
            }
        };
        Some(prim_value)
    }

    pub fn const_value(&self) -> Option<b::ConstValue> {
        let const_value = match self {
            ParserValue::Bool(_) | ParserValue::Number(_) => {
                b::ConstValue::Primitive(self.primitive_value()?)
            }
            ParserValue::String(v) => b::ConstValue::String(v.clone()),
            ParserValue::Array(v) => b::ConstValue::Array(
                v.iter()
                    .map(|item| item.primitive_value())
                    .collect::<Option<Vec<_>>>()?,
            ),
            ParserValue::Record(fields) => {
                let const_fields = fields
                    .iter()
                    .map(|(name, value)| Some((name.clone(), value.primitive_value()?)))
                    .collect::<Option<Vec<_>>>()?;
                b::ConstValue::Record(const_fields)
            }
            _ => {
                return None;
            }
        };
        Some(const_value)
    }
}
