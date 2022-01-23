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
            // TODO: we are deconstructing and reconstructing the same object lol
            Op::Int { val } => stack.push(Op::Int { val }),
            Op::String { val } => stack.push(Op::String { val }),
            Op::Boolean { val } => stack.push(Op::Boolean { val }),
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
            Op::Print => {
                // Print doesn't mutate the stack, so we peek instead of pop
                let x = stack.peek()?;
                x.print()?;
            }
            Op::Or => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(x.or(y)?);
            }
            Op::And => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                stack.push(x.and(y)?);
            }
            Op::Not => {
                let val = stack.pop()?;
                stack.push(val.not()?);
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
