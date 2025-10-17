use super::parameter::Parameter;
use super::token::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    name: String,
    params: Vec<Parameter>,
    body: Box<Expression>,
}

impl Function {
    pub fn new(name: String, params: Vec<Parameter>, body: Expression) -> Self {
        Function {
            name,
            params,
            body: Box::new(body),
        }
    }
}
