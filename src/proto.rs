use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

struct Indent<T: Display>(T);

impl<T> Display for Indent<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in format!("{}", self.0).lines().enumerate() {
            if i > 0 {
                write!(f, "\n")?;
            }
            write!(f, "    {}", line)?;
        }
        Ok(())
    }
}

pub mod lex {
    use std::iter::zip;

    use super::*;

    include!(concat!(env!("OUT_DIR"), "/torvo.lex.rs"));

    impl Eq for Type {}

    impl Hash for Type {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match &self.r#type {
                Some(r#type::Type::Unknown(_)) => {
                    "Unknown".hash(state);
                }
                Some(r#type::Type::Primitive(prim)) => {
                    "Primitive".hash(state);
                    prim.hash(state);
                }
                Some(r#type::Type::Fn(fn_type)) => {
                    "Fn".hash(state);
                    fn_type.args.hash(state);
                    fn_type.ret.hash(state);
                }
                Some(r#type::Type::Ambig(ambig)) => {
                    "Ambig".hash(state);
                    ambig.types.hash(state);
                }
                None => {
                    "".hash(state);
                }
            };
        }
    }

    fn write_scope<T: Display>(f: &mut fmt::Formatter<'_>, scope: &[T]) -> fmt::Result {
        for instr in scope.iter() {
            write!(f, "{}\n", Indent(instr))?;
        }
        Ok(())
    }

    impl Display for Module {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "mod {}\n", self.name)?;
            write_scope(f, &self.symbols)
        }
    }

    impl Display for Symbol {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.symbol {
                Some(symbol::Symbol::FnDecl(func)) => {
                    write!(f, "fn {}(", func.name)?;

                    for (i, (arg, arg_ty)) in
                        zip(func.args.iter(), func.r#type.args.iter()).enumerate()
                    {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {}", arg, arg_ty)?;
                    }

                    write!(f, "): ")?;

                    for (i, ret) in func.r#type.ret.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", ret)?;
                    }

                    write!(f, " =\n")?;

                    write_scope(f, &func.body)
                }
                Some(symbol::Symbol::DataDecl(data)) => {
                    write!(f, "{}: {} =\n", data.name, data.r#type)?;
                    write_scope(f, &data.body)
                }
                None => Ok(()),
            }
        }
    }

    impl Display for Instr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.instr {
                Some(instr::Instr::Assign(assign)) => {
                    write!(f, "{}: {} = {}", assign.name, assign.r#type, assign.value)
                }
                Some(instr::Instr::BinOp(op)) => write!(
                    f,
                    "{}: {} = {} {} {}",
                    op.name,
                    op.r#type,
                    op.left,
                    match op.op() {
                        BinOpType::Add => "+",
                        BinOpType::Sub => "-",
                        BinOpType::Mod => "%",
                        BinOpType::Mul => "*",
                        BinOpType::Div => "/",
                        BinOpType::Pow => "**",
                    },
                    op.right
                ),
                Some(instr::Instr::FnCall(call)) => write!(
                    f,
                    "{}: {} = {}({})",
                    call.name,
                    call.r#type,
                    call.callee,
                    call.args
                        .iter()
                        .map(|arg| format!("{}", arg))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                Some(instr::Instr::FnReturn(ret)) => {
                    write!(f, "return {}", ret)
                }
                Some(instr::Instr::BodyReturn(ret)) => {
                    write!(f, "{}", ret)
                }
                None => Ok(()),
            }
        }
    }

    impl Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.value {
                Some(value::Value::Num(num)) => {
                    write!(f, "{}", num)?;
                }
                Some(value::Value::Ident(ident)) => {
                    write!(f, "{}", ident)?;
                }
                None => {}
            }
            Ok(())
        }
    }

    impl Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.r#type.as_ref() {
                Some(r#type::Type::Unknown(_)) => write!(f, "unknown"),
                Some(r#type::Type::Primitive(prim)) => {
                    write!(f, "{}", PrimType::try_from(*prim).unwrap())
                }
                Some(r#type::Type::Fn(fn_type)) => {
                    write!(f, "fn(")?;

                    for (i, arg) in fn_type.args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg)?;
                    }

                    write!(f, "): (")?;

                    for (i, ret) in fn_type.ret.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", ret)?;
                    }

                    write!(f, ")")
                }
                Some(r#type::Type::Ambig(ambig)) => {
                    write!(f, "(any of ")?;
                    for (i, t) in ambig.types.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", t)?;
                    }
                    write!(f, ")")
                }
                None => Ok(()),
            }
        }
    }

    impl Display for PrimType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PrimType::I8 => write!(f, "i8"),
                PrimType::I16 => write!(f, "i16"),
                PrimType::I32 => write!(f, "i32"),
                PrimType::I64 => write!(f, "i64"),
                PrimType::U8 => write!(f, "u8"),
                PrimType::U16 => write!(f, "u16"),
                PrimType::U32 => write!(f, "u32"),
                PrimType::U64 => write!(f, "u64"),
                PrimType::USize => write!(f, "usize"),
                PrimType::F32 => write!(f, "f32"),
                PrimType::F64 => write!(f, "f64"),
                PrimType::Char => write!(f, "char"),
                PrimType::Bool => write!(f, "bool"),
            }
        }
    }
}
