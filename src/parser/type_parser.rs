use std::collections::HashMap;

use derive_new::new;
use itertools::Itertools;
use tree_sitter as ts;

use crate::utils::{IntoItem, TreeSitterUtils};
use crate::{bytecode as b, context};

#[derive(new)]
pub struct TypeParser<'a> {
    #[new(default)]
    pub typedefs: Vec<b::TypeDef>,
    #[new(value = "default_idents()")]
    pub idents: HashMap<String, b::TypeBody>,
    ctx: &'a context::BuildContext<'a>,
    src_idx: usize,
    mod_idx: usize,
}

impl<'a> TypeParser<'a> {
    pub fn parse_type<'t>(&self, node: ts::Node<'t>) -> b::Type {
        let body = match node.kind() {
            "ident" => {
                let ident = node.get_text(&self.ctx.source(self.src_idx).content().text);
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
                    n.get_text(&self.ctx.source(self.src_idx).content().text)
                        .parse::<usize>()
                        .expect("Cannot cast length to integer")
                });
                b::TypeBody::Array(b::ArrayType::new(item_ty.into(), len))
            }
            "generic_type" => {
                let name = node
                    .required_field("name")
                    .of_kind("ident")
                    .get_text(&self.ctx.source(self.src_idx).content().text);

                let args = node
                    .iter_field("args")
                    .map(|arg_node| self.parse_type(arg_node))
                    .collect_vec();

                match name {
                    "Ptr" => {
                        // TODO: Better error handling
                        assert!(args.len() == 1, "Ptr accepts only one parameter");
                        b::TypeBody::Ptr(args.into_item(0).unwrap().into())
                    }
                    _ => panic!("unhandled generic type: `{name}`"),
                }
            }
            k => panic!("Found unexpected type `{}`", k),
        };
        b::Type::new(body, Some(b::Loc::from_node(self.src_idx, &node)))
    }

    pub fn add_type<'t>(&mut self, name: &'a str, node: ts::Node<'t>) {
        assert_eq!(node.kind(), "type_decl");

        let body_node = node.required_field("body");
        let fields = match body_node.kind() {
            "record_type" => body_node
                .iter_field("fields")
                .map(|field_node| {
                    let name_node = field_node.required_field("name");
                    let name = name_node
                        .get_text(&self.ctx.source(self.src_idx).content().text)
                        .to_string();
                    (
                        name.clone(),
                        b::RecordField::new(
                            b::RecordFieldName::new(
                                name,
                                b::Loc::from_node(self.src_idx, &name_node),
                            ),
                            self.parse_type(field_node.required_field("type")),
                            b::Loc::from_node(self.src_idx, &field_node),
                        ),
                    )
                })
                .collect(),
            v => panic!("Unexpected type body kind: {v}"),
        };

        self.typedefs.push(b::TypeDef {
            body: b::TypeDefBody::Record(b::RecordType { fields }),
            loc: b::Loc::from_node(self.src_idx, &node),
        });
        let type_idx = self.typedefs.len() - 1;
        self.idents.insert(
            name.to_string(),
            b::TypeBody::TypeRef(self.mod_idx, type_idx),
        );
    }
}

fn default_idents() -> HashMap<String, b::TypeBody> {
    HashMap::from([
        ("bool".to_string(), b::TypeBody::Bool),
        ("i8".to_string(), b::TypeBody::I8),
        ("i16".to_string(), b::TypeBody::I16),
        ("i32".to_string(), b::TypeBody::I32),
        ("i64".to_string(), b::TypeBody::I64),
        ("u8".to_string(), b::TypeBody::U8),
        ("u16".to_string(), b::TypeBody::U16),
        ("u32".to_string(), b::TypeBody::U32),
        ("u64".to_string(), b::TypeBody::U64),
        ("usize".to_string(), b::TypeBody::USize),
        ("f32".to_string(), b::TypeBody::F32),
        ("f64".to_string(), b::TypeBody::F64),
        (
            "str".to_string(),
            b::TypeBody::String(b::StringType { len: None }),
        ),
    ])
}
