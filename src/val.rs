use miette::SourceSpan;

use crate::error::RuntimeError;
use std::fmt::Display;

#[derive(Debug)]
pub struct Val {
    span: SourceSpan,
    kind: ValKind,
}

impl Val {
    pub fn new(span: SourceSpan, kind: ValKind) -> Self {
        Self { span, kind }
    }
}

#[derive(Debug)]
pub enum ValKind {
    Int { val: i64 },
    String { val: String },
    Boolean { val: bool },
}

macro_rules! handlers {
    ($self:ident, $other:ident, $src:ident, $err:ident, ($t1:ident, $t2:ident, $op:expr)) => {
        if let ValKind::$t1 { val: x } = $self.kind {
            if let ValKind::$t2 { val: y } = $other.kind {
                return Ok($op(x,y))
            } else {
                return Err(RuntimeError::$err($src, $self.span, $other.span));
            }
        }
    };

    //Err(RuntimeError::InvalidAdd(source, self.span, other.span))

    ($self:ident, $other:ident, $src:ident, $err:ident, ($t1:ident, $t2:ident, $op: expr), $(($ot1:ident, $ot2:ident, $oop:expr)),+) => {
        handlers!($self, $other, $src, $err, ($t1, $t2, $op));
        handlers!($self, $other, $src, $err, $(($ot1, $ot2, $oop)),+);
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ValKind::Int { val, .. } => write!(f, "{}", val),
            ValKind::String { val, .. } => write!(f, "{}", val),
            ValKind::Boolean { val, .. } => write!(f, "{}", val),
        }
    }
}

impl Val {
    fn merge_spans(&self, op: SourceSpan) -> SourceSpan {
        let off = self.span.offset();
        let len = op.offset() + op.len();

        (off, len).into()
    }

    pub fn add(
        self,
        other: Self,
        source: String,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidAdd,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Int { val: x + y }
            )),
            (String, String, |x, y| Val::new(
                merged_span,
                ValKind::String {
                    val: format!("{}{}", x, y)
                }
            ))
        );

        Err(RuntimeError::InvalidAdd(source, self.span, other.span))
    }

    pub fn sub(
        self,
        other: Self,
        source: String,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidSub,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Int { val: x - y }
            ))
        );
        Err(RuntimeError::InvalidSub(source, self.span, other.span))
    }

    //pub fn mul(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidMul,
    //        (Int, Int, |x, y| Self::Int { val: x * y })
    //    );

    //    Err(RuntimeError::InvalidMul)
    //}

    //pub fn div(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidDiv,
    //        (Int, Int, |x, y| Self::Int { val: x / y })
    //    );

    //    Err(RuntimeError::InvalidDiv)
    //}

    //pub fn print(&self) {
    //    println!("{}", self);
    //}

    //pub fn or(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidOr,
    //        (Boolean, Boolean, |x, y| Self::Boolean { val: x || y })
    //    );
    //    Err(RuntimeError::InvalidOr)
    //}

    //pub fn and(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidOr,
    //        (Boolean, Boolean, |x, y| Self::Boolean { val: x && y })
    //    );
    //    Err(RuntimeError::InvalidAnd)
    //}

    //pub fn not(self) -> Result<Self, RuntimeError> {
    //    match self {
    //        Self::Boolean { val } => Ok(Self::Boolean { val: !val }),
    //        _ => Err(RuntimeError::InvalidNot),
    //    }
    //}

    //pub fn eq(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidEq,
    //        (Int, Int, |x, y| Self::Boolean { val: x == y }),
    //        (String, String, |x, y| Self::Boolean { val: x == y }),
    //        (Boolean, Boolean, |x, y| Self::Boolean { val: x == y })
    //    );
    //    Err(RuntimeError::InvalidEq)
    //}

    //pub fn lt(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidLessThan,
    //        (Int, Int, |x, y| Self::Boolean { val: x < y }),
    //        (String, String, |x, y| Self::Boolean { val: x < y })
    //    );
    //    Err(RuntimeError::InvalidLessThan)
    //}

    //pub fn gt(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidGreaterThan,
    //        (Int, Int, |x, y| Self::Boolean { val: x > y }),
    //        (String, String, |x, y| Self::Boolean { val: x > y })
    //    );
    //    Err(RuntimeError::InvalidGreaterThan)
    //}

    //pub fn lte(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidLessThanEq,
    //        (Int, Int, |x, y| Self::Boolean { val: x <= y }),
    //        (String, String, |x, y| Self::Boolean { val: x <= y })
    //    );
    //    Err(RuntimeError::InvalidLessThanEq)
    //}

    //pub fn gte(self, other: Self) -> Result<Self, RuntimeError> {
    //    handlers!(
    //        self,
    //        other,
    //        InvalidGreaterThanEq,
    //        (Int, Int, |x, y| Self::Boolean { val: x >= y }),
    //        (String, String, |x, y| Self::Boolean { val: x >= y })
    //    );
    //    Err(RuntimeError::InvalidGreaterThanEq)
    //}
}
