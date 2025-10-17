use super::variable::Variable;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Variable(Variable),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Variable(String),
    Expression(Vec<Token>),
}

// id = (1, 2).first + 3
// id = <T. Eq a>(x: T). x y
