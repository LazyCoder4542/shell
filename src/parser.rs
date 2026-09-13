#[derive(Debug)]
pub enum Token {
    Word(String),
}
pub struct Parser {}

impl Parser {
    pub fn parse(input: &str) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        let mut curr = String::new();
        let mut singleQ: bool = false;
        let mut doubleQ: bool = false;
        let mut escape: bool = false;
        for c in input.chars() {
          if escape {
            curr.push(c);
            escape = false;
            continue;
          }
          match c {
            '\\' => {
              escape = true;
            }
            '\'' => {
              if !doubleQ {singleQ = !singleQ;}
              else {curr.push('\'');}
            }
            '"' => {
              if !singleQ {doubleQ = !doubleQ;}
              else {curr.push('"');}
            }
            ' ' => {
              if singleQ || doubleQ {curr.push(' ');}
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
