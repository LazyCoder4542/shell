#[derive(Debug)]
pub enum Token {
    Word(String),
}
pub struct Parser {}

impl Parser {
    pub fn parse(input: &str) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        let mut curr = String::new();
        let mut b: bool = false;
        for c in input.chars() {
          match c {
            '\'' => {
              b = !b;
            }
            ' ' => {
              if b {curr.push(' ');}
              else if !curr.is_empty() {
                result.push(Token::Word(std::mem::take(&mut curr)));
              }
            }
            _ => curr.push(c)
          }
        }
        if !curr.is_empty() {
          result.push(Token::Word(std::mem::take(&mut curr)));
        }
        result
    }
}
