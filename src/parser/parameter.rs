#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parameter {
    name: String,
}

impl Parameter {
    pub fn new(name: String) -> Self {
        Parameter { name }
    }
}
