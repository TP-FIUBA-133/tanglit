use crate::doc::{ParserError, TangleError, generate_pdf::GeneratePdfError};
use std::fmt;

pub enum DocError {
    ParseError(ParserError),
    TangleError(TangleError),
    GeneratePdfError(GeneratePdfError),
}

impl fmt::Display for DocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocError::ParseError(e) => write!(f, "Error parsing blocks: {}", e),
            DocError::TangleError(e) => write!(f, "Error tangling block: {}", e),
            DocError::GeneratePdfError(e) => write!(f, "Error generating PDF: {}", e),
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

impl From<GeneratePdfError> for DocError {
    fn from(error: GeneratePdfError) -> Self {
        DocError::GeneratePdfError(error)
    }
}
