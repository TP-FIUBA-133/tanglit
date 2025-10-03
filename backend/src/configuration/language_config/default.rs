use crate::errors::ConfigError;

pub const DEFAULT_TOML_CONFIG_RUST: &str =
    include_str!("../../../resources/config/executors/rust/config.toml");
pub const DEFAULT_TOML_CONFIG_PYTHON: &str =
    include_str!("../../../resources/config/executors/rust/config.toml");
pub const DEFAULT_TOML_CONFIG_C: &str =
    include_str!("../../../resources/config/executors/rust/config.toml");

pub fn get_default_toml(lang: &str) -> Result<String, ConfigError> {
    let content = match lang {
        "rust" => DEFAULT_TOML_CONFIG_RUST.into(),
        "python" => DEFAULT_TOML_CONFIG_PYTHON.into(),
        "c" => DEFAULT_TOML_CONFIG_C.into(),
        _ => Err(ConfigError::NotFound(lang.into()))?,
    };
    Ok(content)
}
