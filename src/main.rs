mod tokenizer;
mod ast;

fn main() {
    let input = String::from(include_str!("input.js"));
    let tokens = tokenizer::parse(&input);

    println!("{:?}", tokens);
}
