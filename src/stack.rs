use std::collections::VecDeque;

use crate::{error::RuntimeError, val::Val};

#[derive(Debug)]
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
        let y = self.pop()?;
        let x = self.pop()?;

        self.push(y);
        self.push(x);

        Ok(())
    }

    // ( a b -- a b a)
    pub fn over(&mut self) -> Result<(), RuntimeError> {
        let y = self.pop()?;
        let x = self.pop()?;

        self.push(x.clone());
        self.push(y);
        self.push(x);

        Ok(())
    }

    // (a b c -- b c a)
    pub fn rot(&mut self) -> Result<(), RuntimeError> {
        match self.vals.pop_front() {
            Some(front) => {
               self.push(front);
               Ok(())
            }
            None => Err(RuntimeError::EmptyStackError),
        }
    }
}
