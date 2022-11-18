use crate::tokenizer::{Token, FileLocation};

use self::{expression_statement::ExpressionStatement, block_statement::BlockStatement};

pub mod expression_statement;
pub mod block_statement;
pub mod variable_declaration;

#[derive(Debug)]
pub struct Literal {
    pub value: String,
    pub raw: String,
    pub range: (usize, usize),
    pub loc: FileLocation,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub range: (usize, usize),
    pub loc: FileLocation,
}

impl From<&Token> for Literal {
    fn from(token: &Token) -> Self {
        Self {
            value: token.value.clone(),
            range: token.range.clone(),
            raw: token.raw_value.clone(),
            loc: token.loc.clone(),
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
}
