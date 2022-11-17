use crate::tokenizer::Token;

use self::expression_statement::ExpressionStatement;

pub mod expression_statement;


#[derive(Debug)]
pub struct Literal {
    pub value: String,
    pub raw: String,
    pub range: (usize, usize),
}

impl From<&Token> for Literal {
    fn from(token: &Token) -> Self {
        Self {
            value: token.value.clone(),
            raw: token.raw_value.clone(),
            range: token.range.clone()
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    ExpressionStatement(ExpressionStatement)
}

