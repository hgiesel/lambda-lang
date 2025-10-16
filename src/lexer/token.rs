#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Backslash,
    Dot,
    ParenthesisOpen,
    ParenthesisClosed,
    Text(String),
}
