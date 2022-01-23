use crate::error::RuntimeError;

#[derive(Debug)]
pub enum Op {
    Int { val: i64 },
    String { val: String },
    Boolean { val: bool },
    Add,
    Sub,
    Mul,
    Div,
    Print,
}

impl Op {
    pub fn add(self, other: Op) -> Result<Op, RuntimeError> {
        match self {
            Op::Int { val: y } => match other {
                Op::Int { val: x } => Ok(Op::Int { val: x + y }),
                _ => Err(RuntimeError::InvalidAdd),
            },
            Op::String { val: y } => match other {
                Op::String { val: x } => Ok(Op::String {
                    val: format!("{}{}", x, y),
                }),
                _ => Err(RuntimeError::InvalidAdd),
            },
            _ => Err(RuntimeError::InvalidAdd),
        }
    }

    pub fn sub(self, other: Op) -> Result<Op, RuntimeError> {
        match self {
            Op::Int { val: y } => match other {
                Op::Int { val: x } => Ok(Op::Int { val: x - y }),
                _ => Err(RuntimeError::InvalidSub),
            },
            _ => Err(RuntimeError::InvalidSub),
        }
    }

    pub fn mul(self, other: Op) -> Result<Op, RuntimeError> {
        match self {
            Op::Int { val: y } => match other {
                Op::Int { val: x } => Ok(Op::Int { val: x - y }),
                _ => Err(RuntimeError::InvalidMul),
            },
            _ => Err(RuntimeError::InvalidMul),
        }
    }

    pub fn div(self, other: Op) -> Result<Op, RuntimeError> {
        match self {
            Op::Int { val: y } => match other {
                Op::Int { val: x } => Ok(Op::Int { val: x - y }),
                _ => Err(RuntimeError::InvalidDiv),
            },
            _ => Err(RuntimeError::InvalidDiv),
        }
    }

    pub fn print(&self) -> Result<(), RuntimeError> {
        match self {
            Op::Int { val } => Ok(println!("{}", val)),
            Op::String { val } => Ok(println!("{}", val)),
            Op::Boolean { val } => Ok(println!("{}", val)),
            _ => Err(RuntimeError::InvalidPrint),
        }
    }
}
