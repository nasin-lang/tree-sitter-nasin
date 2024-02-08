fn main() {
    protobuf_codegen::Codegen::new()
        .includes(&["proto"])
        .input("proto/ast.proto")
        .input("proto/m_ir.proto")
        .out_dir("src/proto")
        .run_from_script();
}
