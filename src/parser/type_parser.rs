use std::collections::HashMap;

use tree_sitter as ts;

use crate::bytecode as b;
use crate::utils::TreeSitterUtils;

pub struct TypeParser<'a> {
    pub typedefs: Vec<b::TypeDef>,
    pub idents: HashMap<&'a str, b::Type>,
    src: &'a str,
}

impl<'a> TypeParser<'a> {
    pub fn new(src: &'a str) -> Self {
        let idents = HashMap::from([
            ("bool", b::Type::Bool),
            ("i8", b::Type::I8),
            ("i16", b::Type::I16),
            ("i32", b::Type::I32),
            ("i64", b::Type::I64),
            ("u8", b::Type::U8),
            ("u16", b::Type::U16),
            ("u32", b::Type::U32),
            ("u64", b::Type::U64),
            ("usize", b::Type::USize),
            ("f32", b::Type::F32),
            ("f64", b::Type::F64),
            ("str", b::Type::String(b::StringType { len: None })),
        ]);
        TypeParser {
            src,
            typedefs: vec![],
            idents,
        }
    }

    pub fn parse_type(&self, node: ts::Node<'a>) -> b::Type {
        match node.kind() {
            "ident" => {
                let ident = node.get_text(self.src);
                match self.idents.get(ident) {
                    Some(ty) => ty.clone(),
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
                b::Type::Array(b::ArrayType::new(item_ty.into(), len))
            }
            k => panic!("Found unexpected type `{}`", k),
        }
    }

    pub fn add_type(&mut self, name: &'a str, node: ts::Node<'a>) {
        assert_eq!(node.kind(), "type_decl");

        let body_node = node.required_field("body");
        let fields = match body_node.kind() {
            "record_type" => body_node
                .iter_field("fields")
                .map(|field_node| {
                    (
                        field_node
                            .required_field("name")
                            .get_text(self.src)
                            .to_string(),
                        b::RecordTypeField {
                            ty: self.parse_type(field_node.required_field("type")),
                        },
                    )
                })
                .collect(),
            v => panic!("Unexpected type body kind: {v}"),
        };

        self.typedefs.push(b::TypeDef {
            body: b::TypeDefBody::Record(b::RecordType { fields }),
        });
        self.idents
            .insert(name, b::Type::TypeRef(self.idents.len() as u16));
    }
}
