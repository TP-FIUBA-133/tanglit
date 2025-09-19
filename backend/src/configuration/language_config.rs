use crate::configuration::template_config::Template;
use crate::errors::ConfigError;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_RUST_CONFIG: &str = include_str!("../../resources/config/executors/rust/config.toml");
const DEFAULT_PYTHON_CONFIG: &str =
    include_str!("../../resources/config/executors/python/config.toml");

enum DefaultLanguages {
    Rust,
    Python,
}

impl DefaultLanguages {
    fn from_str(lang: &str) -> Result<DefaultLanguages, ConfigError> {
        match lang.to_lowercase().as_str() {
            "rust" => Ok(DefaultLanguages::Rust),
            "python" => Ok(DefaultLanguages::Python),
            _ => {
                return Err(ConfigError::NotFound(format!(
                    "No default configuration for language: {}",
                    lang
                )));
            }
        }
    }

    fn get_default_config(&self) -> &'static str {
        match self {
            DefaultLanguages::Rust => DEFAULT_RUST_CONFIG,
            DefaultLanguages::Python => DEFAULT_PYTHON_CONFIG,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct LanguageConfig {
    pub extension: Option<String>,
    #[serde(skip)]
    pub config_dir: Option<PathBuf>,
    pub placeholder_regex: Option<String>, // If empty, we'll use the default
    pub template: Option<Template>,
}

impl LanguageConfig {
    pub fn load_for_lang(lang: &str) -> Result<LanguageConfig, ConfigError> {
        let config_dir = crate::configuration::get_config_dir();
        let config_path = &config_dir.join("executors").join(lang).join("config.toml");
        if config_path.exists() {
            return LanguageConfig::load_from_file(config_path);
        }
        LanguageConfig::load_default(lang)
    }
    pub fn load_from_file(path: &PathBuf) -> Result<LanguageConfig, ConfigError> {
        let content = fs::read_to_string(path)?;
        let mut lang_config = LanguageConfig::load_from_str(&content)?;
        lang_config.config_dir = Some(path.parent().unwrap().to_path_buf());
        Ok(lang_config)
    }

    pub fn load_from_str(toml_str: &str) -> Result<LanguageConfig, ConfigError> {
        let config: LanguageConfig = toml::from_str(toml_str)?;
        Ok(config)
    }

    pub fn load_default(lang: &str) -> Result<LanguageConfig, ConfigError> {
        let default_lang = DefaultLanguages::from_str(lang)?;
        let toml_str = default_lang.get_default_config();
        let lang_config = LanguageConfig::load_from_str(toml_str)?;
        Ok(lang_config)
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::language_config::LanguageConfig;

    #[test]
    fn test_load_config() {
        let config = LanguageConfig::load_from_str(
            "
            extension = 'rs'\n\
            placeholder_regex = '<WAWA>'",
        )
        .unwrap();
        assert_eq!(config.extension, Option::from("rs".to_string()));
        assert_eq!(config.placeholder_regex, Option::from("<WAWA>".to_string()));
    }
}
