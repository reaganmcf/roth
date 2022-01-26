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
use std::{collections::VecDeque, process};
use val::Val;

use lexer::Lexer;
use op::Op;
use parser::Parser;
use stack::Stack;

use crate::{op::OpKind, val::ValKind};

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
                let mut stack = eval(ops, buffer)?;
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

fn eval(mut ops: VecDeque<Op>, source: String) -> Result<Stack> {
    let mut stack = Stack::new();

    let mut eval_mode = EvalMode::Normal;

    while let Some(op) = ops.pop_front() {
        match op.kind {
            OpKind::If => {
                // if condition is true, keep eval'ing until the next 'else' or 'end'
                // if condition is false, skip all tokens until the next 'else' or 'end'

                // TODO: nested ifs
                let val = stack.pop()?;

                match val.kind() {
                    ValKind::Boolean { val } => {
                        if *val {
                            // Keep evaluating as normal
                            eval_mode = EvalMode::If { last_span: op.span };
                        } else {
                            // skip to next 'end' keyword
                            loop {
                                match ops.pop_front() {
                                    Some(o) => {
                                        if let OpKind::End = o.kind {
                                            break;
                                        }
                                    }
                                    _ => {
                                        return Err(RuntimeError::UnclosedIfStatement(
                                            source.to_string(),
                                            op.span,
                                        ))?;
                                    }
                                }
                            }

                            eval_mode = EvalMode::Normal;
                        }
                    }
                    _ => return Err(RuntimeError::IfsExpectBooleans(source.to_string(), op.span))?,
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
            return Err(RuntimeError::UnclosedIfStatement(source, last_span))?;
        }
        _ => {}
    }

    Ok(stack)
}

fn eval_simple(op: Op, stack: &mut Stack, source: &String) -> Result<()> {
    match op.kind {
        OpKind::PushInt { val: v } => stack.push(Val::new(op.span, ValKind::Int { val: v })),
        OpKind::PushString { val: v } => stack.push(Val::new(op.span, ValKind::String { val: v })),
        OpKind::PushBoolean { val: v } => {
            stack.push(Val::new(op.span, ValKind::Boolean { val: v }))
        }
        OpKind::Add => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.add(y, &source, op.span)?);
        }
        OpKind::Sub => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.sub(y, &source, op.span)?);
        }
        OpKind::Mul => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.mul(y, &source, op.span)?);
        }
        OpKind::Div => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.div(y, &source, op.span)?);
        }
        OpKind::Print => {
            // Print doesn't mutate the stack, so we peek instead of pop
            let x = stack.peek()?;
            x.print();
        }
        OpKind::Or => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.or(y, &source, op.span)?);
        }
        OpKind::And => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.and(y, &source, op.span)?);
        }
        OpKind::Not => {
            let val = stack.pop()?;

            stack.push(val.not(&source, op.span)?);
        }
        OpKind::Eq => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.eq(y, &source, op.span)?);
        }
        OpKind::LessThan => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.lt(y, &source, op.span)?);
        }
        OpKind::GreaterThan => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.gt(y, &source, op.span)?);
        }
        OpKind::LessThanEq => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.lte(y, &source, op.span)?);
        }
        OpKind::GreaterThanEq => {
            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x.gte(y, &source, op.span)?);
        }
        _ => unreachable!("non simple opkind should have already been processed"),
    }

    Ok(())
}
