use super::AstNode;


#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<AstNode>,
    pub range: (usize, usize)
}
