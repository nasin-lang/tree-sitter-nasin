use std::fmt::{Display, Write};

pub fn indented<T: Display, I: IntoIterator<Item = T>>(n: usize, items: I) -> String {
    let indent = " ".repeat(n);
    let mut buf = String::new();

    for (i, item) in items.into_iter().enumerate() {
        for (j, line) in item.to_string().lines().enumerate() {
            if i > 0 || j > 0 {
                write!(buf, "\n").unwrap();
            }
            write!(buf, "{}{}", &indent, line).unwrap();
        }
    }

    buf
}
