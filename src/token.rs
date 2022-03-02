use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub span: SourceSpan,
    pub inner: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(inner: String, start: usize, kind: TokenKind) -> Self {
        // spans for strings are actually +2 longer (since we trim the " marks)
        let mut span_len = inner.len();
        if let TokenKind::String = kind {
            span_len += 2;
        }

        Self {
            span: (start, span_len).into(),
            inner,
            kind,
        }
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

    // 'swap'
    Swap,

    // 'over'
    Over,

    // 'rot'
    Rot,

    // '.*'
    Ident,

    // 'type'
    Type,

    // 'type::int'
    TypeInt,

    // 'type::bool'
    TypeBool,

    // 'type::str`
    TypeStr,

    // 'type::box<int>'
    TypeBoxedInt,

    // 'assert'
    Assert,

    // 'box'
    Box,
}
