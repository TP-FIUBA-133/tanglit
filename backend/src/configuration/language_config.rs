use serde::Deserialize;
use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};

use crate::configuration::get_config_dir;
use crate::errors::ConfigError;

const CONFIG_PLACEHOLDER_DEFAULT_PATTERN: &str = "#<([^#<>]+)>#";
const TEMPLATE_FILENAME: &str = "template";
const EXECUTE_SCRIPT_FILENAME: &str = "execute";

#[derive(Deserialize, Clone)]
pub struct LanguageConfig {
    pub extension: Option<String>,
    pub placeholder_regex: Option<String>, // If empty, we'll use the default
    #[serde(skip)]
    pub template: String,
    #[serde(skip)]
    pub execution_script_path: String,
}

impl LanguageConfig {
    pub fn load_for_lang(lang: &str) -> Result<LanguageConfig, ConfigError> {
        let config_dir = get_config_dir();
        let config_path = &config_dir.join("executors").join(lang).join("config.toml");
        let mut config = LanguageConfig::load_from_file(config_path)?;
        let template_path = find_file_in_dir(&config_dir, TEMPLATE_FILENAME).ok_or(
            ConfigError::NotFound(format!("{config_dir:?}/{TEMPLATE_FILENAME}")),
        )?;
        config.template = read_to_string(template_path)?;
        let execution_script_path = find_file_in_dir(&config_dir, EXECUTE_SCRIPT_FILENAME).ok_or(
            ConfigError::InternalError("Execution script not found".to_string()),
        )?;
        config.execution_script_path = execution_script_path.to_string_lossy().to_string();
        Ok(config)
    }

    pub fn load_from_file(path: &PathBuf) -> Result<LanguageConfig, ConfigError> {
        let content = fs::read_to_string(path)?;
        LanguageConfig::load_from_str(&content)
    }

    pub fn load_from_str(toml_str: &str) -> Result<LanguageConfig, ConfigError> {
        let mut config: LanguageConfig = toml::from_str(toml_str)?;
        config.placeholder_regex = Some(
            config
                .placeholder_regex
                .unwrap_or(CONFIG_PLACEHOLDER_DEFAULT_PATTERN.into()),
        );
        Ok(config)
    }
}

pub fn find_file_in_dir(dir: &Path, filename: &str) -> Option<PathBuf> {
    fs::read_dir(dir).ok().and_then(|entries| {
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|path| {
                path.file_stem()
                    .map(|stem| stem == filename)
                    .unwrap_or(false)
            })
    })
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
