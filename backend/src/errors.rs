use std::fmt;
pub enum ParserError {
    InvalidInput(String),
    UnexpectedToken(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
        }
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
        }
    }
}
