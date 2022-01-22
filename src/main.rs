mod error;
mod op;
mod stack;

use reedline::{DefaultPrompt, Reedline, Signal};
use miette::Result;
use std::process;

use op::Op;
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

                let parse = parse(buffer);
                let result = eval(parse)?;
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

fn parse(buffer: String) -> Vec<Op> {
    // Split on whitespace
    let tokens: Vec<&str> = buffer.split_whitespace().collect();
    let mut ops = Vec::new();

    for token in tokens {
        let op = if let Ok(n) = token.parse::<i64>() {
            Op::Int { val: n }
        } else if token.eq("add") {
            Op::Add
        } else if token.eq("sub") {
            Op::Sub
        } else if token.eq("mul") {
            Op::Mul
        } else if token.eq("div") {
            Op::Div
        } else {
            unreachable!("Unknown token: {}", token)
        };

        ops.push(op);
        print!("{} ", token);
    }
    print!("\n");

    return ops;
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
            _ => Ok(default)
        },
        Err(_) => Ok(default)
    }
}
