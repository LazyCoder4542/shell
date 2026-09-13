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
        let mut d_escape: bool = false;
        for c in input.chars() {
          if singleQ {
            if c != '\'' {
              curr.push(c);
            }
            else {singleQ = false;}
          }
          else if d_escape {
            match c {
                '\\' | '"' | '$' => curr.push(c),
                'n' => curr.push('\n'),
                _ => {
                  curr.push('\\');
                  curr.push(c);
                }
            }
            d_escape = false;
          }
          else if doubleQ {
            if c == '\\' {
              d_escape = true;
            }
            else if c != '"' {
              curr.push(c);
            }
            else {doubleQ = false;}
          }
          else if escape {
            curr.push(c);
            escape = false;
          }
          else {
            match c {
              '\\' => {
                escape = true;
              }
              '\'' => {
                singleQ = true;
              }
              '"' => {
                doubleQ = !doubleQ;
              }
              ' ' => {
                if !curr.is_empty() {
                  result.push(Token::Word(std::mem::take(&mut curr)));
                }
              }
              _ => curr.push(c)
            }
          }
        }
        if !curr.is_empty() {
          result.push(Token::Word(std::mem::take(&mut curr)));
        }
        result
    }
}
