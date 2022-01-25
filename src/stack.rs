use crate::{error::RuntimeError, val::Val};

#[derive(Debug)]
pub struct Stack {
    vals: Vec<Val>
}

impl Stack {
    pub fn new() -> Self {
        Self { vals: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.vals.is_empty()
    }

    pub fn push(&mut self, val: Val) {
        self.vals.push(val);
    }

    pub fn pop(&mut self) -> Result<Val, RuntimeError> {
        match self.vals.pop() {
            Some(v) => Ok(v),
            None => Err(RuntimeError::EmptyStackError),
        }
    }

    pub fn peek(&self) -> Result<&Val, RuntimeError> {
        match self.vals.last() {
            Some(v) => Ok(v),
            None => Err(RuntimeError::EmptyStackError)
        }
    }
}
