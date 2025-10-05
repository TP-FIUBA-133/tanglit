use crate::errors::ConfigError;

// Default toml configurations for supported languages
pub const DEFAULT_TOML_CONFIG_RUST: &str =
    include_str!("../../../resources/config/executors/rust/config.toml");
pub const DEFAULT_TOML_CONFIG_PYTHON: &str =
    include_str!("../../../resources/config/executors/rust/config.toml");
pub const DEFAULT_TOML_CONFIG_C: &str =
    include_str!("../../../resources/config/executors/rust/config.toml");

// Default templates content for supported languages
pub const DEFAULT_TEMPLATE_RUST: &str =
    include_str!("../../../resources/config/executors/rust/template");
pub const DEFAULT_TEMPLATE_PYTHON: &str =
    include_str!("../../../resources/config/executors/python/template");
pub const DEFAULT_TEMPLATE_C: &str = include_str!("../../../resources/config/executors/c/template");

// Default execution scripts for supported languages
pub const DEFAULT_EXECUTION_SCRIPT_RUST: &str =
    include_str!("../../../resources/config/executors/rust/execute.sh");
pub const DEFAULT_EXECUTION_SCRIPT_PYTHON: &str =
    include_str!("../../../resources/config/executors/python/execute.sh");
pub const DEFAULT_EXECUTION_SCRIPT_C: &str =
    include_str!("../../../resources/config/executors/c/execute.sh");

pub fn get_default_toml(lang: &str) -> Result<String, ConfigError> {
    let content = match lang {
        "rust" => DEFAULT_TOML_CONFIG_RUST.into(),
        "python" => DEFAULT_TOML_CONFIG_PYTHON.into(),
        "c" => DEFAULT_TOML_CONFIG_C.into(),
        _ => Err(ConfigError::NotFound(lang.into()))?,
    };
    Ok(content)
}

/// Returns the default template for a given language, if available.
/// We don't return an error if not found, as it's optional.
pub fn get_default_template(lang: &str) -> Option<String> {
    match lang {
        "rust" => Some(DEFAULT_TEMPLATE_RUST.into()),
        "python" => Some(DEFAULT_TEMPLATE_PYTHON.into()),
        "c" => Some(DEFAULT_TEMPLATE_C.into()),
        _ => None,
    }
}

/// Returns the default execution script for a given language, if available.
/// We don't return an error if not found, as it's optional.
pub fn get_default_execution_script(lang: &str) -> Option<String> {
    match lang {
        "rust" => Some(DEFAULT_EXECUTION_SCRIPT_RUST.into()),
        "python" => Some(DEFAULT_EXECUTION_SCRIPT_PYTHON.into()),
        "c" => Some(DEFAULT_EXECUTION_SCRIPT_C.into()),
        _ => None,
    }
}

// TODO: Review in which cases we want to return an error instead of Option
// For example, we don't need a template if we are excluding content from slides.
