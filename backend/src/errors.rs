use std::fmt;
pub enum ParserError {
    InvalidInput(String),
    CodeBlockError(String),
    ConversionError(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::CodeBlockError(msg) => write!(f, "Error parsing Code Block: {}", msg),
            ParserError::ConversionError(msg) => {
                write!(f, "Error converting AST back to markdown: {}", msg)
            }
        }
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::CodeBlockError(msg) => write!(f, "Error parsing Code Block: {}", msg),
            ParserError::ConversionError(msg) => {
                write!(f, "Error converting AST back to markdown: {}", msg)
            }
        }
    }
}

#[derive(PartialEq)]
pub enum TangleError {
    BlockNotFound(String),
    InternalError(String),
}

impl fmt::Display for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl fmt::Debug for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

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
