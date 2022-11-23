use crate::{ast::{parser::AstParser, nodes::Identifier, AstParseError}, tokenizer::{TokenType, Separator}};

use super::parts::identifier::parse_identifier;

// Parsing util functions
pub fn is_open_param_bracket(parser: &AstParser, index: usize) -> bool {
    match parser.token_at(index) {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Separator(Separator::Parenthesis));
            let is_value = token.value == "(";

            is_type && is_value
        },
        None => false,
    }
}

pub fn is_param_separator(parser: &AstParser, index: usize) -> bool {
    match parser.token_at(index) {
        Some(token) => matches!(token.token_type, TokenType::Separator(Separator::Comma)),
        None => false,
    }
}

pub fn is_closed_param_bracket(parser: &AstParser, index: usize) -> bool {
    match parser.token_at(index) {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Separator(Separator::Parenthesis));
            let is_value = token.value == ")";

            is_type && is_value
        },
        None => false,
    }
}

pub fn parse_function_name(parser: &mut AstParser) -> Result<Identifier, AstParseError> {
    Ok(parse_identifier(parser)?)
}

pub fn is_semicolon_terminator(parser: &AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            matches!(token.token_type, TokenType::Separator(Separator::Terminator))
        },
        None => false,
    }
}
