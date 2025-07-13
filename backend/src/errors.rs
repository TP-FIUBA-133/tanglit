use crate::doc::{ParserError, TangleError};
use std::fmt;

pub enum ExecutionError {
    ParseError(ParserError),
    TangleError(TangleError),
    WriteError(String),
    UnsupportedLanguage(String),
    InternalError(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::ParseError(e) => write!(f, "Error parsing blocks: {}", e),
            ExecutionError::TangleError(e) => write!(f, "Error tangling block: {}", e),
            ExecutionError::WriteError(msg) => write!(f, "Error writing file: {}", msg),
            ExecutionError::UnsupportedLanguage(msg) => write!(f, "Unsupported language: {}", msg),
            ExecutionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl fmt::Debug for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::ParseError(e) => write!(f, "Error parsing blocks: {}", e),
            ExecutionError::TangleError(e) => write!(f, "Error tangling block: {}", e),
            ExecutionError::WriteError(msg) => write!(f, "Error writing file: {}", msg),
            ExecutionError::UnsupportedLanguage(msg) => write!(f, "Unsupported language: {}", msg),
            ExecutionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl From<ParserError> for ExecutionError {
    fn from(error: ParserError) -> Self {
        ExecutionError::ParseError(error)
    }
}

impl From<TangleError> for ExecutionError {
    fn from(error: TangleError) -> Self {
        ExecutionError::TangleError(error)
    }
}
