use crate::tokenizer::Token;

use super::{AstError, Program, AstErrorType};

#[derive(Debug)]
pub struct AstParser<'a> {
    index: usize,
    tokens: &'a Vec<Token>
}

impl<'a> AstParser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> AstParser {
        AstParser { 
            index: 0,
            tokens
        }
    }

    pub fn has_tokens(&self) -> bool {
        self.token().is_some()
    }

    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn peek_forward(&self, distance: usize) -> Option<&Token> {
        self.tokens.get(self.index + distance)
    }

    pub fn peek_back(&self) -> Option<&Token> {
        self.tokens.get(self.index - 1)
    }

    pub fn walk_back(&mut self) {
        self.index -= 1;
    }

    pub fn get_current_index(&self) -> usize {
        return self.index;
    }

    pub fn consume(&mut self) -> Option<&Token> {
        let value = self.tokens.get(self.index);

        self.index += 1;

        return value;
    }

    pub fn next(&mut self) {
        self.index +=1;
    }

}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstError> {
    let mut parser = AstParser::new(tokens);
    let mut program = Program::default();

    while parser.has_tokens() {
        return Err(AstError {
            index: parser.get_current_index(),
            error_type: AstErrorType::UnexpectedToken
        });
    }

    Ok(program)
}
