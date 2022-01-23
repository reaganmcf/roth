use crate::token::{Token, TokenKind};
use miette::Result;
use std::collections::VecDeque;

pub struct Lexer {
    source: VecDeque<char>,
    cursor: u64,
}

impl Lexer {
    pub fn new(buffer: &str) -> Self {
        Self {
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

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        self.eat_trivia();
        while self.source.front().is_some() {
            let start = self.cursor;
            let mut curr = String::new();
            while let Some(c) = self.source.pop_front() {
                self.cursor += 1;
                if c.is_whitespace() {
                    let kind = TokenKind::new(&curr)?;
                    let end = self.cursor;

                    // Gotta subtract one frm the end becuase we don't
                    // want to include whitespace in the Span
                    tokens.push(Token::new(curr.clone(), start, end - 1, kind));
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
                let kind = TokenKind::new(&curr)?;
                let end = self.cursor;

                tokens.push(Token::new(curr.clone(), start, end, kind));
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
