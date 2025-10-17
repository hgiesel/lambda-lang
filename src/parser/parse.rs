use super::function::Function;
use super::token::Expression;
use super::variable::Variable;
use crate::lexer::Token as LexerToken;

pub struct Parser {
    tokens: Vec<LexerToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<LexerToken> {
        self.tokens.get(self.pos).cloned()
    }

    pub fn parse_func(&mut self) -> Option<Function> {
        let name = match self.current() {
            Some(LexerToken::Text(val)) => {
                self.pos += 1;
                val
            }
            _ => return None,
        };

        if self.current() != Some(LexerToken::ParenthesisOpen) {
            return None;
        } else {
            self.pos += 1;
        }

        if self.current() != Some(LexerToken::ParenthesisClosed) {
            return None;
        } else {
            self.pos += 1;
        }

        let mut func = Function::new(
            name.into(),
            Vec::new(),
            Expression::Variable(Variable::new("bar".into())),
        );

        Some(func)
    }
}
