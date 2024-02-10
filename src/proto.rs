pub mod ast {
    include!(concat!(env!("OUT_DIR"), "/torvo.ast.rs"));
}

pub mod m_ir {
    include!(concat!(env!("OUT_DIR"), "/torvo.m_ir.rs"));
}
