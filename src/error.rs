use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::val::ValType;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Cannot open file")]
    #[diagnostic(
        code(roth::cannot_open_file),
        help("Make sure the file `{0}` exists and you have read permissions")
    )]
    CannotReadFile(String),

    #[error("unterminated string literal")]
    #[diagnostic(code(roth::unterminated_string))]
    UnterminatedStringLiteral,

    #[error("Can't include non-existent file")]
    #[diagnostic(
        code(roth::include_file_doesnt_exist),
        help("Make sure there exists a file at `{1}`")
    )]
    CantIncludeNonExistentFile(
        #[source_code] String,
        String, // file name
        #[label("This include statement points to a non existent file")] SourceSpan,
    ),

    #[error("Unable to open or read file inside `include` statement")]
    #[diagnostic(
        code(roth::cant_open_include_file),
        help("Unable to open or read the file at `{1}`")
    )]
    CantOpenOrReadIncludeFile(
        #[source_code] String,
        String, // file name
        #[label("This include statement points to a file that you don't have access to open")]
        SourceSpan,
    ),

    #[error("Can't create a box without a type")]
    #[diagnostic(
        code(roth::boxes_need_types),
        help("Create a box with the syntax `box type::... foo`")
    )]
    BoxesNeedTypes(
        #[source_code] String,
        #[label("this value should be `type::...`")] SourceSpan,
    ),

    #[error("Can't create a box without a name")]
    #[diagnostic(
        code(roth::boxes_need_names),
        help("Create a box with the syntax `box type::... foo`")
    )]
    BoxesNeedNames(
        #[source_code] String,
        #[label("No name found for this box definition")] SourceSpan,
    ),

    #[error("Boxes cannot be created for this type")]
    #[diagnostic(
        code(roth::unboxable_type),
        help("boxes can only be created for types `type::int`, `type::str`, and `type::bool`")
    )]
    UnboxableType(
        #[source_code] String,
        #[label("this type is not boxable")] SourceSpan,
    ),
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

    #[error("Unexpected end token")]
    #[diagnostic(
        code(roth::unexpected_end_token),
        help("End tokens are used for macros, if statements, loops, etc. - but this end token is by itself")
    )]
    UnexpectedEndToken(
        #[source_code] String,
        #[label("Try removing this token")] SourceSpan,
    ),

    #[error("Assertion failed")]
    #[diagnostic(code(roth::assertion_failed))]
    AssertionFailed(
        #[source_code] String,
        #[label("this assertion did not evaluate to `true`")] SourceSpan,
    ),

    #[error("Can't assert this type")]
    #[diagnostic(
        code(roth::invalid_add),
        help("assert statements can only be used with bool values")
    )]
    InvalidAssert(
        #[source_code] String,
        #[label("can only assert bool values")] SourceSpan,
    ),

    #[error("Can't pack non-box types")]
    #[diagnostic(
        code(roth::can_only_pack_boxes),
        help("`pack` can only be used on boxes")
    )]
    CanOnlyPackBoxes(
        #[source_code] String,
        #[label("this value is not a box")] SourceSpan,
    ),

    #[error("Can't unpack non-box types")]
    #[diagnostic(
        code(roth::can_only_pack_boxes),
        help("`unpack` can only be used on boxes")
    )]
    CanOnlyUnpackBoxes(
        #[source_code] String,
        #[label("this value is not a box")] SourceSpan,
    ),

    #[error("{1} boxes can only be packed with {1} values")]
    #[diagnostic(
        code(roth::incompatible_box),
        help("`pack` only works with compatible value types")
    )]
    IncompatibleBox(
        #[source_code] String,
        ValType,
        #[label("This type is not {1}")] SourceSpan,
    ),

    #[error("Box with identical name already exists")]
    #[diagnostic(code(roth::box_already_exists), help("rename `{1}` to something else"))]
    BoxWithIdenticalNameAlreadyExists(
        #[source_code] String,
        String, // name that's already in use
        #[label("rename this")] SourceSpan,
    ),

    #[error("Unknown box")]
    #[diagnostic(
        code(roth::unknown_box),
        help("couldn't find any boxes with this name. Is it a typo?")
    )]
    UnknownBox(
        #[source_code] String,
        #[label("no box with this name")] SourceSpan
    )
}
