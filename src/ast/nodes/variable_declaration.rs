use super::{Identifier, expression_statement::Expression};

#[derive(Debug)]
pub struct VariableDeclaration {
    pub id: Identifier,
    pub init: Expression,
    pub range: (usize, usize)
}
