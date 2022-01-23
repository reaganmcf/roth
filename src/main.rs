mod error;
mod lexer;
mod op;
mod parser;
mod stack;
mod token;

use miette::Result;
use reedline::{DefaultPrompt, Reedline, Signal};
use std::process;

use lexer::Lexer;
use op::Op;
use parser::Parser;
use stack::Stack;

fn main() -> Result<()> {
    let mut line_editor = Reedline::create().unwrap();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                if buffer == "exit" {
                    process::exit(0);
                }

                let tokens = Lexer::new(buffer.as_str()).lex()?;
                let ops = Parser::new(tokens).parse()?;
                let result = eval(ops)?;
                println!("Result: {}", result);
            }
            Signal::CtrlD | Signal::CtrlC => {
                line_editor.print_crlf().unwrap();
                process::exit(0);
            }
            Signal::CtrlL => {
                line_editor.clear_screen().unwrap();
            }
        }
    }
}

fn eval(ops: Vec<Op>) -> Result<i64> {
    let mut stack = Stack::new();

    for op in ops.into_iter() {
        match op {
            Op::Int { val } => stack.push(Op::Int { val }),
            Op::Add => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(x.add(y)?);
            }
            Op::Sub => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(x.sub(y)?);
            }
            Op::Mul => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(x.mul(y)?);
            }
            Op::Div => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(x.div(y)?);
            }
        }
    }

    // Try to return the last item in the stack, if present
    // Otherwise, just return 0
    let default = 0;

    match stack.pop() {
        Ok(e) => match e {
            Op::Int { val } => Ok(val),
            _ => Ok(default),
        },
        Err(_) => Ok(default),
    }
}
