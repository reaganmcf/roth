use crate::{
    error::ParseError,
    token::{Token, TokenKind},
};
use miette::{NamedSource, Result};
use std::collections::VecDeque;

pub struct Lexer<'a> {
    raw_source: &'a str,
    source: VecDeque<char>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            raw_source: buffer,
            source: buffer.chars().collect(),
            cursor: 0,
        }
    }

    fn eat_trivia(&mut self) {
        let trivia: Vec<char> = vec![' ', '\n', '\t'];
        while let Some(c) = self.source.front() {
            if trivia.contains(&c) {
                self.source.pop_front();
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn create_token(
        &self,
        mut raw_token: String,
        start: usize,
        end: usize,
    ) -> Result<Token, ParseError> {
        let kind = match raw_token.as_str() {
            "+" => Ok(TokenKind::Add),
            "-" => Ok(TokenKind::Sub),
            "*" => Ok(TokenKind::Mul),
            "/" => Ok(TokenKind::Div),
            "print" => Ok(TokenKind::Print),
            "or" => Ok(TokenKind::Or),
            "and" => Ok(TokenKind::And),
            "not" => Ok(TokenKind::Not),
            _ => {
                if raw_token.starts_with("\"") {
                    if raw_token.ends_with("\"") {
                        // remove leading and trailing quotation marks
                        raw_token.remove(0);
                        raw_token.remove(raw_token.len() - 1);
                        Ok(TokenKind::String)
                    } else {
                        Err(ParseError::UnterminatedStringLiteral)
                    }
                } else if raw_token.parse::<i64>().is_ok() {
                    Ok(TokenKind::Number)
                } else if raw_token.parse::<bool>().is_ok() {
                    Ok(TokenKind::Boolean)
                } else {
                    Err(ParseError::UnkownToken(
                        self.raw_source.to_string(),
                        (start, raw_token.len()).into(),
                    ))
                }
            }
        }?;

        Ok(Token::new(raw_token, start, end, kind))
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        self.eat_trivia();
        while self.source.front().is_some() {
            let start = self.cursor;
            let mut curr = String::new();
            while let Some(c) = self.source.pop_front() {
                self.cursor += 1;
                if c.is_whitespace() {
                    let end = self.cursor;

                    // Gotta subtract one frm the end becuase we don't
                    // want to include whitespace in the Span
                    let token = self.create_token(curr.clone(), start, end)?;
                    tokens.push(token);
                    curr.clear();
                    break;
                } else {
                    // keep parsing chars
                    curr.push(c);
                }
            }

            // Check if we have an unfinished token here
            // which can happen at the end of the file
            if curr.len() != 0 {
                let end = self.cursor;

                let token = self.create_token(curr.clone(), start, end)?;
                tokens.push(token);
                curr.clear();
            }

            self.eat_trivia();
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenKind},
    };

    #[test]
    fn test_simple_lex() {
        let buff = "1 2 +";
        let mut lexer = Lexer::new(buff);
        let actual = lexer.lex().unwrap();

        let expected = vec![
            Token::new("1".into(), 0, 1, TokenKind::Number),
            Token::new("2".into(), 2, 3, TokenKind::Number),
            Token::new("+".into(), 4, 5, TokenKind::Add),
        ];

        assert_eq!(actual, expected);
    }
}
