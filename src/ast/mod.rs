use crate::tokenizer::Token;

use self::nodes::AstNode;

pub mod nodes;
mod parsers;
mod parser;

#[derive(Debug, PartialEq)]
pub enum AstErrorType {
    UnexpectedToken,
    UnexpectedTokenStart,
    UnexpectedEndOfInput,
}

#[derive(Debug)]
pub struct SearchResult<T> {
    pub value: T,
    pub ast_range: (usize, usize)
}

#[derive(Debug)]
pub struct AstParseError {
    pub index: usize,
    pub error_type: AstErrorType, 
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<AstNode>
}

impl Default for Program {
    fn default() -> Self {
        Self { body: Default::default() }
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstParseError> {
    parser::parse(tokens)
}
