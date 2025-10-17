#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
}
