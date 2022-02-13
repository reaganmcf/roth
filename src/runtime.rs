use std::collections::VecDeque;

use miette::{Result, SourceSpan};

use crate::{
    error::RuntimeError,
    op::{Op, OpKind},
    stack::Stack,
    val::{Val, ValKind},
};

enum EvalMode {
    Normal,
    If { last_span: SourceSpan },
}

pub struct Runtime {
    source: String,
    ops: VecDeque<Op>,
    stack: Stack,
    mode: EvalMode,
}

impl Runtime {
    pub fn new(source: String, ops: VecDeque<Op>) -> Self {
        Self {
            source,
            ops,
            mode: EvalMode::Normal,
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) -> Result<Stack> {
        while let Some(op) = self.ops.pop_front() {
            match op.kind {
                OpKind::If => self.eval_if(op)?,
                OpKind::End => {
                    if let EvalMode::Normal = self.mode {
                        return Err(
                            RuntimeError::UnexpectedEndToken(self.source.clone(), op.span).into(),
                        );
                    }
                }
                _ => self.eval_simple(op)?,
            }
        }

        // If we are still in a non normal eval mode here, then the program probably has structural errors
        // and it wasn't supposed to make it this far
        match &self.mode {
            EvalMode::If { last_span } => Err(RuntimeError::UnclosedIfStatement(
                self.source.clone(),
                last_span.clone(),
            )
            .into()),
            _ => Ok(self.stack.clone()),
        }
    }

    fn eval_if(&mut self, op: Op) -> Result<()> {
        // if condition is true, set eval_mode and carry on as normal
        // if condition is false, skip all tokens until the next CORRESPONDING 'else' or 'end'
        //      we keep track of the corresponding 'end' via 'if_counter'

        let val = self.stack.pop()?;

        match val.kind() {
            ValKind::Boolean { val } => {
                if *val {
                    // Keep evaluating as normal
                    self.mode = EvalMode::If { last_span: op.span };
                } else {
                    self.skip_until_corresponding_end(op)?;
                }
            }
            _ => {
                return Err(
                    RuntimeError::IfsExpectBooleans(self.source.to_string(), op.span).into(),
                )
            }
        }
        Ok(())
    }

    fn skip_until_corresponding_end(&mut self, op: Op) -> Result<()> {
        let mut if_counter: usize = 1;
        // skip to next 'end' keyword
        loop {
            match self.ops.pop_front() {
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
                        self.source.to_string(),
                        op.span,
                    )
                    .into());
                }
            }
        }

        self.mode = EvalMode::Normal;

        Ok(())
    }

    fn eval_simple(&mut self, op: Op) -> Result<()> {
        match op.kind {
            OpKind::PushInt { val: v } => {
                self.stack.push(Val::new(op.span, ValKind::Int { val: v }))
            }
            OpKind::PushString { val: v } => self
                .stack
                .push(Val::new(op.span, ValKind::String { val: v })),
            OpKind::PushBoolean { val: v } => self
                .stack
                .push(Val::new(op.span, ValKind::Boolean { val: v })),
            OpKind::Add => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.add(y, self.source.as_str(), op.span)?);
            }
            OpKind::Sub => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.sub(y, self.source.as_str(), op.span)?);
            }
            OpKind::Mul => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.mul(y, self.source.as_str(), op.span)?);
            }
            OpKind::Div => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.div(y, self.source.as_str(), op.span)?);
            }
            OpKind::Print => {
                let x = self.stack.pop()?;
                x.print();
            }
            OpKind::Or => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.or(y, self.source.as_str(), op.span)?);
            }
            OpKind::And => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.and(y, self.source.as_str(), op.span)?);
            }
            OpKind::Not => {
                let val = self.stack.pop()?;

                self.stack.push(val.not(self.source.as_str(), op.span)?);
            }
            OpKind::Eq => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.eq(y, self.source.as_str(), op.span)?);
            }
            OpKind::LessThan => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.lt(y, self.source.as_str(), op.span)?);
            }
            OpKind::GreaterThan => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.gt(y, self.source.as_str(), op.span)?);
            }
            OpKind::LessThanEq => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.lte(y, self.source.as_str(), op.span)?);
            }
            OpKind::GreaterThanEq => {
                let y = self.stack.pop()?;
                let x = self.stack.pop()?;

                self.stack.push(x.gte(y, self.source.as_str(), op.span)?);
            }
            OpKind::Dup => {
                self.stack.dup()?;
            }
            OpKind::Drop => {
                self.stack.pop()?;
            }
            OpKind::Swap => {
                self.stack.swap()?;
            }
            OpKind::Over => {
                self.stack.over()?;
            }
            OpKind::Rot => {
                self.stack.rot()?;
            }
            _ => unreachable!("non simple opkind should have already been processed"),
        }

        Ok(())
    }
}
