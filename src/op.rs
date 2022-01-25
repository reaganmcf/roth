#[derive(Debug)]
pub enum Op {
    PushInt { val: i64 },
    PushString { val: String },
    PushBoolean { val: bool },
    Add,
    Sub,
    Mul,
    Div,
    Print,
    Or,
    And,
    Not,
    Eq,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
}
