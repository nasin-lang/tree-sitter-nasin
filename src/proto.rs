use std::fmt::{self, Display};

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

pub mod ast {
    include!(concat!(env!("OUT_DIR"), "/torvo.ast.rs"));
}

pub mod lex {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/torvo.lex.rs"));

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
                    write!(f, "fn {}({}) =\n", func.name, func.args.join(", "),)?;
                    write_scope(f, &func.body)
                }
                Some(symbol::Symbol::DataDecl(data)) => {
                    write!(f, "{} =\n", data.name)?;
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
                    write!(f, "{} = {}", assign.name, assign.value)
                }
                Some(instr::Instr::BinOp(op)) => write!(
                    f,
                    "{} = {} {} {}",
                    op.name,
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
                    "{} = {}({})",
                    call.name,
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
                Some(value::Value::Num(num)) => write!(f, "{}", num.value),
                Some(value::Value::Ident(ident)) => write!(f, "{}", ident),
                None => Ok(()),
            }
        }
    }
}
