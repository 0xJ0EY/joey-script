use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::util as util;

pub fn is_whitespace(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_whitespace(token)
}

pub fn consume_whitespace(tokenizer: &mut Tokenizer) {
    tokenizer.consume();
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::tokenizer::Tokenizer;

    #[test]
    fn is_whitespace_input_whitespace() {
        let input = String::from_str(" ").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_whitespace(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_text_input_not_whitespace() {
        let input = String::from_str("Foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_whitespace(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_whitespace_input() {
        let input = String::from_str(" ").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        super::consume_whitespace(&mut tokenizer);

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

}
