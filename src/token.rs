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
    // '+'
    Add,

    // '-'
    Sub,

    // '*'
    Mul,

    // '/'
    Div,

    // [1-9]+
    Number,

    // ".+"
    String,
}
