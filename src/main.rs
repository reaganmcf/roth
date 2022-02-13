mod error;
mod lexer;
mod op;
mod parser;
mod runtime;
mod stack;
mod token;
mod val;

use miette::Result;
use reedline::{DefaultPrompt, Reedline, Signal};
use runtime::Runtime;
use std::process;

use lexer::Lexer;
use parser::Parser;
use stack::Stack;

use crate::error::ParseError;

fn main() -> Result<()> {
    // if file was passed in stdin, then eval without repl
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        let file_name = args.get(1).unwrap();
        match std::fs::read_to_string(file_name) {
            Ok(contents) => {
                eval(contents)?;
                Ok(())
            }
            _ => Err(ParseError::CannotReadFile(file_name.to_string()).into()),
        }
    } else {
        repl()
    }
}

fn repl() -> Result<()> {
    let mut line_editor = Reedline::create().unwrap();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                if buffer == "exit" {
                    process::exit(0);
                }

                match eval(buffer) {
                    Ok(mut stack) => {
                        if let Ok(v) = stack.pop() {
                            println!("{}", v);
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
            Signal::CtrlD | Signal::CtrlC => {
                line_editor.print_crlf().unwrap();
            }
            Signal::CtrlL => {
                line_editor.clear_screen().unwrap();
            }
        }
    }
}

fn eval(source: String) -> Result<Stack> {
    let tokens = Lexer::new(source.as_str()).lex()?;
    let ops = Parser::new(tokens).parse(source.to_string())?;
    Runtime::new(source, ops).run()
}
