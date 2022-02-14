use crate::{
    error::ParseError,
    token::{Token, TokenKind},
};
use miette::Result;
use std::collections::VecDeque;

pub struct Lexer {
    source: VecDeque<char>,
    tokens: Vec<Token>,
    cursor: usize,
}

impl Lexer {
    pub fn new(buffer: &str) -> Self {
        Self {
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
            "dup" => Ok(TokenKind::Dup),
            "drop" => Ok(TokenKind::Drop),
            "swap" => Ok(TokenKind::Swap),
            "over" => Ok(TokenKind::Over),
            "rot" => Ok(TokenKind::Rot),
            "type" => Ok(TokenKind::Type),
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
                } else if raw_token.parse::<i128>().is_ok() {
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
            self.cursor += 1;
            if c == '\n' {
                break;
            }
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>> {
        self.eat_trivia();
        while self.source.front().is_some() {
            let mut start = self.cursor;
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
                        start = self.cursor;
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
    use crate::{lexer::Lexer, token::Token};
    use expect_test::expect;

    fn test(buff: &str) -> Vec<Token> {
        Lexer::new(buff).lex().unwrap()
    }

    #[test]
    fn test_simple_lex() {
        let actual = test("1 2 +");

        let expected = expect![[r#"
            [
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            0,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "1",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            2,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "2",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            4,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "+",
                    kind: Add,
                },
            ]
        "#]];

        expected.assert_debug_eq(&actual);
    }

    #[test]
    fn test_simple_comment() {
        let actual = test("//simple_comment");

        let expected = expect![[r#"
            []
        "#]];

        expected.assert_debug_eq(&actual);
    }

    #[test]
    fn test_simple_comment_with_spaces() {
        let actual = test("// this is a comment with spaces");

        let expected = expect![[r#"
            []
        "#]];

        expected.assert_debug_eq(&actual);
    }

    #[test]
    fn test_comments_before_expr() {
        let actual = test(
            "// this a comment before some expressions
1 2 +",
        );

        let expected = expect![[r#"
            [
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            42,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "1",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            44,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "2",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            46,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "+",
                    kind: Add,
                },
            ]
        "#]];

        expected.assert_debug_eq(&actual)
    }

    #[test]
    fn test_weird_comments() {
        let actual = test(
            "1 2 +
// some weird comment here
// afjh fj ak// kfja / / fjkod j/ f
1
// some thing
2
// some other thing
+
",
        );

        let expected = expect![[r#"
            [
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            0,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "1",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            2,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "2",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            4,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "+",
                    kind: Add,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            69,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "1",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            85,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "2",
                    kind: Number,
                },
                Token {
                    span: SourceSpan {
                        offset: SourceOffset(
                            107,
                        ),
                        length: SourceOffset(
                            1,
                        ),
                    },
                    inner: "+",
                    kind: Add,
                },
            ]
        "#]];

        expected.assert_debug_eq(&actual);
    }
}
