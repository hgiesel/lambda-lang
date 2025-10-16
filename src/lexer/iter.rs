use super::token::Token;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Counter {
    index: usize,
    input: String,
}

impl Counter {
    fn new(input: &str) -> Self {
        Counter {
            index: 0,
            input: input.to_string(),
        }
    }

    pub fn print_all(&mut self) {
        while let Some(token) = self.next() {
            println!("{}: {:?}", self.index, token);
        }
    }
}

impl Iterator for Counter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let re = Lazy::new(|| Regex::new(r"^\w+").unwrap());

        while let Some(rem) = self.input.get(self.index..) {
            if rem.starts_with('.') {
                self.index += 1;
                return Some(Token::Dot);
            } else if rem.starts_with('\\') {
                self.index += 1;
                return Some(Token::Backslash);
            } else if rem.starts_with('(') {
                self.index += 1;
                return Some(Token::ParenthesisOpen);
            } else if rem.starts_with(')') {
                self.index += 1;
                return Some(Token::ParenthesisClosed);
            } else if let Some(found) = re.find(rem) {
                self.index += found.len();
                return Some(Token::Text(found.as_str().to_string()));
            } else {
                self.index += 1;
            }
        }
        None
    }
}

pub fn foo(input: &str) -> Counter {
    Counter::new(input)
}
