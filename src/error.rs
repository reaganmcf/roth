use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Cannot open file")]
    #[diagnostic(
        code(roth::cannot_open_file),
        help("Make sure the file `{0}` exists and you have read permissions")
    )]
    CannotReadFile(String),

    #[error("Reached EOF before finding `end` token for macro definition")]
    #[diagnostic(
        code(roth::unclosed_macro),
        help("Make sure macro {0} has a corresponding `end` token")
    )]
    UnclosedMacro(String),

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

    #[error("Can't logical not this type")]
    #[diagnostic(code(roth::invalid_not))]
    InvalidNot(#[source_code] String, #[label("this value")] SourceSpan),

    #[error("Can't eq these types")]
    #[diagnostic(code(roth::invalid_eq))]
    InvalidEq(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't '<' these types")]
    #[diagnostic(code(roth::invalid_less_than))]
    InvalidLessThan(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't '>' these types")]
    #[diagnostic(code(roth::invalid_greater_than))]
    InvalidGreaterThan(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't '<=' these types")]
    #[diagnostic(code(roth::invalid_less_than_eq))]
    InvalidLessThanEq(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Can't '>=' these types")]
    #[diagnostic(code(roth::invalid_greater_than_eq))]
    InvalidGreaterThanEq(
        #[source_code] String,
        #[label("this value")] SourceSpan,
        #[label("with that value")] SourceSpan,
    ),

    #[error("Only boolean values can be used for if statements")]
    #[diagnostic(code(roth::ifs_expect_booleans))]
    IfsExpectBooleans(
        #[source_code] String,
        #[label("only boolean types work for if statements")] SourceSpan,
    ),

    #[error("Unclosed if statement")]
    #[diagnostic(
        code(roth::unclosed_if_statement),
        help("This usually happens when you forget to close an if statement with a corresponding 'end' token")
    )]
    UnclosedIfStatement(
        #[source_code] String,
        #[label("if statement has no closing 'end' token")] SourceSpan,
    ),
}
