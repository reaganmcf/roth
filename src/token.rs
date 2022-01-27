#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub start: usize,
    pub inner: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(inner: String, start: usize, kind: TokenKind) -> Self {
        Self { start, inner, kind }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // [1-9]+
    Number,

    // ".+"
    String,

    // [true|false]
    Boolean,

    // '+'
    Add,

    // '-'
    Sub,

    // '*'
    Mul,

    // '/'
    Div,

    // 'print'
    Print,

    // 'or'
    Or,

    // 'and'
    And,

    // 'not'
    Not,

    // 'eq'
    Eq,

    // <
    LessThan,

    // >
    GreaterThan,

    // <=
    LessThanEq,

    // >=
    GreaterThanEq,

    // 'if'
    If,

    // 'end'
    End,

    // 'macro'
    Macro,

    // 'dup'
    Dup,

    // 'drop',
    Drop,

    // '.*'
    Ident,
}
