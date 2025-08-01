use crate::doc::DocError;
use std::fmt;

pub enum ExecutionError {
    DocError(DocError),
    WriteError(String),
    UnsupportedLanguage(String),
    InternalError(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::DocError(e) => write!(f, "Document error: {}", e),
            ExecutionError::WriteError(msg) => write!(f, "Error writing file: {}", msg),
            ExecutionError::UnsupportedLanguage(msg) => write!(f, "Unsupported language: {}", msg),
            ExecutionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl fmt::Debug for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::DocError(e) => write!(f, "Document error: {}", e),
            ExecutionError::WriteError(msg) => write!(f, "Error writing file: {}", msg),
            ExecutionError::UnsupportedLanguage(msg) => write!(f, "Unsupported language: {}", msg),
            ExecutionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl From<DocError> for ExecutionError {
    fn from(error: DocError) -> Self {
        ExecutionError::DocError(error)
    }
}
