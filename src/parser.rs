use crate::error::ParseError;
use crate::op::Op;
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
            let op = match token.kind {
                TokenKind::Add => Op::Add,
                TokenKind::Sub => Op::Sub,
                TokenKind::Mul => Op::Mul,
                TokenKind::Div => Op::Div,
                TokenKind::Print => Op::Print,
                TokenKind::Or => Op::Or,
                TokenKind::And => Op::And,
                TokenKind::Not => Op::Not,
                TokenKind::Eq => Op::Eq,
                TokenKind::LessThan => Op::LessThan,
                TokenKind::GreaterThan => Op::GreaterThan,
                TokenKind::LessThanEq => Op::LessThanEq,
                TokenKind::GreaterThanEq => Op::GreaterThanEq,
                TokenKind::String => Op::PushString {val: token.inner.clone() },
                TokenKind::Number => match token.inner.parse::<i64>() {
                    Ok(v) => Op::PushInt { val: v },
                    Err(_) => unreachable!("Lexer said it was a number, but it can't be parsed as one"),
                },
                TokenKind::Boolean => match token.inner.parse::<bool>() {
                    Ok(v) => Op::PushBoolean { val: v},
                    Err(_) => unreachable!("Lexer said it was a boolean, but it can't be parsed as one")
                }
            };

            ops.push(op);
        }

        Ok(ops)
    }
}
