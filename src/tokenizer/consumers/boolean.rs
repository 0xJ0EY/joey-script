use std::vec;

use crate::tokenizer::{tokenizer::Tokenizer, TokenizeError, Token};

fn is_true(tokenizer: &Tokenizer) -> bool {
    let word = vec!['t', 'r', 'u', 'e'];

    for index in 0..word.len() {
        let char = tokenizer.peek_forward(index);
        
        if char.is_none() { return false }
    }

    true
} 

fn is_false(tokenizer: &Tokenizer) -> bool {
    let word = vec!['f', 'a', 'l', 's', 'e'];

    for index in 0..word.len() {
        let char = tokenizer.peek_forward(index);
        
        if char.is_none() { return false }
    }

    true
} 

pub fn is_boolean(tokenizer: &Tokenizer) -> bool {
    is_true(tokenizer) || is_false(tokenizer)
}


pub fn consume_boolean(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    todo!()
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::tokenizer::Tokenizer;

    #[test]
    fn true_is_true_is_true() {
        let input = String::from_str("true").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_true(&tokenizer);

        assert_eq!(result, true);
    }   
    
    #[test]
    fn false_is_false_is_true() {
        let input = String::from_str("false").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_false(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn false_is_boolean_is_true() {
        let input = String::from_str("false").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_boolean(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn true_is_boolean_is_true() {
        let input = String::from_str("true").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_boolean(&tokenizer);

        assert_eq!(result, true);
    }

}


