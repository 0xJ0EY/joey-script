use crate::tokenizer::Token;

pub mod nodes;

pub struct AstError {

}

pub struct Program {
    
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstError> {
    Ok(Program {})
}
