use crate::tokenizer::Token;

use super::{AstParseError, Program, AstErrorType, parsers::{is_expression_statement, parse_expression_statement}};

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

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstParseError> {
    let mut parser = AstParser::new(tokens);
    let mut program = Program::default();

    while parser.has_tokens() {
        if is_expression_statement(&parser) {
            parse_expression_statement(&mut parser);
            continue;
        }

        return Err(AstParseError {
            index: parser.get_current_index(),
            error_type: AstErrorType::UnexpectedToken
        });
    }

    Ok(program)
}
