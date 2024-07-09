pub fn encode_string_lit(s: &str) -> String {
    let lit = s
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t");
    format!("\"{lit}\"")
}

pub fn decode_string_lit(lit: &str) -> String {
    lit.replace("\\\"", "\"")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\\\", "\\")
}
