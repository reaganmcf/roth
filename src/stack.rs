use crate::{error::RuntimeError, op::Op};

#[derive(Debug)]
pub struct Stack {
    ops: Vec<Op>,
}

impl Stack {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }


    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
    }

    pub fn pop(&mut self) -> Result<Op, RuntimeError> {
        match self.ops.pop() {
            Some(o) => Ok(o),
            None => Err(RuntimeError::EmptyStackError),
        }
    }

    pub fn peek(&self) -> Result<&Op, RuntimeError> {
        match self.ops.last() {
            Some(o) => Ok(o),
            None => Err(RuntimeError::EmptyStackError)
        }
    }
}
