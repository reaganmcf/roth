use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("unknown token")]
    #[diagnostic(code(roth::unknown_token))]
    UnkownToken(
        #[source_code] String,
        #[label("Unknown token")] SourceSpan,
    ),

    #[error("unterminated string literal")]
    #[diagnostic(code(roth::unterminated_string))]
    UnterminatedStringLiteral,
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

    #[error("Can only use `print` on value types")]
    #[diagnostic(code(roth::invalid_print))]
    InvalidPrint,

    #[error("Can't or these types")]
    #[diagnostic(code(roth::invalid_or))]
    InvalidOr,

    #[error("Can't and these types")]
    #[diagnostic(code(roth::invalid_and))]
    InvalidAnd,

    #[error("Can't not this types")]
    #[diagnostic(code(roth::invalid_not))]
    InvalidNot,

    #[error("Can't eq these types")]
    #[diagnostic(code(roth::invalid_not))]
    InvalidEq,

    #[error("Can't '<' these types")]
    #[diagnostic(code(roth::invalid_less_than))]
    InvalidLessThan,

    #[error("Can't '>' these types")]
    #[diagnostic(code(roth::invalid_greater_than))]
    InvalidGreaterThan,

    #[error("Can't '<=' these types")]
    #[diagnostic(code(roth::invalid_less_than_eq))]
    InvalidLessThanEq,

    #[error("Can't '>=' these types")]
    #[diagnostic(code(roth::invalid_greater_than_eq))]
    InvalidGreaterThanEq,

}
