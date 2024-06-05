use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
    Func(FuncType),
    Infer(InferType),
    String(StringType),
    Array(ArrayType),
    TypeRef(u32),
}

impl Type {
    pub fn unknown() -> Self {
        Type::Infer(InferType {
            types: vec![],
            properties: vec![],
        })
    }

    pub fn is_unknown(&self) -> bool {
        if let Type::Infer(i) = self {
            return i.types.is_empty() && i.properties.is_empty();
        }
        false
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Type::Infer(_))
    }

    pub fn is_composite(&self) -> bool {
        matches!(self, Type::Func(_) | Type::String(_) | Type::Array(_))
    }

    pub fn is_primitive(&self) -> bool {
        self.is_bool() || self.is_number()
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Type::Bool)
    }

    pub fn is_number(&self) -> bool {
        self.is_signed_int() || self.is_unsigned_int() || self.is_float()
    }

    pub fn is_signed_int(&self) -> bool {
        matches!(self, Type::I8 | Type::I16 | Type::I32 | Type::I64)
    }

    pub fn is_unsigned_int(&self) -> bool {
        matches!(
            self,
            Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Type::Bool => write!(f, "bool"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::USize => write!(f, "usize"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::Func(v) => {
                write!(f, "(func")?;

                if v.params.len() > 0 {
                    write!(f, " (params")?;
                    for arg in &v.params {
                        write!(f, " {}", arg)?;
                    }
                    write!(f, ")")?;
                }

                if v.ret.len() > 0 {
                    write!(f, " (returns")?;
                    for ret in &v.ret {
                        write!(f, " {}", ret)?;
                    }
                    write!(f, ")")?;
                }

                write!(f, ")")?;

                Ok(())
            }
            Type::Infer(v) => {
                write!(f, "(infer")?;
                for t in &v.types {
                    write!(f, " {}", t)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Type::String(v) => {
                if let Some(len) = v.len {
                    write!(f, "(string {})", len)?;
                } else {
                    write!(f, "string")?;
                }
                Ok(())
            }
            Type::Array(v) => {
                write!(f, "(array {}", v.item)?;
                if let Some(len) = v.len {
                    write!(f, " {}", len)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Type::TypeRef(i) => write!(f, "(type {i})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncType {
    pub params: Vec<Type>,
    pub ret: Vec<Type>,
}

impl FuncType {
    pub fn new(params: Vec<Type>, ret: Vec<Type>) -> Self {
        Self { params, ret }
    }

    /// Returns a function type that takes a type as argument and returns the same type.
    /// For this to work, the type must be an absolute type, not an ambiguous or unknown
    /// one.
    pub fn id_sig(ty: &Type) -> FuncType {
        assert!(!ty.is_infer());
        assert!(!ty.is_infer());
        FuncType {
            params: vec![ty.clone()],
            ret: vec![ty.clone()],
        }
    }

    /// Returns a function type for a binary operation with the given type. For this to
    /// work, the type must be an absolute type, not an ambiguous or unknown one.
    pub fn binop_sig(operands_ty: &Type, result_ty: &Type) -> FuncType {
        assert!(!operands_ty.is_infer());
        assert!(!operands_ty.is_infer());
        assert!(!result_ty.is_infer());
        assert!(!result_ty.is_infer());
        FuncType {
            params: vec![operands_ty.clone(), operands_ty.clone()],
            ret: vec![result_ty.clone()],
        }
    }

    /// Returns a function type for an array const operation with the given type. For this
    /// to work, the type must be an absolute type, not an ambiguous or unknown one.
    pub fn array_sig(ty: &Type, len: usize) -> FuncType {
        assert!(!ty.is_infer());
        assert!(!ty.is_infer());
        FuncType {
            params: (0..len).map(|_| ty.clone()).collect(),
            ret: vec![Type::Array(ArrayType::new(ty.clone(), Some(len)))],
        }
    }

    /// Returns a function type for an if expression that results in the given type. For
    /// this to work, the type must be an absolute type, not an ambiguous or unknown one.
    pub fn if_sig(ty: &Type) -> FuncType {
        assert!(!ty.is_infer());
        assert!(!ty.is_infer());
        FuncType {
            params: vec![Type::Bool, ty.clone(), ty.clone()],
            ret: vec![ty.clone()],
        }
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct InferType {
    pub types: Vec<Type>,
    pub properties: Vec<(String, Type)>,
}

impl InferType {
    pub fn new(
        types: impl IntoIterator<Item = Type>,
        properties: impl IntoIterator<Item = (String, Type)>,
    ) -> Self {
        Self {
            types: types.into_iter().collect(),
            properties: properties.into_iter().collect(),
        }
    }
}

impl PartialEq for InferType {
    fn eq(&self, other: &Self) -> bool {
        if self.types.len() != other.types.len() {
            return false;
        }

        let a_set: HashSet<_> = self.types.iter().collect();
        let b_set: HashSet<_> = other.types.iter().collect();

        a_set == b_set
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringType {
    pub len: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub item: Box<Type>,
    pub len: Option<usize>,
}

impl ArrayType {
    pub fn new(item: Type, len: Option<usize>) -> Self {
        Self {
            item: Box::new(item),
            len,
        }
    }
}
