use std::fmt::{self, Display, Write};
use std::hash::{Hash, Hasher};

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

pub mod m_ir {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/torvo.m_ir.rs"));

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

    impl Display for Module {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "module \"{}\":", self.name,)?;

            for (i, global) in self.data.iter().enumerate() {
                write!(f, "\n  global {}:", i)?;

                if let Some(Export { name }) = &global.export {
                    write!(f, " (export \"{}\")", name)?;
                }

                write!(f, " {}", global.r#type)?;
            }

            for (i, func) in self.funcs.iter().enumerate() {
                write!(f, "\n  func {}:", i)?;

                if let Some(Export { name }) = &func.export {
                    write!(f, " (export \"{}\")", name)?;
                }

                write!(f, " (params")?;
                for param in &func.params {
                    write!(f, " {}", param.r#type)?;
                }
                write!(f, ")")?;

                write!(f, " (returns")?;
                for ret in &func.ret {
                    write!(f, " {}", ret)?;
                }
                write!(f, ")")?;

                for (i, local) in func.locals.iter().enumerate() {
                    write!(f, "\n       %{}: {}", i, local.r#type)?;
                }

                write!(f, "\n{}", indented(4, &func.body))?;
            }

            if let Some(init) = &self.init {
                write!(f, "\n  init: ")?;

                for (i, local) in init.locals.iter().enumerate() {
                    if i > 0 {
                        write!(f, "\n        ")?;
                    }
                    write!(f, "%{}: {}", i, local.r#type)?;
                }

                write!(f, "\n{}", indented(4, &init.body))?;
            }

            Ok(())
        }
    }

    impl Display for Instr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.instr {
                Some(instr::Instr::Const(v)) => {
                    write!(f, "%{} = const ", v.target_idx)?;
                    match &v.value {
                        Some(r#const::Value::Number(num)) => {
                            write!(f, "{}", num)?;
                        }
                        None => {}
                    }
                }
                Some(instr::Instr::LoadGlobal(v)) => {
                    write!(
                        f,
                        "%{} = load_global <global {}>",
                        v.target_idx, v.global_idx
                    )?;
                }
                Some(instr::Instr::StoreGlobal(v)) => {
                    write!(f, "store_global <global {}>, {}", v.global_idx, v.value)?;
                }
                Some(instr::Instr::BinOp(op)) => {
                    write!(
                        f,
                        "%{} = bin_op{} {}, {}",
                        op.target_idx,
                        match op.op() {
                            BinOpType::Add => "+",
                            BinOpType::Sub => "-",
                            BinOpType::Mod => "%",
                            BinOpType::Mul => "*",
                            BinOpType::Div => "/",
                            BinOpType::Pow => "**",
                        },
                        op.left,
                        op.right
                    )?;
                }
                Some(instr::Instr::FnCall(call)) => {
                    write!(f, "%{} = fn_call <func {}>", call.target_idx, call.func_idx,)?;
                    for arg in &call.args {
                        write!(f, ", {}", arg)?;
                    }
                }
                Some(instr::Instr::FnReturn(ret)) => {
                    write!(f, "return {}", ret)?;
                }
                None => {}
            }
            Ok(())
        }
    }

    impl Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.value {
                Some(value::Value::Local(idx)) => {
                    write!(f, "%{}", idx)?;
                }
                Some(value::Value::Param(idx)) => {
                    write!(f, "<param {}>", idx)?;
                }
                None => {}
            }
            Ok(())
        }
    }

    impl Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.r#type.as_ref() {
                Some(r#type::Type::Unknown(_)) => {
                    write!(f, "unknown")?;
                }
                Some(r#type::Type::Primitive(prim)) => {
                    write!(f, "{}", PrimType::try_from(*prim).unwrap())?;
                }
                Some(r#type::Type::Fn(fn_type)) => {
                    write!(f, "(func (params")?;

                    for arg in &fn_type.args {
                        write!(f, " {}", arg)?;
                    }

                    write!(f, ") (returns")?;
                    for ret in &fn_type.ret {
                        write!(f, "{}", ret)?;
                    }

                    write!(f, "))")?;
                }
                Some(r#type::Type::Ambig(ambig)) => {
                    write!(f, "(ambig")?;
                    for t in &ambig.types {
                        write!(f, " {}", t)?;
                    }
                    write!(f, ")")?;
                }
                None => {}
            }
            Ok(())
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
