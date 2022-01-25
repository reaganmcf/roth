use crate::error::RuntimeError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Val {
    Int { val: i64 },
    String { val: String },
    Boolean { val: bool },
}

macro_rules! handlers {
    ($self:ident, $other:ident, $err:ident, ($t1:ident, $t2:ident, $op:expr)) => {
        if let Self::$t1 { val: x } = $self {
            if let Self::$t2 { val: y } = $other {
                return Ok($op(x,y));
            } else {
                return Err(RuntimeError::$err);
            }
        }
    };

    ($self:ident, $other:ident, $err:ident, ($t1:ident, $t2:ident, $op: expr), $(($ot1:ident, $ot2:ident, $oop:expr)),+) => {
        handlers!($self, $other, $err, ($t1, $t2, $op));
        handlers!($self, $other, $err, $(($ot1, $ot2, $oop)),+);
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int { val } => write!(f, "{}", val),
            Self::String { val } => write!(f, "{}", val),
            Self::Boolean { val } => write!(f, "{}", val),
        }
    }
}

impl Val {
    pub fn add(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidAdd,
            (Int, Int, |x, y| Self::Int { val: x + y }),
            (String, String, |x, y| Self::String {
                val: format!("{}{}", x, y)
            })
        );
        Err(RuntimeError::InvalidAdd)
    }

    pub fn sub(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidSub,
            (Int, Int, |x, y| Self::Int { val: x - y })
        );
        Err(RuntimeError::InvalidSub)
    }

    pub fn mul(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidMul,
            (Int, Int, |x, y| Self::Int { val: x * y })
        );

        Err(RuntimeError::InvalidMul)
    }

    pub fn div(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidDiv,
            (Int, Int, |x, y| Self::Int { val: x / y })
        );

        Err(RuntimeError::InvalidDiv)
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn or(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidOr,
            (Boolean, Boolean, |x, y| Self::Boolean { val: x || y })
        );
        Err(RuntimeError::InvalidOr)
    }

    pub fn and(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidOr,
            (Boolean, Boolean, |x, y| Self::Boolean { val: x && y })
        );
        Err(RuntimeError::InvalidAnd)
    }

    pub fn not(self) -> Result<Self, RuntimeError> {
        match self {
            Self::Boolean { val } => Ok(Self::Boolean { val: !val }),
            _ => Err(RuntimeError::InvalidNot),
        }
    }

    pub fn eq(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidEq,
            (Int, Int, |x, y| Self::Boolean { val: x == y }),
            (String, String, |x, y| Self::Boolean { val: x == y }),
            (Boolean, Boolean, |x, y| Self::Boolean { val: x == y })
        );
        Err(RuntimeError::InvalidEq)
    }

    pub fn lt(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidLessThan,
            (Int, Int, |x, y| Self::Boolean { val: x < y }),
            (String, String, |x, y| Self::Boolean { val: x < y })
        );
        Err(RuntimeError::InvalidLessThan)
    }

    pub fn gt(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidGreaterThan,
            (Int, Int, |x, y| Self::Boolean { val: x > y }),
            (String, String, |x, y| Self::Boolean { val: x > y })
        );
        Err(RuntimeError::InvalidGreaterThan)
    }

    pub fn lte(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidLessThanEq,
            (Int, Int, |x, y| Self::Boolean { val: x <= y }),
            (String, String, |x, y| Self::Boolean { val: x <= y })
        );
        Err(RuntimeError::InvalidLessThanEq)
    }

    pub fn gte(self, other: Self) -> Result<Self, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidGreaterThanEq,
            (Int, Int, |x, y| Self::Boolean { val: x >= y }),
            (String, String, |x, y| Self::Boolean { val: x >= y })
        );
        Err(RuntimeError::InvalidGreaterThanEq)
    }
}
