use crate::tokenizer::Token;

use self::nodes::AstNode;

pub mod nodes;
mod parsers;
mod parser;

#[derive(Debug)]
pub enum AstErrorType {
    UnexpectedToken,
}

#[derive(Debug)]
pub struct AstError {
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

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstError> {
    parser::parse(tokens)
}
