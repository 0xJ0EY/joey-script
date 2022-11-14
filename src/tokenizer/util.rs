extern crate regex;
use regex::Regex;
use once_cell::sync::Lazy;

static REGEX_WHITESPACE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s").unwrap()
});

static REGEX_IDENTIFIER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-zA-Z]").unwrap()
});

static REGEX_NUMBER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[0-9]").unwrap()
});

static REGEX_EOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\n").unwrap()
});

pub fn is_whitespace(token: &char) -> bool {   
    REGEX_WHITESPACE.is_match(&(*token).to_string())
}

pub fn is_identifier(token: &char) -> bool {
    REGEX_IDENTIFIER.is_match(&(*token).to_string())
}

pub fn is_number(token: &char) -> bool {
    REGEX_NUMBER.is_match(&(*token).to_string())
}

pub fn is_separator(token: &char) -> bool {
    *token == '.' || *token == ','
}

pub fn is_period(token: &char) -> bool {
    *token == '.'
}

pub fn is_comma(token: &char) -> bool {
    *token == ','
}

pub fn is_parenthesis(token: &char) -> bool {
    *token == '(' || *token == ')'
}

pub fn is_terminator(token: &char) -> bool {
    *token == ';'
}

pub fn is_curly_brace(token: &char) -> bool {
    *token == '{' || *token == '}'
}

pub fn is_eol(token: &char) -> bool {
    REGEX_EOL.is_match(&(*token).to_string())
}

pub fn is_operator(token: &char) -> bool {
    match *token {
        '=' |
        '>' |
        '<' | 
        '!' |
        '+' |
        '-' |
        '/' |
        '*' |
        '%' |
        '&' |
        '|' |
        '^' |
        '~' => true,
        _ => false
    }
}

pub fn is_escape_char(token: &char) -> bool {
    *token == '\\'
}

pub fn is_string_delimiter(token: &char) -> bool {
    *token == '\'' || *token == '\"'
}
