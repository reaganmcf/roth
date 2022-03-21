use miette::SourceSpan;

use crate::val::ValType;

#[derive(Debug, Clone)]
pub struct Op {
    pub(crate) span: SourceSpan,
    pub(crate) kind: OpKind,
}

impl Op {
    pub fn new(span: SourceSpan, kind: OpKind) -> Self {
        Self { span, kind }
    }
}

#[derive(Debug, Clone)]
pub enum OpKind {
    PushInt { val: i128 },
    PushStr { val: String },
    PushBool { val: bool },
    CreateBox { val_type: ValType, name: String },
    PushBox { name: String },
    Pack,
    Unpack,
    PushTypeInt,
    PushTypeStr,
    PushTypeBool,
    PushTypeBoxedInt,
    PushTypeBoxedStr,
    PushTypeBoxedBool,
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Or,
    And,
    Not,
    Eq,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
    If,
    End,
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    GetType,
    Assert,
    Until
}
