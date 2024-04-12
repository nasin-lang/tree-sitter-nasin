use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Write};

use itertools::{izip, Itertools};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub name: String,
    pub globals: Vec<Global>,
    pub funcs: Vec<Func>,
    pub init: Option<ModuleInit>,
}

impl Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module \"{}\":", self.name,)?;

        for (i, global) in self.globals.iter().enumerate() {
            write!(f, "\n  global {}:", i)?;

            if let Some(Export { name }) = &global.export {
                write!(f, " (export \"{}\")", name)?;
            }

            write!(f, " {}", global.ty)?;

            if let Some(value) = &global.value {
                write!(f, " {}", value)?;
            }
        }

        for (i, func) in self.funcs.iter().enumerate() {
            write!(f, "\n  func {}:", i)?;

            if let Some(Export { name }) = &func.export {
                write!(f, " (export \"{}\")", name)?;
            }

            if let Some(Extern { name }) = &func.extern_ {
                write!(f, " (extern \"{}\")", name)?;
            }

            if func.params.len() > 0 {
                write!(f, " (params")?;
                for param in &func.params {
                    write!(f, " {}", param.ty)?;
                }
                write!(f, ")")?;
            }

            if func.ret.len() > 0 {
                write!(f, " (returns")?;
                for ret in &func.ret {
                    write!(f, " {}", ret)?;
                }
                write!(f, ")")?;
            }

            for (i, local) in func.locals.iter().enumerate() {
                write!(f, "\n       %{}: {}", i, local.ty)?;
            }

            if func.body.len() > 0 {
                write!(f, "\n{}", indented(4, &func.body))?;
            }
        }

        if let Some(init) = &self.init {
            write!(f, "\n  init: ")?;

            for (i, local) in init.locals.iter().enumerate() {
                if i > 0 {
                    write!(f, "\n        ")?;
                }
                write!(f, "%{}: {}", i, local.ty)?;
            }

            write!(f, "\n{}", indented(4, &init.body))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Global {
    pub ty: Type,
    pub value: Option<ConstValue>,
    pub export: Option<Export>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Func {
    pub params: Vec<Param>,
    pub ret: Vec<Type>,
    pub locals: Vec<Local>,
    pub body: Vec<Instr>,
    pub export: Option<Export>,
    pub extern_: Option<Extern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleInit {
    pub locals: Vec<Local>,
    pub body: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Local {
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Export {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extern {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr {
    LoadGlobal(LoadGlobalInstr),
    StoreGlobal(StoreGlobalInstr),
    CreateNumber(CreateNumberInstr),
    CreateData(CreateDataInstr),
    Add(BinOpInstr),
    Sub(BinOpInstr),
    Mul(BinOpInstr),
    Div(BinOpInstr),
    Mod(BinOpInstr),
    Pow(BinOpInstr),
    Call(CallInstr),
    Return(ReturnInstr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadGlobalInstr {
    pub target_idx: u32,
    pub global_idx: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreGlobalInstr {
    pub global_idx: u32,
    pub field_idx: Option<u32>,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateNumberInstr {
    pub target_idx: u32,
    pub number: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateDataInstr {
    pub target_idx: u32,
    pub items: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinOpInstr {
    pub target_idx: u32,
    pub left: Value,
    pub right: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallInstr {
    pub target_idx: u32,
    pub func_idx: u32,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnInstr {
    pub value: Option<Value>,
}

impl Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instr::LoadGlobal(v) => {
                write!(
                    f,
                    "%{} = load_global <global {}>",
                    v.target_idx, v.global_idx
                )?;
            }
            Instr::StoreGlobal(v) => {
                write!(f, "store_global <global {}", v.global_idx)?;
                if let Some(field_idx) = v.field_idx {
                    write!(f, ".{}", field_idx)?;
                }
                write!(f, ">, {}", v.value)?;
            }
            Instr::CreateNumber(v) => {
                write!(f, "%{} = create_number {}", v.target_idx, &v.number)?;
            }
            Instr::CreateData(v) => {
                write!(f, "%{} = create_data", v.target_idx)?;
                for item in &v.items {
                    write!(f, ", {}", item)?;
                }
            }
            Instr::Add(v) => {
                write!(f, "%{} = add {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Sub(v) => {
                write!(f, "%{} = sub {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Mul(v) => {
                write!(f, "%{} = mul {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Div(v) => {
                write!(f, "%{} = div {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Mod(v) => {
                write!(f, "%{} = mod {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Pow(v) => {
                write!(f, "%{} = pow {}, {}", v.target_idx, v.left, v.right)?;
            }
            Instr::Call(v) => {
                write!(f, "%{} = call <func {}>", v.target_idx, v.func_idx,)?;
                for arg in &v.args {
                    write!(f, ", {}", arg)?;
                }
            }
            Instr::Return(ret) => {
                write!(f, "return")?;
                if let Some(value) = &ret.value {
                    write!(f, " {}", value)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstValue {
    Number(String),
    Array(Vec<ConstValue>),
}

impl Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ConstValue::Number(num) => {
                write!(f, "{}", num)?;
            }
            ConstValue::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Local(u32),
    Param(u32),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Value::Local(idx) => {
                write!(f, "%{}", idx)?;
            }
            Value::Param(idx) => {
                write!(f, "<param {}>", idx)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Unknown,
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
    Ambig(AmbigType),
    Array(ArrayType),
}

impl Type {
    /// Returns the type of a number literal. Most of the time, this will be a ambiguous
    /// type, including all possible types that the number can be parsed as.
    pub fn num_type(num: &str) -> Self {
        let is_float = num.contains('.');
        let is_negative = num.starts_with('-');

        if is_float {
            Type::ambig([Self::F32, Self::F64])
        } else if is_negative {
            Type::ambig([
                Self::I8,
                Self::I16,
                Self::I32,
                Self::I64,
                Self::F32,
                Self::F64,
            ])
        } else {
            Type::ambig([
                Self::U8,
                Self::U16,
                Self::U32,
                Self::U64,
                Self::USize,
                Self::I8,
                Self::I16,
                Self::I32,
                Self::I64,
                Self::F32,
                Self::F64,
            ])
        }
    }

    /// Returns an ambiguous type with the given types. If there is only one type, returns
    /// that type instead. If no types are given, returns an unknown type.
    pub fn ambig<I>(types: I) -> Self
    where
        I: IntoIterator<Item = Type>,
    {
        let ambig = AmbigType::new(types);

        if ambig.types.len() == 1 {
            return ambig.types[0].clone();
        }

        if ambig.types.is_empty() {
            return Self::Unknown;
        }

        Self::Ambig(ambig)
    }

    /// Returns a type for a function. If any of the arguments or the return type is
    /// ambiguous, returns an ambiguous type for all combinations of the function
    /// signature.
    pub fn func_type<A, R>(args: A, ret: R) -> Self
    where
        A: IntoIterator<Item = Type>,
        R: IntoIterator<Item = Type>,
    {
        let args = args
            .into_iter()
            .map(|ty| ty.into_possible_types())
            .multi_cartesian_product();
        let ret = ret
            .into_iter()
            .map(|ty| ty.into_possible_types())
            .multi_cartesian_product();

        Self::ambig(
            args.cartesian_product(ret)
                .map(|(args, ret)| Type::Func(FuncType::new(args, ret))),
        )
    }

    /// Returns a type for a array. If the item type is ambiguous, returns an ambiguous
    /// type where the item type is concrete.
    pub fn array_type(item_type: Type, len: Option<usize>) -> Self {
        match item_type {
            Type::Ambig(ambig) => Type::ambig(
                ambig
                    .types
                    .into_iter()
                    .map(|ty| Type::Array(ArrayType::new(ty, len))),
            ),
            _ => Type::Array(ArrayType::new(item_type, len)),
        }
    }

    /// Returns true if all the types are the same or are supertype/subtype of each other.
    pub fn matches<'a, I>(types: I) -> bool
    where
        I: IntoIterator<Item = &'a Type>,
    {
        Self::merge(types).is_some()
    }

    /// Merges a list of types into a single type. If the types are incompatible, returns
    /// None.
    pub fn merge<'a, I>(types: I) -> Option<Self>
    where
        I: IntoIterator<Item = &'a Type>,
    {
        let mut iter = types.into_iter();
        let mut res_type = iter.next()?.clone();

        for ty in iter {
            res_type = res_type.merge_with(ty)?;
        }

        Some(res_type)
    }

    /// Returns an vector listing all the possible types of a type. If the type is not
    /// ambiguous, returns an iterator with only the type itself.
    pub fn possible_types(&self) -> Vec<&Self> {
        match &self {
            Type::Ambig(ambig) => ambig.types.iter().collect(),
            _ => vec![&self],
        }
    }

    /// Returns an vector listing all the possible types of a type. If the type is not
    /// ambiguous, returns an iterator with only the type itself.
    pub fn into_possible_types(self) -> Vec<Self> {
        match self {
            Type::Ambig(ambig) => ambig.types,
            _ => vec![self],
        }
    }

    /// Merges with other type into single type. If the types are incompatible, returns
    /// None.
    pub fn merge_with(&self, other: &Self) -> Option<Self> {
        if self == other || other.is_unknown() {
            return Some(self.clone());
        }

        if self.is_unknown() {
            return Some(other.clone());
        }

        if self.is_ambig() || other.is_ambig() {
            let a_types = self.possible_types();
            let b_types = other.possible_types();

            let res_type = Self::ambig(
                a_types
                    .into_iter()
                    .flat_map(|a| b_types.iter().filter_map(|b| a.merge_with(b))),
            );

            if res_type.is_unknown() {
                return None;
            }

            return Some(res_type);
        }

        if let (Type::Array(a), Type::Array(b)) = (&self, &other) {
            let len = match (a.len, b.len) {
                (Some(a_len), Some(b_len)) => {
                    if a_len != b_len {
                        return None;
                    }
                    Some(a_len)
                }
                (Some(len), None) | (None, Some(len)) => Some(len),
                (None, None) => None,
            };

            let item = a.item.merge_with(&b.item)?;

            return Some(Self::array_type(item, len));
        }

        if let (Type::Func(a), Type::Func(b)) = (&self, &other) {
            if a.params.len() != b.params.len() || a.ret.len() != b.ret.len() {
                return None;
            }

            let mut params = Vec::with_capacity(a.params.len());
            let mut ret = Vec::with_capacity(a.ret.len());

            for (a, b) in izip!(&a.params, &b.params) {
                params.push(a.merge_with(b)?);
            }

            for (a, b) in izip!(&a.ret, &b.ret) {
                ret.push(a.merge_with(b)?);
            }

            return Some(Self::func_type(params, ret));
        }

        None
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Type::Unknown)
    }

    pub fn is_ambig(&self) -> bool {
        matches!(self, Type::Ambig(_))
    }

    pub fn is_composite(&self) -> bool {
        matches!(self, Type::Func(_) | Type::Array(_))
    }

    pub fn is_primitive(&self) -> bool {
        !self.is_composite() && !self.is_unknown() && !self.is_ambig()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Type::Unknown => write!(f, "unknown"),
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
            Type::Ambig(v) => {
                write!(f, "(ambig")?;
                for t in &v.types {
                    write!(f, " {}", t)?;
                }
                write!(f, ")")?;
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

    /// Returns a function type for a binary operation with the given type. For this to
    /// work, the type must be a absolute type, not a ambiguous or unknown type.
    pub fn binop_sig(ty: &Type) -> FuncType {
        assert!(!ty.is_unknown());
        assert!(!ty.is_ambig());
        FuncType {
            params: vec![ty.clone(), ty.clone()],
            ret: vec![ty.clone()],
        }
    }

    /// Returns a function type for a array const operation with the given type. For this to
    /// work, the type must be a absolute type, not a ambiguous or unknown type.
    pub fn array_sig(ty: &Type, len: usize) -> FuncType {
        assert!(!ty.is_unknown());
        assert!(!ty.is_ambig());
        FuncType {
            params: (0..len).map(|_| ty.clone()).collect(),
            ret: vec![Type::Array(ArrayType::new(ty.clone(), Some(len)))],
        }
    }
}

#[derive(Debug, Clone, Eq, Hash)]
pub struct AmbigType {
    pub types: Vec<Type>,
}

impl AmbigType {
    pub fn new<I>(types: I) -> Self
    where
        I: IntoIterator<Item = Type>,
    {
        let types: HashSet<_> = types
            .into_iter()
            .flat_map(|ty| ty.into_possible_types())
            .collect();
        Self {
            types: types.into_iter().collect(),
        }
    }
}

impl PartialEq for AmbigType {
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

fn indented<T: Display, I: IntoIterator<Item = T>>(n: usize, items: I) -> String {
    let mut s = String::new();

    for (i, item) in items.into_iter().enumerate() {
        for (j, line) in item.to_string().lines().enumerate() {
            if i > 0 || j > 0 {
                write!(s, "\n").unwrap();
            }
            write!(s, "{}{}", " ".repeat(n), line).unwrap();
        }
    }

    s
}
