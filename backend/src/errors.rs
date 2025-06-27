use std::fmt;
pub enum ParserError {
    InvalidInput(String),
    UnexpectedToken(String), // currently not used
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

pub enum TangleError {
    BlockNotFound(String),
    LanguageNotSupported(String),
}

impl fmt::Display for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::LanguageNotSupported(lang) => {
                write!(
                    f,
                    "Programming language not supported for execution: {}",
                    lang
                )
            }
        }
    }
}

impl fmt::Debug for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::LanguageNotSupported(lang) => {
                write!(
                    f,
                    "Programming language not supported for execution: {}",
                    lang
                )
            }
        }
    }
}
