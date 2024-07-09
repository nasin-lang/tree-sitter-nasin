use std::collections::HashMap;

use crate::mir;

pub fn builtin_types() -> HashMap<String, mir::Type> {
    HashMap::from([
        ("bool".to_string(), mir::Type::Bool),
        ("i8".to_string(), mir::Type::I8),
        ("i16".to_string(), mir::Type::I16),
        ("i32".to_string(), mir::Type::I32),
        ("i64".to_string(), mir::Type::I64),
        ("u8".to_string(), mir::Type::U8),
        ("u16".to_string(), mir::Type::U16),
        ("u32".to_string(), mir::Type::U32),
        ("u64".to_string(), mir::Type::U64),
        ("usize".to_string(), mir::Type::USize),
        ("f32".to_string(), mir::Type::F32),
        ("f64".to_string(), mir::Type::F64),
        (
            "str".to_string(),
            mir::Type::String(mir::StringType { len: None }),
        ),
    ])
}
