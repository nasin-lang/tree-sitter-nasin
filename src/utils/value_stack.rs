use std::fmt::Debug;

use crate::bytecode::{JumpCount, RelativeValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueStack<V, B> {
    stack: Vec<V>,
    block_stack: Vec<B>,
}

impl<V, B> ValueStack<V, B> {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            block_stack: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn block_len(&self) -> usize {
        self.stack.len()
    }

    pub fn get(&self, pos: RelativeValue) -> Option<&V> {
        self.stack.get(self.idx(pos))
    }

    pub fn get_block(&self, pos: JumpCount) -> Option<&B> {
        self.block_stack.get(self.block_idx(pos))
    }

    pub fn get_mut(&mut self, pos: RelativeValue) -> Option<&mut V> {
        let idx = self.idx(pos);
        self.stack.get_mut(idx)
    }

    pub fn push(&mut self, value: V) {
        self.stack.push(value);
    }

    pub fn push_block(&mut self, block: B) {
        self.block_stack.push(block);
    }

    pub fn pop(&mut self) -> V {
        self.stack.pop().unwrap()
    }

    pub fn pop_block(&mut self) -> B {
        self.block_stack.pop().unwrap()
    }

    pub fn pop_many(&mut self, len: usize) -> Vec<V> {
        self.stack.drain(self.stack.len() - len..).collect()
    }

    pub fn pull(&mut self, pos: RelativeValue) {
        let value = self.stack.remove(self.idx(pos));
        self.stack.push(value);
    }

    pub fn dup(&mut self, pos: RelativeValue)
    where
        V: Clone,
    {
        self.stack.push(self.stack[self.idx(pos)].clone());
    }

    pub fn drop(&mut self, pos: RelativeValue) {
        self.stack.remove(self.idx(pos));
    }

    fn idx(&self, pos: RelativeValue) -> usize {
        self.stack.len() - pos as usize - 1
    }

    fn block_idx(&self, pos: JumpCount) -> usize {
        self.block_stack.len() - pos as usize - 1
    }
}

impl<V, B> IntoIterator for ValueStack<V, B> {
    type Item = V;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.stack.into_iter()
    }
}

impl<'a, V, B> IntoIterator for &'a ValueStack<V, B> {
    type Item = &'a V;
    type IntoIter = <&'a Vec<V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&self.stack).into_iter()
    }
}

impl<'a, V, B> IntoIterator for &'a mut ValueStack<V, B> {
    type Item = &'a mut V;
    type IntoIter = <&'a mut Vec<V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.stack).into_iter()
    }
}
