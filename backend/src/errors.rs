use crate::doc::{DocError, TangleError};
use std::{fmt, io};

pub enum ExecutionError {
    DocError(DocError),
    WriteError(String),
    UnsupportedLanguage(String),
    InternalError(String),
    ImportError(String),
    ConfigError(ConfigError),
    ExecutionScriptNotFound,
    TemplateNotFound,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::DocError(e) => write!(f, "Document error: {}", e),
            ExecutionError::WriteError(msg) => write!(f, "Error writing file: {}", msg),
            ExecutionError::UnsupportedLanguage(msg) => write!(f, "Unsupported language: {}", msg),
            ExecutionError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ExecutionError::ImportError(msg) => write!(f, "Import codeblock error: {}", msg),
            ExecutionError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            ExecutionError::ExecutionScriptNotFound => {
                write!(f, "Execution script not found in language configuration")
            }
            ExecutionError::TemplateNotFound => {
                write!(f, "Template file not found in language configuration")
            }
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
            ExecutionError::ImportError(msg) => write!(f, "Import codeblock error: {}", msg),
            ExecutionError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            ExecutionError::ExecutionScriptNotFound => {
                write!(f, "Execution script not found in language configuration")
            }
            ExecutionError::TemplateNotFound => {
                write!(f, "Template file not found in language configuration")
            }
        }
    }
}

impl From<DocError> for ExecutionError {
    fn from(error: DocError) -> Self {
        ExecutionError::DocError(error)
    }
}

impl From<TangleError> for ExecutionError {
    fn from(error: TangleError) -> Self {
        ExecutionError::DocError(DocError::from(error))
    }
}

impl From<ConfigError> for ExecutionError {
    fn from(error: ConfigError) -> Self {
        ExecutionError::ConfigError(error)
    }
}

pub enum ConfigError {
    IoError(String),
    ParseError(String),
    NotFound(String),
    InternalError(String),
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::NotFound(msg) => write!(f, "Config not found for lang: {}", msg),
            ConfigError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::IoError(error.to_string())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::ParseError(error.to_string())
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ConfigError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}
