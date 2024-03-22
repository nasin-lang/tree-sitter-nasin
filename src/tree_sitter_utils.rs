use tree_sitter as ts;

pub trait TreeSitterUtils<'a> {
    fn of_kind(self, kind: &'a str) -> Self;
    fn get_text(&'a self, source: &'a str) -> &'a str;
    fn iter_children(&'a self) -> impl Iterator<Item = ts::Node<'a>>;
    fn iter_field(&'a self, field: &str) -> impl Iterator<Item = ts::Node<'a>>;
    fn field(&'a self, field: &str) -> Option<ts::Node<'a>>;
    fn required_field(&'a self, field: &str) -> ts::Node<'a>;
}

impl<'a> TreeSitterUtils<'a> for ts::Node<'a> {
    fn of_kind(self, kind: &'a str) -> Self {
        assert!(self.is_named());
        assert_eq!(self.kind(), kind);
        self
    }

    fn get_text(&'a self, source: &'a str) -> &'a str {
        &source[self.start_byte()..self.end_byte()]
    }

    fn iter_children(&'a self) -> impl Iterator<Item = ts::Node<'a>> {
        Children::new(self, None)
    }

    fn iter_field(&'a self, field: &str) -> impl Iterator<Item = ts::Node<'a>> {
        Children::new(self, Some(field.to_string()))
    }

    fn field(&'a self, field: &str) -> Option<ts::Node<'a>> {
        self.iter_field(field).next()
    }

    fn required_field(&'a self, field: &str) -> ts::Node<'a> {
        self.field(field)
            .expect(&format!("Field {} is missing", field))
    }
}

struct Children<'a> {
    field: Option<String>,
    cursor: ts::TreeCursor<'a>,
    finished: bool,
}

impl<'a> Children<'a> {
    fn new(node: &'a ts::Node<'a>, field: Option<String>) -> Self {
        let mut cursor = node.walk();
        let has_children = cursor.goto_first_child();

        Children {
            field,
            cursor,
            finished: !has_children,
        }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = ts::Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip to a valid node
        while !self.finished {
            match (self.cursor.node().is_named(), self.field.as_ref()) {
                (true, Some(field)) if self.cursor.field_name().is_some_and(|f| f == field) => {
                    break
                }
                (true, None) => break,
                _ => {
                    self.finished = !self.cursor.goto_next_sibling();
                }
            }
        }

        if self.finished {
            return None;
        }

        let node = self.cursor.node();

        self.finished = !self.cursor.goto_next_sibling();

        Some(node)
    }
}
