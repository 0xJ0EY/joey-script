use crate::tokenizer::consumers::curly_brace::consume_curly_brace;
use crate::tokenizer::consumers::keywords::consume_keyword;
use crate::tokenizer::consumers::parenthesis::consume_parenthesis;
use crate::tokenizer::consumers::separator::{consume_period, consume_comma};

use super::consumers::boolean::{is_boolean, consume_boolean};
use super::consumers::comments::{is_line_comment, is_block_comment, consume_line_comment, consume_block_comment};
use super::consumers::curly_brace::is_curly_brace;
use super::consumers::keywords::find_keyword;
use super::consumers::null::{is_null, consume_null};
use super::consumers::operator::{is_operator, consume_operator};
use super::consumers::parenthesis::is_parenthesis;
use super::consumers::separator::{is_period, is_comma};
use super::consumers::string::{is_string, consume_string};
use super::consumers::terminator::{is_terminator, consume_terminator};
use super::{Token, TokenizeError};
use super::consumers::identifier::{is_identifier, consume_identifier};
use super::consumers::number::{is_number, consume_number};
use super::consumers::whitespace::{is_whitespace, consume_whitespace};

#[derive(Debug)]
pub struct Tokenizer {
    index: usize,
    file_content: Vec<char>,
}

impl Tokenizer {
    pub fn new(file_content: &String) -> Tokenizer {
        Tokenizer {
            index: 0,
            file_content: file_content.chars().collect()
        }
    }

    pub fn has_tokens(&self) -> bool {
        self.token().is_some()
    }

    pub fn token(&self) -> Option<&char> {
        self.file_content.get(self.index)
    }

    pub fn peek(&self) -> Option<&char> {
        self.file_content.get(self.index + 1)
    }

    pub fn peek_forward(&self, distance: usize) -> Option<&char> {
        self.file_content.get(self.index + distance)
    }

    pub fn peek_back(&self) -> Option<&char> {
        self.file_content.get(self.index - 1)
    }

    pub fn walk_back(&mut self) {
        self.index -= 1;
    }

    pub fn get_current_index(&self) -> usize {
        return self.index;
    }

    pub fn consume(&mut self) -> Option<&char> {
        let value = self.file_content.get(self.index);

        self.index += 1;

        return value;
    }

    pub fn next(&mut self) {
        self.index +=1;
    }
}

macro_rules! consume_and_handle {
    ($a: expr, $b: expr) => {
        match $a {
            Ok(token) => $b.push(token),
            Err(e) => return Err(e),
        }
    };
}

pub fn parse(file_content: &String) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut tokenizer = Tokenizer::new(file_content);

    while tokenizer.has_tokens() {
        if is_whitespace(&tokenizer) {

            let result = consume_whitespace(&mut tokenizer);

            if result.is_err() {
                return Err(result.unwrap_err())
            }

            continue;
        }

        if is_line_comment(&tokenizer) {
            consume_line_comment(&mut tokenizer);
            continue;
        }

        if is_block_comment(&tokenizer) {
            consume_block_comment(&mut tokenizer);
            continue;
        }

        if let Ok(keyword) = find_keyword(&tokenizer) {
            consume_and_handle!(consume_keyword(&mut tokenizer, keyword), tokens);
            continue;
        }

        if is_boolean(&tokenizer) {
            consume_and_handle!(consume_boolean(&mut tokenizer), tokens);
            continue;
        }

        if is_null(&tokenizer) {
            consume_and_handle!(consume_null(&mut tokenizer), tokens);
            continue;
        }

        if is_terminator(&tokenizer) {
            consume_and_handle!(consume_terminator(&mut tokenizer), tokens);
            continue;
        }

        if is_period(&tokenizer) {
            consume_and_handle!(consume_period(&mut tokenizer), tokens);
            continue;
        }

        if is_comma(&tokenizer) {
            consume_and_handle!(consume_comma(&mut tokenizer), tokens);
            continue;
        }

        if is_parenthesis(&tokenizer) {
            consume_and_handle!(consume_parenthesis(&mut tokenizer), tokens);
            continue;
        }

        if is_curly_brace(&tokenizer) {
            consume_and_handle!(consume_curly_brace(&mut tokenizer), tokens);
            continue;
        }

        if is_number(&tokenizer) {
            consume_and_handle!(consume_number(&mut tokenizer), tokens);
            continue;
        }

        if is_identifier(&tokenizer) {
            consume_and_handle!(consume_identifier(&mut tokenizer), tokens);
            continue;
        }

        if is_string(&tokenizer) {
            consume_and_handle!(consume_string(&mut tokenizer), tokens);
            continue;
        }

        if is_operator(&tokenizer) {
            consume_and_handle!(consume_operator(&mut tokenizer), tokens);
            continue;
        }

        return Err(TokenizeError {
            error_type: super::TokenErrorType::UnexpectedToken,
            index: tokenizer.get_current_index(),
        })
    }

    return Ok(tokens);
}

