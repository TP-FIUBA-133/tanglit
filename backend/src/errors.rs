use std::fmt;
pub enum ParserError {
    InvalidInput(String),
    CodeBlockError(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::CodeBlockError(msg) => write!(f, "Error parsing Code Block: {}", msg),
        }
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::CodeBlockError(msg) => write!(f, "Error parsing Code Block: {}", msg),
        }
    }
}

pub enum TangleError {
    BlockNotFound(String),
}

impl fmt::Display for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
        }
    }
}

impl fmt::Debug for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
        }
    }
}
