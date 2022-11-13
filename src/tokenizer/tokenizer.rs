use crate::tokenizer::consumers::seperator::consume_seperator;

use super::consumers::comments::{is_line_comment, is_block_comment, consume_line_comment, consume_block_comment};
use super::consumers::operator::{is_operator, consume_operator};
use super::consumers::seperator::is_separator;
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

    pub fn token_at(&self, index: usize) -> Option<&char> {
        self.file_content.get(index)
    }

    pub fn peek(&self) -> Option<&char> {
        self.file_content.get(self.index + 1)
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
            consume_whitespace(&mut tokenizer);
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

        if is_terminator(&tokenizer) {
            consume_and_handle!(consume_terminator(&mut tokenizer), tokens);
            continue;
        }

        if is_separator(&tokenizer) {
            consume_and_handle!(consume_seperator(&mut tokenizer), tokens);
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
        }

        return Err(TokenizeError {
            error_type: super::TokenErrorType::UnexpectedToken,
            index: tokenizer.get_current_index(),
        })
    }

    return Ok(tokens);
}

