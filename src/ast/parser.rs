use crate::tokenizer::{Token, TokenType, Separator};

use super::{AstParseError, Program, AstErrorType, nodes::AstNode, parsers::expression_statements::{is_expression_statement, parse_expression_statement}};

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

    pub fn can_insert_automatic_semicolon(&self, index: usize) -> bool {
        // https://262.ecma-international.org/13.0/#sec-rules-of-automatic-semicolon-insertion
        // TODO: missing "The previous token is ) and the inserted semicolon would then be parsed as the terminating semicolon of a do-while statement (14.7.2)."
        let current_token   = self.tokens.get(index);
        let offending_token = self.tokens.get(index + 1);

        println!("{:?}", current_token);
        println!("{:?}", offending_token);

        if current_token.is_none() || offending_token.is_none() { return false; }

        let current_token   = current_token.unwrap();
        let offending_token = offending_token.unwrap();

        let offending_token_is_on_a_different_line = || {
            current_token.loc.end.line != offending_token.loc.start.line
        };

        let offending_token_is_closing_bracket = || {
            matches!(offending_token.token_type, TokenType::Separator(Separator::CurlyBrace)) && offending_token.raw_value == "}"
        };

        println!("{:?}", offending_token_is_on_a_different_line());

        offending_token_is_on_a_different_line() || offending_token_is_closing_bracket()
    }

}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstParseError> {
    let mut parser = AstParser::new(tokens);
    let mut program = Program::default();

    while parser.has_tokens() {
        if is_expression_statement(&parser) {
            let expression_statement = parse_expression_statement(&mut parser)?;
            program.body.push(AstNode::ExpressionStatement(expression_statement));
            continue;
        }

        return Err(AstParseError {
            index: parser.get_current_index(),
            error_type: AstErrorType::UnexpectedToken
        });
    }

    Ok(program)
}
