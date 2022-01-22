use crate::error::RuntimeError;

#[derive(Debug)]
pub enum Op {
    Int { val: i64 },
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn add(self, other: Op) -> Result<Op, RuntimeError> {
        if let Op::Int { val: y } = self {
            if let Op::Int { val: x } = other {
                Ok(Op::Int { val: x + y })
            } else {
                Err(RuntimeError::InvalidAdd)
            }
        } else {
            Err(RuntimeError::InvalidAdd)
        }
    }

    pub fn sub(self, other: Op) -> Result<Op, RuntimeError> {
        if let Op::Int { val: y } = self {
            if let Op::Int { val: x } = other {
                Ok(Op::Int { val: x - y })
            } else {
                Err(RuntimeError::InvalidSub)
            }
        } else {
            Err(RuntimeError::InvalidSub)
        }
    }

    pub fn mul(self, other: Op) -> Result<Op, RuntimeError> {
        if let Op::Int { val: y } = self {
            if let Op::Int { val: x } = other {
                Ok(Op::Int { val: x * y })
            } else {
                Err(RuntimeError::InvalidMul)
            }
        } else {
            Err(RuntimeError::InvalidMul)
        }
    }

    pub fn div(self, other: Op) -> Result<Op, RuntimeError> {
        if let Op::Int { val: y } = self {
            if let Op::Int { val: x } = other {
                Ok(Op::Int { val: x / y })
            } else {
                Err(RuntimeError::InvalidDiv)
            }
        } else {
            Err(RuntimeError::InvalidDiv)
        }
    }
}
