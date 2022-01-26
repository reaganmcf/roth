mod error;
mod lexer;
mod op;
mod parser;
mod stack;
mod token;
mod val;

use error::RuntimeError;
use miette::{Result, SourceSpan};
use reedline::{DefaultPrompt, Reedline, Signal};
use std::process;
use val::Val;

use lexer::Lexer;
use op::Op;
use parser::Parser;
use stack::Stack;

use crate::{error::ParseError, op::OpKind, val::ValKind};

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

                let mut stack = eval(buffer)?;
                if let Ok(v) = stack.pop() {
                    println!("{}", v);
                }
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

enum EvalMode {
    Normal,

    If { last_span: SourceSpan },
}

fn eval(source: String) -> Result<Stack> {
    let tokens = Lexer::new(source.as_str()).lex()?;
    let mut ops = Parser::new(tokens).parse()?;
    let mut stack = Stack::new();

    let mut eval_mode = EvalMode::Normal;

    while let Some(op) = ops.pop_front() {
        match op.kind {
            OpKind::If => {
                // if condition is true, set eval_mode and carry on as normal
                // if condition is false, skip all tokens until the next CORRESPONDING 'else' or 'end'
                //      we keep track of the corresponding 'end' via 'if_counter'

                let val = stack.pop()?;

                match val.kind() {
                    ValKind::Boolean { val } => {
                        if *val {
                            // Keep evaluating as normal
                            eval_mode = EvalMode::If { last_span: op.span };
                        } else {
                            let mut if_counter: usize = 1;
                            // skip to next 'end' keyword
                            loop {
                                match ops.pop_front() {
                                    Some(o) => match o.kind {
                                        OpKind::End => {
                                            if_counter -= 1;
                                            if if_counter == 0 {
                                                break;
                                            }
                                        }
                                        OpKind::If => {
                                            if_counter += 1;
                                        }
                                        _ => {}
                                    },
                                    _ => {
                                        return Err(RuntimeError::UnclosedIfStatement(
                                            source.to_string(),
                                            op.span,
                                        )
                                        .into());
                                    }
                                }
                            }

                            eval_mode = EvalMode::Normal;
                        }
                    }
                    _ => {
                        return Err(
                            RuntimeError::IfsExpectBooleans(source.to_string(), op.span).into()
                        )
                    }
                }
            }
            OpKind::End => {
                eval_mode = EvalMode::Normal;
            }
            _ => eval_simple(op, &mut stack, &source)?,
        }
    }

    // If we are still in a non normal eval mode here, then the program probably has structural errors
    // and it wasn't supposed to make it this far
    match eval_mode {
        EvalMode::If { last_span } => {
            Err(RuntimeError::UnclosedIfStatement(source, last_span).into())
        }
        _ => Ok(stack),
    }
}

fn eval_simple(op: Op, stack: &mut Stack, source: &str) -> Result<()> {
    match op.kind {
        OpKind::PushInt { val: v } => stack.push(Val::new(op.span, ValKind::Int { val: v })),
        OpKind::PushString { val: v } => stack.push(Val::new(op.span, ValKind::String { val: v })),
        OpKind::PushBoolean { val: v } => {
            stack.push(Val::new(op.span, ValKind::Boolean { val: v }))
        }
        OpKind::Add => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.add(y, source, op.span)?);
        }
        OpKind::Sub => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.sub(y, source, op.span)?);
        }
        OpKind::Mul => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.mul(y, source, op.span)?);
        }
        OpKind::Div => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.div(y, source, op.span)?);
        }
        OpKind::Print => {
            // Print doesn't mutate the stack, so we peek instead of pop
            let x = stack.peek()?;
            x.print();
        }
        OpKind::Or => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.or(y, source, op.span)?);
        }
        OpKind::And => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.and(y, source, op.span)?);
        }
        OpKind::Not => {
            let val = stack.pop()?;

            stack.push(val.not(source, op.span)?);
        }
        OpKind::Eq => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.eq(y, source, op.span)?);
        }
        OpKind::LessThan => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.lt(y, source, op.span)?);
        }
        OpKind::GreaterThan => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.gt(y, source, op.span)?);
        }
        OpKind::LessThanEq => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.lte(y, source, op.span)?);
        }
        OpKind::GreaterThanEq => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.gte(y, source, op.span)?);
        }
        _ => unreachable!("non simple opkind should have already been processed"),
    }

    Ok(())
}
