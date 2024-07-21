use std::cmp::min;
use std::fmt::Debug;

use derive_more::IntoIterator;
use derive_new::new;

use crate::bytecode::RelativeValue;

#[derive(Debug, Clone, PartialEq, Eq, IntoIterator)]
pub struct ValueStack<V, S> {
    #[into_iterator(owned, ref, ref_mut)]
    stack: Vec<V>,
    scopes: Vec<Scope<S>>,
}

impl<V, S> ValueStack<V, S> {
    pub fn new(initial_payload: S) -> Self {
        Self {
            stack: vec![],
            scopes: vec![Scope::new(initial_payload, 0)],
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn get(&self, pos: RelativeValue) -> Option<&V> {
        self.stack.get(self.idx(pos))
    }

    pub fn get_mut(&mut self, pos: RelativeValue) -> Option<&mut V> {
        let idx = self.idx(pos);
        self.stack.get_mut(idx)
    }

    pub fn push(&mut self, value: V) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> V {
        assert!(
            self.stack.len() > self.get_scope().start,
            "should not pop under the current scope start"
        );
        self.stack.pop().unwrap()
    }

    pub fn pop_many(&mut self, len: usize) -> Vec<V> {
        let at = self.stack.len() - len;
        assert!(
            at >= self.get_scope().start,
            "should not pop under the current scope start"
        );
        self.stack.split_off(at)
    }

    pub fn dup(&mut self, pos: RelativeValue)
    where
        V: Clone,
    {
        self.stack.push(self.stack[self.idx(pos)].clone());
    }

    pub fn scope_len(&self) -> usize {
        self.scopes.len()
    }

    pub fn get_scope(&self) -> &Scope<S> {
        self.scopes.last().unwrap()
    }

    pub fn get_scope_mut(&mut self) -> &mut Scope<S> {
        self.scopes.last_mut().unwrap()
    }

    pub fn get_loop_scope(&self) -> Option<&Scope<S>> {
        self.scopes.iter().rev().find(|scope| scope.is_loop)
    }

    pub fn get_loop_scope_mut(&mut self) -> Option<&mut Scope<S>> {
        self.scopes.iter_mut().rev().find(|scope| scope.is_loop)
    }

    pub fn create_scope(&mut self, payload: S) -> &mut Scope<S> {
        self.scopes.push(Scope::new(payload, self.stack.len()));
        self.scopes.last_mut().unwrap()
    }

    pub fn branch_scope(&mut self) -> (&mut Scope<S>, Vec<V>)
    where
        V: Clone,
    {
        assert!(
            self.scopes.len() > 1,
            "should not brach the first scope of function"
        );
        let scope = self.scopes.last_mut().unwrap();
        scope.branches += 1;
        scope.is_never = false;
        let removed = self.stack.split_off(min(scope.start, self.stack.len()));
        (scope, removed)
    }

    pub fn end_scope(&mut self) -> (Scope<S>, Vec<V>) {
        assert!(
            self.scopes.len() > 1,
            "should not end the first scope of function"
        );
        let scope = self.scopes.pop().unwrap();
        let removed = self.stack.split_off(min(scope.start, self.stack.len()));

        if scope.never_branches == scope.branches {
            self.get_scope_mut().mark_as_never();
        }

        (scope, removed)
    }

    fn idx(&self, pos: RelativeValue) -> usize {
        self.stack.len() - pos as usize - 1
    }
}

impl<V, S> Extend<V> for ValueStack<V, S> {
    fn extend<I: IntoIterator<Item = V>>(&mut self, iter: I) {
        self.stack.extend(iter);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new)]
pub struct Scope<P> {
    pub payload: P,
    #[new(value = "false")]
    pub is_loop: bool,
    #[new(value = "0")]
    pub loop_arity: u8,
    #[new(value = "1")]
    branches: usize,
    #[new(value = "0")]
    never_branches: usize,
    #[new(value = "false")]
    is_never: bool,
    start: usize,
}

impl<B> Scope<B> {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn is_never(&self) -> bool {
        self.is_never
    }

    pub fn mark_as_never(&mut self) {
        self.is_never = true;
        self.never_branches += 1;
    }
}
