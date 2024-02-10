fn main() {
    prost_build::compile_protos(&["proto/ast.proto", "proto/lex.proto"], &["proto"])
        .expect("Failed to compile protos");
}
