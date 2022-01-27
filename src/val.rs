use miette::SourceSpan;

use crate::error::RuntimeError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Val {
    span: SourceSpan,
    kind: ValKind,
}

impl Val {
    pub fn new(span: SourceSpan, kind: ValKind) -> Self {
        Self { span, kind }
    }

    pub fn kind(&self) -> &ValKind {
        &self.kind
    }
}

#[derive(Debug, Clone)]
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
                return Err(RuntimeError::$err($src.to_string(), $self.span, $other.span));
            }
        }
    };

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
        source: &str,
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

        Err(RuntimeError::InvalidAdd(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn sub(
        self,
        other: Self,
        source: &str,
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
        Err(RuntimeError::InvalidSub(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn mul(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidMul,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Int { val: x * y }
            ))
        );

        Err(RuntimeError::InvalidMul(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn div(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidDiv,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Int { val: x / y }
            ))
        );

        Err(RuntimeError::InvalidDiv(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn print(&self) {
        println!("{}", self);
    }

    pub fn or(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidOr,
            (Boolean, Boolean, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x || y }
            ))
        );
        Err(RuntimeError::InvalidOr(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn and(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidAnd,
            (Boolean, Boolean, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x && y }
            ))
        );
        Err(RuntimeError::InvalidAnd(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn not(self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        match self.kind {
            ValKind::Boolean { val } => Ok(Val::new(merged_span, ValKind::Boolean { val: !val })),
            _ => Err(RuntimeError::InvalidNot(source.to_string(), self.span)),
        }
    }

    pub fn eq(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidEq,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x == y }
            )),
            (String, String, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x == y }
            )),
            (Boolean, Boolean, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x == y }
            ))
        );
        Err(RuntimeError::InvalidEq(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn lt(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidLessThan,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x < y }
            )),
            (String, String, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x < y }
            ))
        );
        Err(RuntimeError::InvalidLessThan(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn gt(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidGreaterThan,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x > y }
            )),
            (String, String, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x > y }
            ))
        );
        Err(RuntimeError::InvalidGreaterThan(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn lte(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidLessThanEq,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x <= y }
            )),
            (String, String, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x <= y }
            ))
        );
        Err(RuntimeError::InvalidLessThanEq(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn gte(
        self,
        other: Self,
        source: &str,
        op_span: SourceSpan,
    ) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidGreaterThanEq,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x >= y }
            )),
            (String, String, |x, y| Val::new(
                merged_span,
                ValKind::Boolean { val: x >= y }
            ))
        );
        Err(RuntimeError::InvalidGreaterThanEq(
            source.to_string(),
            self.span,
            other.span,
        ))
    }
}
