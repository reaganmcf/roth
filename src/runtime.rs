use std::collections::{HashMap, VecDeque};

use miette::{Result, SourceSpan};

use crate::{
    error::RuntimeError,
    op::{Op, OpKind},
    stack::Stack,
    val::{Val, ValKind, ValType},
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
    box_ids: HashMap<String, (ValType, usize)>,
    boxes: Vec<Val>,
}

impl Runtime {
    pub fn new(source: String, ops: VecDeque<Op>) -> Self {
        Self {
            source,
            ops,
            mode: EvalMode::Normal,
            stack: Stack::new(),
            box_ids: HashMap::new(),
            boxes: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Stack> {
        while let Some(op) = self.ops.pop_front() {
            match op.kind {
                OpKind::If => self.eval_if(op)?,
                OpKind::End => match self.mode {
                    EvalMode::Normal => {
                        return Err(
                            RuntimeError::UnexpectedEndToken(self.source.clone(), op.span).into(),
                        )
                    }
                    EvalMode::If { .. } => self.mode = EvalMode::Normal,
                },
                OpKind::CreateBox { .. } => self.eval_create_box(op)?,
                OpKind::Pack => self.eval_pack_box()?,
                OpKind::Unpack => self.eval_unpack_box()?,
                OpKind::PushBox { .. } => self.eval_push_box(op)?,
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
            ValKind::Bool { val } => {
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

    fn eval_create_box(&mut self, op: Op) -> Result<()> {
        if let OpKind::CreateBox { val_type, name } = op.kind {
            if self.box_ids.contains_key(&name) {
                return Err(RuntimeError::BoxWithIdenticalNameAlreadyExists(
                    self.source.clone(),
                    name.clone(),
                    op.span,
                )
                .into());
            }

            let box_id = self.boxes.len();

            let val = match val_type {
                ValType::Int => ValKind::Int { val: 0 },
                ValType::Str => ValKind::Str { val: String::new() },
                ValType::Bool => ValKind::Bool { val: false },
                _ => unreachable!("ICE: parser only allows simple boxes"),
            };

            self.box_ids.insert(name, (val_type, box_id));
            self.boxes.push(Val::new(op.span, val));

            Ok(())
        } else {
            unreachable!("ICE: eval_create_box called with non create box operation")
        }
    }

    fn eval_pack_box(&mut self) -> Result<()> {
        let b = self.stack.pop()?;
        let val = self.stack.pop()?;

        match b.kind() {
            ValKind::BoxedInt { box_id } => match val.kind() {
                ValKind::Int { .. } => {
                    if self.boxes.get(*box_id).is_some() {
                        self.boxes[*box_id] = val;
                    } else {
                        unreachable!("ICE: invalid id");
                    }
                }
                _ => {
                    return Err(RuntimeError::IncompatibleBox(
                        self.source.clone(),
                        ValType::Int,
                        val.span(),
                    )
                    .into())
                }
            },
            ValKind::BoxedStr { box_id } => match val.kind() {
                ValKind::Str { .. } => {
                    if self.boxes.get(*box_id).is_some() {
                        self.boxes[*box_id] = val;
                    } else {
                        unreachable!("ICE: invalid id");
                    }
                }
                _ => {
                    return Err(RuntimeError::IncompatibleBox(
                        self.source.clone(),
                        ValType::Str,
                        val.span(),
                    )
                    .into())
                }
            },
            ValKind::BoxedBool { box_id } => match val.kind() {
                ValKind::Bool { .. } => {
                    if self.boxes.get(*box_id).is_some() {
                        self.boxes[*box_id] = val;
                    } else {
                        unreachable!("ICE: invalid id");
                    }
                }
                _ => {
                    return Err(RuntimeError::IncompatibleBox(
                        self.source.clone(),
                        ValType::Bool,
                        val.span(),
                    )
                    .into())
                }
            },

            _ => return Err(RuntimeError::CanOnlyPackBoxes(self.source.clone(), b.span()).into()),
        };

        Ok(())
    }

    fn eval_unpack_box(&mut self) -> Result<()> {
        let b = self.stack.pop()?;

        match b.kind() {
            ValKind::BoxedInt { box_id } => {
                if let Some(val) = self.boxes.get(*box_id) {
                    self.stack.push(val.clone());
                } else {
                    unreachable!("ICE: invalid id")
                }
            }
            ValKind::BoxedStr { box_id } => {
                if let Some(val) = self.boxes.get(*box_id) {
                    self.stack.push(val.clone());
                } else {
                    unreachable!("ICE: invalid id")
                }
            }
            ValKind::BoxedBool { box_id } => {
                if let Some(val) = self.boxes.get(*box_id) {
                    self.stack.push(val.clone());
                } else {
                    unreachable!("ICE: invalid id")
                }
            }
            _ => {
                return Err(RuntimeError::CanOnlyUnpackBoxes(self.source.clone(), b.span()).into())
            }
        }

        Ok(())
    }

    fn eval_push_box(&mut self, op: Op) -> Result<()> {
        if let OpKind::PushBox { name } = op.kind {
            if let Some((val_type, box_id)) = self.box_ids.get(&name) {
                let val = match val_type {
                    ValType::Int => Val::new(op.span, ValKind::BoxedInt { box_id: *box_id }),
                    ValType::Str => Val::new(op.span, ValKind::BoxedStr { box_id: *box_id }),
                    ValType::Bool => Val::new(op.span, ValKind::BoxedBool { box_id: *box_id }),
                    _ => unreachable!("ICE: val_type can only be Int, Str, or Bool"),
                };
                self.stack.push(val);
                Ok(())
            } else {
                return Err(RuntimeError::UnknownBox(self.source.clone(), op.span).into());
            }
        } else {
            unreachable!("eval_push_box called with non PushBox op")
        }
    }

    fn eval_simple(&mut self, op: Op) -> Result<()> {
        match op.kind {
            OpKind::PushInt { val: v } => {
                self.stack.push(Val::new(op.span, ValKind::Int { val: v }))
            }
            OpKind::PushStr { val: v } => {
                self.stack.push(Val::new(op.span, ValKind::Str { val: v }))
            }
            OpKind::PushBool { val: v } => {
                self.stack.push(Val::new(op.span, ValKind::Bool { val: v }))
            }
            OpKind::PushTypeInt => self
                .stack
                .push(Val::new(op.span, ValKind::Type { val: ValType::Int })),
            OpKind::PushTypeStr => self
                .stack
                .push(Val::new(op.span, ValKind::Type { val: ValType::Str })),
            OpKind::PushTypeBool => self
                .stack
                .push(Val::new(op.span, ValKind::Type { val: ValType::Bool })),
            OpKind::PushTypeBoxedInt => self.stack.push(Val::new(
                op.span,
                ValKind::Type {
                    val: ValType::BoxedInt,
                },
            )),
            OpKind::PushTypeBoxedStr => self.stack.push(Val::new(
                op.span,
                ValKind::Type {
                    val: ValType::BoxedStr,
                },
            )),
            OpKind::PushTypeBoxedBool => self.stack.push(Val::new(
                op.span,
                ValKind::Type {
                    val: ValType::BoxedBool,
                },
            )),
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
            OpKind::GetType => {
                let x = self.stack.pop()?;
                self.stack.push(x.get_type(op.span))
            }
            OpKind::Assert => {
                let x = self.stack.pop()?;
                x.assert(self.source.as_str(), op.span)?
            }

            _ => unreachable!("non simple opkind should have already been processed"),
        }

        Ok(())
    }
}
