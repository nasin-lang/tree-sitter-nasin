static SRC: &str = r#"
fn sum(a b): i32 =
    a + b

foo: i32 =
    1 + 2 - 3 * 4

main =
    a = foo
    b = 1 ** 2 / (3 + 4);
    c = 1 + sum(
        sum(2, 3)
        4 % 5
    )
    a * b + c
"#;

pub fn toy() {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_torvo::language()).unwrap();

    let tree = parser.parse(SRC, None).unwrap();
    println!("{}", tree.root_node().to_sexp());
}
