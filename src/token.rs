#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub inner: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(inner: String, start: usize, end: usize, kind: TokenKind) -> Self {
        Self {
            start,
            end,
            inner,
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
    Not
}
