use miette::Result;

use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub start: u64,
    pub end: u64,
    pub inner: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(inner: String, start: u64, end: u64, kind: TokenKind) -> Self {
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
}

impl TokenKind {
    pub fn new(from: &String) -> Result<Self, ParseError> {
        match from.as_str() {
            "+" => Ok(TokenKind::Add),
            "-" => Ok(TokenKind::Sub),
            "*" => Ok(TokenKind::Mul),
            "/" => Ok(TokenKind::Div),
            _ => {
                if from.parse::<i64>().is_ok() {
                    Ok(TokenKind::Number)
                } else {
                    Err(ParseError::UnkownToken)
                }
            }
        }
    }
}
