fn main() {
    prost_build::compile_protos(&["proto/ast.proto", "proto/m_ir.proto"], &["proto"])
        .expect("Failed to compile protos");
    // protobuf_codegen::Codegen::new()
    //     .includes(&["proto"])
    //     .input("proto/ast.proto")
    //     .input("proto/m_ir.proto")
    //     .out_dir("src/proto")
    //     .run_from_script();
}
