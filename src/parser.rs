use std::collections::VecDeque;

use crate::error::ParseError;
use crate::op::{Op, OpKind};
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    pub fn parse(&mut self) -> Result<VecDeque<Op>, ParseError> {
        // Split on whitespace
        let mut ops = VecDeque::new();

        while let Some(token) = self.tokens.pop_front() {
            let op_kind = match token.kind {
                TokenKind::Macro => {
                    continue;
                }
                TokenKind::Ident => OpKind::PushBox { name: token.inner }, 
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
                TokenKind::If => OpKind::If,
                TokenKind::End => OpKind::End,
                TokenKind::Dup => OpKind::Dup,
                TokenKind::Drop => OpKind::Drop,
                TokenKind::Swap => OpKind::Swap,
                TokenKind::Over => OpKind::Over,
                TokenKind::Rot => OpKind::Rot,
                TokenKind::Type => OpKind::GetType,
                TokenKind::TypeInt => OpKind::PushTypeInt,
                TokenKind::TypeStr => OpKind::PushTypeStr,
                TokenKind::TypeBool => OpKind::PushTypeBool,
                TokenKind::TypeBoxedInt => OpKind::PushTypeBoxedInt,
                TokenKind::TypeBoxedStr => OpKind::PushTypeBoxedStr,
                TokenKind::TypeBoxedBool => OpKind::PushTypeBoxedBool,
                TokenKind::Assert => OpKind::Assert,
                TokenKind::Box => OpKind::CreateBox,
                TokenKind::Pack => OpKind::Pack,
                TokenKind::Unpack => OpKind::Unpack,
                TokenKind::String => OpKind::PushStr {
                    val: token.inner.clone(),
                },
                TokenKind::Number => match token.inner.parse::<i128>() {
                    Ok(v) => OpKind::PushInt { val: v },
                    Err(_) => {
                        unreachable!("Lexer said itsomethignCoo was a number, but it can't be parsed as one")
                    }
                },
                TokenKind::Boolean => match token.inner.parse::<bool>() {
                    Ok(v) => OpKind::PushBool { val: v },
                    Err(_) => {
                        unreachable!("Lexer said it was a boolean, but it can't be parsed as one")
                    }
                },
            };

            ops.push_back(Op::new(token.span, op_kind));
        }

        Ok(ops)
    }
}
