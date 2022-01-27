use std::collections::{HashMap, VecDeque};

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
        let mut macro_table = HashMap::new();

        while let Some(token) = self.tokens.pop_front() {
            let op_kind = match token.kind {
                TokenKind::Macro => {
                    let (m_def, m_body) = self.parse_macro_def()?;
                    macro_table.insert(m_def, m_body);

                    continue;
                }
                TokenKind::Ident => {
                    // Ident's are macro names that are yet to be expanded
                    if let Some(expansion) = macro_table.get(&token.inner) {
                        for token in expansion.into_iter().rev() {
                            // we want to add them in the front in order,
                            // so we add them to the front in reverse order
                            self.tokens.push_front(token.clone());
                        }
                    } else {
                        unreachable!("no corresponding macro")
                    }

                    continue;
                }
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
            ops.push_back(Op::new(span.into(), op_kind));
        }

        Ok(ops)
    }

    fn parse_macro_def(&mut self) -> Result<(String, Vec<Token>), ParseError> {
        if let Some(next) = self.tokens.pop_front() {
            match next.kind {
                TokenKind::Ident => {
                    let macro_name = next.inner;
                    // consume until corresponding 'end' token
                    let macro_body = self.parse_macro_body(&macro_name)?;
                    Ok((macro_name, macro_body))
                }
                _ => unreachable!("unnamed macro"),
            }
        } else {
            unreachable!("eof after macro")
        }
    }

    fn parse_macro_body(&mut self, macro_name: &String) -> Result<Vec<Token>, ParseError> {
        let mut macro_body = vec![];
        let mut count: usize = 1;
        loop {
            match self.tokens.pop_front() {
                Some(t) => match t.kind {
                    TokenKind::End => {
                        count -= 1;
                        if count == 0 {
                            break;
                        }
                    }
                    _ => {
                        if matches!(t.kind, TokenKind::Macro | TokenKind::If) {
                            count += 1;
                        }

                        macro_body.push(t);
                    }
                },
                _ => return Err(ParseError::UnclosedMacro(macro_name.to_string())),
            }
        }

        Ok(macro_body)
    }
}
