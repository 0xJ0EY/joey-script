use super::tokenizer::Tokenizer;

pub mod whitespace;
pub mod number;
pub mod identifier;
pub mod string;
pub mod comments;
pub mod operator;
pub mod terminator;
pub mod seperator;
pub mod parenthesis;
pub mod curly_brace;
pub mod boolean;
pub mod null;
pub mod keywords;

#[macro_export]
macro_rules! tokenize_error {
    ($a: expr, $b: expr) => {
        Err(TokenizeError { error_type: $a, index: $b.get_current_index() })
    };
}

fn is_word(tokenizer: &Tokenizer, word: &str) -> bool {
    let mut chars = word.chars();

    for index in 0..word.len() {
        let token_char = tokenizer.peek_forward(index);
        let word_char   = chars.next();

        if token_char.is_none() || word_char.is_none() { return false }
        if token_char.unwrap().to_owned() != word_char.unwrap() { return false }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::tokenizer::Tokenizer;

    #[test]
    fn foobar_equals_foobar() {
        let input = String::from_str("foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);
        let word = "foobar";
        
        let result = super::is_word(&tokenizer, word);

        assert_eq!(result, true);
    }

    #[test]
    fn barfoo_equals_foobar() {
        let input = String::from_str("barfoo").unwrap();
        let tokenizer = Tokenizer::new(&input);
        let word = "foobar";
        
        let result = super::is_word(&tokenizer, word);

        assert_eq!(result, false);
    }

}
