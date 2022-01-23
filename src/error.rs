use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("unknown token")]
    #[diagnostic(code(roth::unknown_token))]
    UnkownToken,
}

#[derive(Error, Debug, Diagnostic)]
pub enum RuntimeError {
    #[error("Empty stack")]
    #[diagnostic(code(roth::empty_stack))]
    EmptyStackError,

    #[error("Can't add these types")]
    #[diagnostic(code(roth::invalid_add))]
    InvalidAdd,

    #[error("Can't subtract these types")]
    #[diagnostic(code(roth::invalid_sub))]
    InvalidSub,

    #[error("Can't multiply these types")]
    #[diagnostic(code(roth::invalid_mul))]
    InvalidMul,

    #[error("Can't divide these types")]
    #[diagnostic(code(roth::invalid_div))]
    InvalidDiv,
}
