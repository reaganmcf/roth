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
    Or,
    And,
    Not,
    Eq,
}

macro_rules! handlers {
    ($self:ident, $other:ident, $err:ident, ($t1:ident, $t2:ident, $op:expr)) => {
        if let Op::$t1 { val: x } = $self {
            if let Op::$t2 { val: y } = $other {
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

impl Op {
    pub fn add(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidAdd,
            (Int, Int, |x, y| Op::Int { val: x + y }),
            (String, String, |x, y| Op::String {
                val: format!("{}{}", x, y)
            })
        );
        Err(RuntimeError::InvalidAdd)
    }

    pub fn sub(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidSub,
            (Int, Int, |x, y| Op::Int { val: x - y })
        );
        Err(RuntimeError::InvalidSub)
    }

    pub fn mul(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidMul,
            (Int, Int, |x, y| Op::Int { val: x * y })
        );

        Err(RuntimeError::InvalidMul)
    }

    pub fn div(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidDiv,
            (Int, Int, |x, y| Op::Int { val: x / y })
        );

        Err(RuntimeError::InvalidDiv)
    }

    pub fn print(&self) -> Result<(), RuntimeError> {
        match self {
            Op::Int { val } => Ok(println!("{}", val)),
            Op::String { val } => Ok(println!("{}", val)),
            Op::Boolean { val } => Ok(println!("{}", val)),
            _ => Err(RuntimeError::InvalidPrint),
        }
    }

    pub fn or(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidOr,
            (Boolean, Boolean, |x, y| Op::Boolean { val: x || y })
        );
        Err(RuntimeError::InvalidOr)
    }

    pub fn and(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidOr,
            (Boolean, Boolean, |x, y| Op::Boolean { val: x && y })
        );
        Err(RuntimeError::InvalidAnd)
    }

    pub fn not(self) -> Result<Op, RuntimeError> {
        match self {
            Op::Boolean { val } => Ok(Op::Boolean { val: !val }),
            _ => Err(RuntimeError::InvalidNot),
        }
    }

    pub fn eq(self, other: Op) -> Result<Op, RuntimeError> {
        handlers!(
            self,
            other,
            InvalidEq,
            (Int, Int, |x, y| Op::Boolean { val: x == y }),
            (String, String, |x, y| Op::Boolean { val: x == y }),
            (Boolean, Boolean, |x, y| Op::Boolean { val: x == y })
        );
        Err(RuntimeError::InvalidEq)
    }
}
