use super::{Identifier, block_statement::BlockStatement};

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
    pub range: (usize, usize)
}
