use std::borrow::Cow;
use std::path::Path;
use std::{fs, io};

use derive_new::new;
use itertools::Itertools;

#[derive(Debug, new)]
pub struct Sources<'a> {
    #[new(default)]
    pub sources: Vec<Source<'a>>,
}
impl<'a> Sources<'a> {
    pub fn path(&self, idx: usize) -> &Path {
        self.sources.get(idx).expect("source should exist").path
    }
    pub fn content(&self, idx: usize) -> &str {
        self.sources
            .get(idx)
            .expect("source should exist")
            .content
            .as_ref()
    }
    pub fn line(&self, idx: usize, n: usize) -> &str {
        self.sources
            .get(idx)
            .expect("source should exist")
            .line(n)
            .expect("line should exist")
    }
    pub fn insert(&mut self, path: &'a Path, content: &'a str) {
        self.sources.push(Source::new(path, Cow::Borrowed(content)));
    }
    pub fn read(&mut self, path: &'a Path) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.sources.push(Source::new(path, Cow::Owned(content)));
        Ok(())
    }
}

#[derive(Debug)]
pub struct Source<'a> {
    pub path: &'a Path,
    pub content: Cow<'a, str>,
    /// Indexes of each line after the first (which is implied to be at index 0)
    lines: Vec<usize>,
}
impl<'a> Source<'a> {
    fn new(path: &'a Path, content: Cow<'a, str>) -> Self {
        let lines = content
            .match_indices('\n')
            .map(|(i, _)| i + 1)
            .collect_vec();

        Self {
            path,
            content,
            lines,
        }
    }
    fn line(&self, n: usize) -> Option<&str> {
        if n > self.lines.len() + 1 {
            return None;
        }

        // Line 1 is not in the array
        // Line 2 is at index 0
        // Line 3 is at index 1
        // ...
        let start = if n > 1 { self.lines[n - 2] } else { 0 };
        let end = if n < self.lines.len() + 1 {
            self.lines[n - 1] - 1
        } else {
            self.content.len()
        };

        Some(&self.content[start..end])
    }
}
