use super::Literal;

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
    
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub range: (usize, usize),
}
