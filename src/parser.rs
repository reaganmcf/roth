use std::collections::VecDeque;

use crate::error::ParseError;
use crate::op::{Op, OpKind};
use crate::token::{Token, TokenKind};
use crate::val::ValType;

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
    source_code: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source_code: String) -> Self {
        Self {
            tokens: tokens.into(),
            source_code,
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
                TokenKind::Box => self.parse_create_box(token.clone())?,
                TokenKind::Pack => OpKind::Pack,
                TokenKind::Unpack => OpKind::Unpack,
                TokenKind::Until => OpKind::Until,
                TokenKind::String => OpKind::PushStr {
                    val: token.inner.clone(),
                },
                TokenKind::Number => match token.inner.parse::<i128>() {
                    Ok(v) => OpKind::PushInt { val: v },
                    Err(_) => {
                        unreachable!("Lexer said it was a number, but it can't be parsed as one")
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

    // Boxes are created via the following syntax
    //  box <type> <ident>
    fn parse_create_box(&mut self, box_token: Token) -> Result<OpKind, ParseError> {
        if let Some(type_token) = self.tokens.pop_front() {
            let box_type = match type_token.kind {
                TokenKind::TypeInt => ValType::Int,
                TokenKind::TypeStr => ValType::Str,
                TokenKind::TypeBool => ValType::Bool,
                TokenKind::TypeBoxedInt | TokenKind::TypeBoxedStr | TokenKind::TypeBoxedBool => {
                    return Err(ParseError::UnboxableType(self.source_code.clone(), type_token.span))
                }
                _ => {
                    return Err(ParseError::BoxesNeedTypes(
                        self.source_code.clone(),
                        type_token.span,
                    ))
                }
            };
            if let Some(ident_token) = self.tokens.pop_front() {
                Ok(OpKind::CreateBox {
                    name: ident_token.inner,
                    val_type: box_type,
                })
            } else {
                let start = box_token.span.offset();
                let end = type_token.span.offset() + type_token.span.len();
                Err(ParseError::BoxesNeedNames(
                    self.source_code.clone(),
                    (start, end).into(),
                ))
            }
        } else {
            let start = box_token.span.offset() + box_token.span.len() - 1;
            let end = start;
            Err(ParseError::BoxesNeedTypes(
                self.source_code.clone(),
                (start, end).into(),
            ))
        }
    }
}
