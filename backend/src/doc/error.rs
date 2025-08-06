use crate::doc::{ParserError, TangleError};
use std::fmt;

pub enum DocError {
    ParseError(ParserError),
    TangleError(TangleError),
}

impl fmt::Display for DocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocError::ParseError(e) => write!(f, "Error parsing blocks: {}", e),
            DocError::TangleError(e) => write!(f, "Error tangling block: {}", e),
        }
    }
}

impl From<ParserError> for DocError {
    fn from(error: ParserError) -> Self {
        DocError::ParseError(error)
    }
}

impl From<TangleError> for DocError {
    fn from(error: TangleError) -> Self {
        DocError::TangleError(error)
    }
}
