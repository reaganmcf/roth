use crate::{
    error::ParseError,
    token::{Token, TokenKind},
};
use fancy_regex::Regex;
use miette::Result;
use std::collections::VecDeque;

pub struct Lexer<'a> {
    raw_source: &'a str,
    source: VecDeque<char>,
    tokens: Vec<Token>,
    cursor: usize,
}

const MACRO_REGEX: &str = r"^\s*macro (?<name>.+?)(?=\s|\n)(?<content>[\s\S]*?)(?=end)";

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            raw_source: buffer,
            source: buffer.chars().collect(),
            tokens: vec![],
            cursor: 0,
        }
    }

    fn eat_trivia(&mut self) {
        let trivia: Vec<char> = vec![' ', '\n', '\t'];
        while let Some(c) = self.source.front() {
            if trivia.contains(c) {
                self.source.pop_front();
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn create_token(&self, mut raw_token: String, start: usize) -> Result<Token, ParseError> {
        let kind = match raw_token.as_str() {
            "+" => Ok(TokenKind::Add),
            "-" => Ok(TokenKind::Sub),
            "*" => Ok(TokenKind::Mul),
            "/" => Ok(TokenKind::Div),
            "print" => Ok(TokenKind::Print),
            "or" => Ok(TokenKind::Or),
            "and" => Ok(TokenKind::And),
            "not" => Ok(TokenKind::Not),
            "eq" => Ok(TokenKind::Eq),
            "<" => Ok(TokenKind::LessThan),
            ">" => Ok(TokenKind::GreaterThan),
            "<=" => Ok(TokenKind::LessThanEq),
            ">=" => Ok(TokenKind::GreaterThanEq),
            "if" => Ok(TokenKind::If),
            "end" => Ok(TokenKind::End),
            "macro" => Ok(TokenKind::Macro),
            _ => {
                if raw_token.starts_with('\"') {
                    if raw_token.ends_with('\"') {
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
                    Ok(TokenKind::Ident)
                }
            }
        }?;

        Ok(Token::new(raw_token, start, kind))
    }

    fn eat_until_newline(&mut self) {
        while let Some(c) = self.source.pop_front() {
            if c == '\n' {
                break;
            }
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>> {
        self.eat_trivia();
        while self.source.front().is_some() {
            let start = self.cursor;
            let mut curr = String::new();
            while let Some(c) = self.source.pop_front() {
                self.cursor += 1;

                let in_mid_of_string = curr.starts_with('\"') && !curr.ends_with('\"');

                // make sure we aren't at a whitespace inside of a string
                if c.is_whitespace() && !in_mid_of_string {
                    let token = self.create_token(curr.clone(), start)?;
                    self.tokens.push(token);
                    curr.clear();
                    break;
                } else {
                    // keep parsing chars
                    curr.push(c);

                    // if we are a comment, eat until new line
                    if curr == "//" {
                        self.eat_until_newline();
                        curr.clear();
                    }
                }
            }

            // Check if we have an unfinished token here
            // which can happen at the end of the file
            if !curr.is_empty() {
                let token = self.create_token(curr.clone(), start)?;
                self.tokens.push(token);
                curr.clear();
            }

            self.eat_trivia();
        }

        Ok(self.tokens)
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
        let lexer = Lexer::new(buff);
        let actual = lexer.lex().unwrap();

        let expected = vec![
            Token::new("1".into(), 0, TokenKind::Number),
            Token::new("2".into(), 2, TokenKind::Number),
            Token::new("+".into(), 4, TokenKind::Add),
        ];

        assert_eq!(actual, expected);
    }
}
