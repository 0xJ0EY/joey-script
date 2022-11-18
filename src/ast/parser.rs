use crate::tokenizer::{Token, TokenType, Separator};

use super::{AstParseError, Program, AstErrorType, nodes::AstNode, parsers::{expression_statements::{parse_expression_statement, is_expression_statement}, block_statements::{is_closed_block_statement, is_open_block_statement, parse_block_statement}}};

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

        if current_token.is_none() || offending_token.is_none() { return false; }

        let current_token   = current_token.unwrap();
        let offending_token = offending_token.unwrap();

        let offending_token_is_on_a_different_line = || {
            current_token.loc.end.line != offending_token.loc.start.line
        };

        let offending_token_is_closing_bracket = || {
            matches!(offending_token.token_type, TokenType::Separator(Separator::CurlyBrace)) && offending_token.raw_value == "}"
        };

        offending_token_is_on_a_different_line() || offending_token_is_closing_bracket()
    }

    fn parse(&mut self) -> Result<AstNode, AstParseError> {
        if is_expression_statement(self) {
            let expression_statement = parse_expression_statement(self)?;
            return Ok(AstNode::ExpressionStatement(expression_statement));
        }

        return Err(AstParseError {
            index: self.get_current_index(),
            error_type: AstErrorType::UnexpectedToken
        })
    }

    pub fn parse_program(&mut self) -> Result<Vec<AstNode>, AstParseError> {
        let mut body: Vec<AstNode> = Vec::new();

        while self.has_tokens() {
            if is_closed_block_statement(self) {
                return Err(AstParseError {
                    index: self.get_current_index(),
                    error_type: AstErrorType::UnexpectedToken,
                });
            }

            if is_open_block_statement(self) {
                let block = parse_block_statement(self)?;
                let block_statement = AstNode::BlockStatement(block);

                body.push(block_statement);
                continue;
            }
    
            let node = self.parse()?;
            body.push(node);
        }

        Ok(body)
    }

    pub fn parse_block(&mut self) -> Result<Vec<AstNode>, AstParseError> {
        let mut body = Vec::new();

        while self.has_tokens() {
            if is_closed_block_statement(self) {
                self.next();
                return Ok(body);
            }

            if is_open_block_statement(self) {
                parse_block_statement(self)?;
                continue;
            }
    
            let node = self.parse()?;
            body.push(node);
        }

        return Err(AstParseError {
            index: self.get_current_index(),
            error_type: AstErrorType::UnexpectedEndOfInput
        })
    }

}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, AstParseError> {
    let mut parser = AstParser::new(tokens);
    let mut program = Program::default();

    program.body = parser.parse_program()?;

    Ok(program)
}
