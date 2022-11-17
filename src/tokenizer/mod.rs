mod tokenizer;
mod util;
mod consumers;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Literal(Literal),
    Separator(Separator),
    Operator,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number,
    String,
    Boolean,
    Null
}

#[derive(Debug, PartialEq)]
pub enum Separator {
    Period,
    Comma,
    Parenthesis,
    CurlyBrace,
    Terminator,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub raw_value: String,
    pub range: (usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum TokenErrorType {
    UnexpectedToken,
    UnterminatedStringLiteral
}

#[derive(Debug)]
pub struct TokenizeError {
    pub error_type: TokenErrorType,
    pub index: usize
}

pub fn parse(file_content: &String) -> Result<Vec<Token>, TokenizeError> {
    tokenizer::parse(file_content)
}
