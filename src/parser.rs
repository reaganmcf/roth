use crate::error::ParseError;
use crate::op::{Op, OpKind};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Vec<Op>, ParseError> {
        // Split on whitespace
        let mut ops = Vec::new();

        for token in &self.tokens {
            let op_kind = match token.kind {
                TokenKind::Add => OpKind::Add,
                TokenKind::Sub => OpKind::Sub,
                TokenKind::Mul => OpKind::Mul,
                TokenKind::Div => OpKind::Div,
                TokenKind::Print => OpKind::Print,
                TokenKind::Or => OpKind::Or,
                TokenKind::And => OpKind::And,
                TokenKind::Not => OpKind::Not,
                TokenKind::Eq => OpKind::Eq,
                TokenKind::LessThan => OpKind::LessThan,
                TokenKind::GreaterThan => OpKind::GreaterThan,
                TokenKind::LessThanEq => OpKind::LessThanEq,
                TokenKind::GreaterThanEq => OpKind::GreaterThanEq,
                TokenKind::String => OpKind::PushString {
                    val: token.inner.clone(),
                },
                TokenKind::Number => match token.inner.parse::<i64>() {
                    Ok(v) => OpKind::PushInt { val: v },
                    Err(_) => {
                        unreachable!("Lexer said it was a number, but it can't be parsed as one")
                    }
                },
                TokenKind::Boolean => match token.inner.parse::<bool>() {
                    Ok(v) => OpKind::PushBoolean { val: v },
                    Err(_) => {
                        unreachable!("Lexer said it was a boolean, but it can't be parsed as one")
                    }
                },
            };

            // spans for strings are actually +2 longer (since we trim the " marks)
            let mut span_len = token.inner.len();
            if let TokenKind::String = token.kind {
                span_len += 2;
            }

            let span = (token.start, span_len);
            ops.push(Op::new(span.into(), op_kind));
        }

        Ok(ops)
    }
}
