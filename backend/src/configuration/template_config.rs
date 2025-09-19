use std::{fs, path::Path};

use regex::Regex;

use crate::errors::ConfigError;

const CONFIG_PLACEHOLDER_DEFAULT_PATTERN: &str = "#<([^#<>]+)>#";

#[derive(Debug, Clone)]
pub struct Template {
    // TODO: add configuration field to configure template parameters
    pub placeholder_pattern: Regex,
    pub template_content: String,
}

impl Template {
    /// Loads a template from a file path.
    pub fn load_from_file(
        file_path: &Path,
        placeholder_regex: Option<&str>,
    ) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(file_path)?;
        Self::load(&content, placeholder_regex)
    }

    /// Loads a template from the contents of a template file.
    pub fn load(content: &str, placeholder_regex: Option<&str>) -> Result<Self, ConfigError> {
        let placeholder = Regex::new(
            placeholder_regex.unwrap_or(CONFIG_PLACEHOLDER_DEFAULT_PATTERN),
        )
        .map_err(|e| ConfigError::InternalError(format!("Invalid placeholder regex: {}", e)))?;

        let template_content = content.to_string();

        Ok(Template {
            placeholder_pattern: placeholder,
            template_content,
        })
    }
}
