use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("unknown token")]
    #[diagnostic(code(roth::unknown_token))]
    UnkownToken(#[source_code] String, #[label("Unknown token")] SourceSpan),

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
    InvalidAdd(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't subtract these types")]
    #[diagnostic(code(roth::invalid_sub))]
    InvalidSub(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("from that value")] SourceSpan,
    ),

    #[error("Can't multiply these types")]
    #[diagnostic(code(roth::invalid_mul))]
    InvalidMul(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't divide these types")]
    #[diagnostic(code(roth::invalid_div))]
    InvalidDiv(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("by that value")] SourceSpan,
    ),

    #[error("Can't logical or these types")]
    #[diagnostic(code(roth::invalid_or))]
    InvalidOr(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't logical and these types")]
    #[diagnostic(code(roth::invalid_and))]
    InvalidAnd(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't not this type")]
    #[diagnostic(code(roth::invalid_not))]
    InvalidNot(#[source_code] String, #[label("this value")] SourceSpan),

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
