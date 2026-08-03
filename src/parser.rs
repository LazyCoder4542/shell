pub enum Token {
    Word(String)
}
pub struct Parser {}

impl Parser {
    pub fn parse(input: &str) -> Vec<Token> {
        input.split(" ").map(|x| {
            Token::Word(x.to_string())
        }).collect()
    }
}