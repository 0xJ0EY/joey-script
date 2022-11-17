use super::Literal;

#[derive(Debug)]
pub struct BinaryExpression {
    operator: String,
    left: Literal,
    right: Literal,
}

#[derive(Debug)]
pub struct CallExpression {

}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub range: (usize, usize),
}
