use std::collections::HashMap;

use tree_sitter as ts;

use crate::bytecode as b;
use crate::utils::TreeSitterUtils;

pub struct TypeParser<'a> {
    pub typedefs: Vec<b::TypeDef>,
    pub idents: HashMap<&'a str, b::TypeBody>,
    src: &'a str,
}

impl<'a> TypeParser<'a> {
    pub fn new(src: &'a str) -> Self {
        let idents = HashMap::from([
            ("bool", b::TypeBody::Bool),
            ("i8", b::TypeBody::I8),
            ("i16", b::TypeBody::I16),
            ("i32", b::TypeBody::I32),
            ("i64", b::TypeBody::I64),
            ("u8", b::TypeBody::U8),
            ("u16", b::TypeBody::U16),
            ("u32", b::TypeBody::U32),
            ("u64", b::TypeBody::U64),
            ("usize", b::TypeBody::USize),
            ("f32", b::TypeBody::F32),
            ("f64", b::TypeBody::F64),
            ("str", b::TypeBody::String(b::StringType { len: None })),
        ]);
        TypeParser {
            src,
            typedefs: vec![],
            idents,
        }
    }

    pub fn parse_type(&self, node: ts::Node<'a>) -> b::Type {
        let body = match node.kind() {
            "ident" => {
                let ident = node.get_text(self.src);
                match self.idents.get(ident) {
                    Some(body) => body.clone(),
                    None => {
                        // TODO: improve error handling
                        panic!("Type \"{ident}\" not found");
                    }
                }
            }
            "array_type" => {
                let item_ty = self.parse_type(node.required_field("item_type"));
                let len = node.field("length").map(|n| {
                    n.get_text(self.src)
                        .parse::<usize>()
                        .expect("Cannot cast length to integer")
                });
                b::TypeBody::Array(b::ArrayType::new(item_ty.into(), len))
            }
            k => panic!("Found unexpected type `{}`", k),
        };
        b::Type::new(body, Some(b::Loc::from_node(0, &node)))
    }

    pub fn add_type(&mut self, name: &'a str, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "type_decl");

        let body_node = node.required_field("body");
        let fields = match body_node.kind() {
            "record_type" => body_node
                .iter_field("fields")
                .map(|field_node| {
                    let name_node = field_node.required_field("name");
                    let name = name_node.get_text(self.src).to_string();
                    (
                        name.clone(),
                        b::RecordField::new(
                            b::RecordFieldName::new(
                                name,
                                b::Loc::from_node(0, &name_node),
                            ),
                            self.parse_type(field_node.required_field("type")),
                            b::Loc::from_node(0, &field_node),
                        ),
                    )
                })
                .collect(),
            v => panic!("Unexpected type body kind: {v}"),
        };

        self.typedefs.push(b::TypeDef {
            body: b::TypeDefBody::Record(b::RecordType { fields }),
            loc: b::Loc::from_node(0, &node),
        });
        let type_idx = self.typedefs.len() - 1;
        self.idents.insert(name, b::TypeBody::TypeRef(type_idx));
    }
}
