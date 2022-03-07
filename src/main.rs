mod error;
mod lexer;
mod op;
mod parser;
mod preprocessor;
mod runtime;
mod stack;
mod token;
mod val;

use miette::Result;
use preprocessor::PreProcessor;
use reedline::{DefaultPrompt, Reedline, Signal};
use runtime::Runtime;
use std::{env::set_current_dir, path::PathBuf, process};

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
                // we have to set our current working directory to where this file is
                let path = PathBuf::from(file_name);
                match path.parent() {
                    Some(dir) => {
                        if set_current_dir(dir).is_err() {
                            todo!("failed to set current dir to file's parent");
                        }
                    }
                    None => todo!("file name doesn't have parent folder"),
                }
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
    let expanded_source = PreProcessor::new(source.as_str()).expand()?;
    let tokens = Lexer::new(expanded_source.as_str()).lex()?;
    let ops = Parser::new(tokens, expanded_source.clone()).parse()?;
    Runtime::new(expanded_source, ops).run()
}
