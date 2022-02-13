use std::collections::VecDeque;

use crate::{error::RuntimeError, val::Val};

#[derive(Debug, Clone)]
pub struct Stack {
    vals: VecDeque<Val>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            vals: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: Val) {
        self.vals.push_back(val);
    }

    pub fn pop(&mut self) -> Result<Val, RuntimeError> {
        match self.vals.pop_back() {
            Some(v) => Ok(v),
            None => Err(RuntimeError::EmptyStackError),
        }
    }

    pub fn peek(&self) -> Result<&Val, RuntimeError> {
        match self.vals.back() {
            Some(v) => Ok(v),
            None => Err(RuntimeError::EmptyStackError),
        }
    }

    // (a -- a a)
    pub fn dup(&mut self) -> Result<(), RuntimeError> {
        self.vals.push_back(self.peek()?.clone());
        Ok(())
    }

    // ( a b -- b a)
    pub fn swap(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(b);
        self.push(a);

        Ok(())
    }

    // ( a b -- a b a)
    pub fn over(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(a.clone());
        self.push(b);
        self.push(a);

        Ok(())
    }

    // (a b c -- b c a)
    pub fn rot(&mut self) -> Result<(), RuntimeError> {
        let c = self.pop()?;
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(b);
        self.push(c);
        self.push(a);

        Ok(())
    }
}
