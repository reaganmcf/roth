use miette::SourceSpan;

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
    PushString { val: String },
    PushBoolean { val: bool },
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
}
