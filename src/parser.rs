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
                TokenKind::Number => match token.inner.parse::<i64>() {
                    Ok(v) => Op::Int { val: v },
                    Err(_) => unreachable!("number wasn't actually number???"),
                },
                TokenKind::String => Op::String {val: token.inner.clone() }
            };

            ops.push(op);
        }

        Ok(ops)
    }
}
