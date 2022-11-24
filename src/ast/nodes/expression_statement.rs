use super::{Literal, Identifier};

#[derive(Debug)]
pub struct LiteralExpression {
    pub value: Literal,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub operator: String,
    pub left: Literal,
    pub right: Literal,
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct ObjectExpression {
    
}

#[derive(Debug)]
pub struct ArrayExpression {

}

#[derive(Debug)]
pub struct SequenceExpression {
    pub expressions: Vec<Expression>
}

#[derive(Debug)]
pub struct IdentifierExpression {
    pub identifier: Identifier,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(IdentifierExpression),
    Literal(LiteralExpression),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    ObjectExpression(ObjectExpression),
    SequenceExpression(SequenceExpression),
    ArrayExpression(ArrayExpression),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub range: (usize, usize),
}
