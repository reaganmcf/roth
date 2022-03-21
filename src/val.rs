use miette::{Result, SourceSpan};

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

    pub fn span(&self) -> SourceSpan {
        self.span.clone()
    }

    pub fn kind(&self) -> &ValKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValType {
    Int,
    Str,
    Bool,
    BoxedInt,
    BoxedStr,
    BoxedBool,
}

#[derive(Debug, Clone)]
pub enum ValKind {
    Int { val: i128 },
    Str { val: String },
    Bool { val: bool },
    Type { val: ValType },
    BoxedInt { box_id: usize },
    BoxedStr { box_id: usize },
    BoxedBool { box_id: usize },
}

impl std::fmt::Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValType::Int => write!(f, "type::int"),
            ValType::Str => write!(f, "type::str"),
            ValType::Bool => write!(f, "type::bool"),
            ValType::BoxedInt => write!(f, "type::box<int>"),
            ValType::BoxedStr => write!(f, "type::box<str>"),
            ValType::BoxedBool => write!(f, "type::box<bool>"),
        }
    }
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
            ValKind::Int { val } => write!(f, "{}", val),
            ValKind::Str { val } => write!(f, "{}", val),
            ValKind::Bool { val } => write!(f, "{}", val),
            ValKind::Type { val } => write!(f, "{}", val),
            ValKind::BoxedInt { .. } => write!(f, "BoxedInt"),
            ValKind::BoxedStr { .. } => write!(f, "BoxedStr"),
            ValKind::BoxedBool { .. } => write!(f, "BoxedBool"),
        }
    }
}

impl Val {
    fn merge_spans(&self, op: SourceSpan) -> SourceSpan {
        let off = self.span.offset();
        let len = op.offset() + op.len();

        (off, len).into()
    }

    pub fn add(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        let x = self.kind;
        let y = other.kind;
        match (x, y) {
            (ValKind::Int { val: x }, ValKind::Int { val: y }) => {
                Ok(Val::new(merged_span, ValKind::Int { val: x + y }))
            }
            (ValKind::Str { val: x }, ValKind::Str { val: y }) => Ok(Val::new(
                merged_span,
                ValKind::Str {
                    val: format!("{}{}", x, y),
                },
            )),
            (ValKind::Str { val: x }, ValKind::Int { val: y }) => Ok(Val::new(
                merged_span,
                ValKind::Str {
                    val: format!("{}{}", x, y),
                },
            )),
            (ValKind::Int { val: x }, ValKind::Str { val: y }) => Ok(Val::new(
                merged_span,
                ValKind::Str {
                    val: format!("{}{}", x, y),
                },
            )),
            _ => Err(RuntimeError::InvalidAdd(
                source.to_string(),
                self.span,
                other.span,
            )),
        }
    }

    pub fn sub(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
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

    pub fn mul(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
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

    pub fn div(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
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

    pub fn or(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidOr,
            (Bool, Bool, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x || y }
            ))
        );
        Err(RuntimeError::InvalidOr(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn and(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidAnd,
            (Bool, Bool, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x && y }
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
            ValKind::Bool { val } => Ok(Val::new(merged_span, ValKind::Bool { val: !val })),
            _ => Err(RuntimeError::InvalidNot(source.to_string(), self.span)),
        }
    }

    pub fn eq(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidEq,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x == y }
            )),
            (Str, Str, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x == y }
            )),
            (Bool, Bool, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x == y }
            )),
            (Type, Type, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x == y }
            ))
        );
        Err(RuntimeError::InvalidEq(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn lt(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidLessThan,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x < y }
            )),
            (Str, Str, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x < y }
            ))
        );
        Err(RuntimeError::InvalidLessThan(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn gt(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidGreaterThan,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x > y }
            )),
            (Str, Str, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x > y }
            ))
        );
        Err(RuntimeError::InvalidGreaterThan(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn lte(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidLessThanEq,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x <= y }
            )),
            (Str, Str, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x <= y }
            ))
        );
        Err(RuntimeError::InvalidLessThanEq(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn gte(self, other: Self, source: &str, op_span: SourceSpan) -> Result<Self, RuntimeError> {
        let merged_span = self.merge_spans(op_span);
        handlers!(
            self,
            other,
            source,
            InvalidGreaterThanEq,
            (Int, Int, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x >= y }
            )),
            (Str, Str, |x, y| Val::new(
                merged_span,
                ValKind::Bool { val: x >= y }
            ))
        );
        Err(RuntimeError::InvalidGreaterThanEq(
            source.to_string(),
            self.span,
            other.span,
        ))
    }

    pub fn assert(self, source: &str, op_span: SourceSpan) -> Result<(), RuntimeError> {
        match self.kind {
            ValKind::Bool { val } => {
                if !val {
                    return Err(RuntimeError::AssertionFailed(source.to_string(), op_span));
                }
                Ok(())
            }
            _ => Err(RuntimeError::InvalidAssert(source.to_string(), op_span)),
        }
    }

    pub fn get_type(self, op_span: SourceSpan) -> Self {
        let merged_span = self.merge_spans(op_span);
        match &self.kind {
            ValKind::Int { .. } => Val::new(merged_span, ValKind::Type { val: ValType::Int }),
            ValKind::Bool { .. } => Val::new(merged_span, ValKind::Type { val: ValType::Bool }),
            ValKind::Str { .. } => Val::new(merged_span, ValKind::Type { val: ValType::Str }),
            ValKind::Type { .. } => self.clone(),
            ValKind::BoxedInt { .. } => Val::new(
                merged_span,
                ValKind::Type {
                    val: ValType::BoxedInt,
                },
            ),
            ValKind::BoxedStr { .. } => Val::new(
                merged_span,
                ValKind::Type {
                    val: ValType::BoxedStr,
                },
            ),
            ValKind::BoxedBool { .. } => Val::new(
                merged_span,
                ValKind::Type {
                    val: ValType::BoxedBool,
                },
            ),
        }
    }
}
