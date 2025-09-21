use serde::Deserialize;
use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};

use crate::configuration::get_config_dir;
use crate::errors::ConfigError;

pub const PLACEHOLDER_DEFAULT_PATTERN: &str = "#<([^#<>]+)>#";
const TEMPLATE_FILENAME: &str = "template";
const EXECUTION_SCRIPT_FILENAME: &str = "execute";
const EXECUTORS_DIRNAME: &str = "executors";
const TOML_CONFIG_FILENAME: &str = "config.toml";

#[derive(Deserialize, Clone)]
pub struct LanguageConfig {
    pub extension: Option<String>,
    pub placeholder_regex: Option<String>, // If empty, we'll use the default
    #[serde(skip)]
    pub template: Option<String>,
    #[serde(skip)]
    pub execution_script_path: Option<PathBuf>,
}

impl LanguageConfig {
    pub fn load_for_lang(lang: &str) -> Result<LanguageConfig, ConfigError> {
        let lang_config_path = &get_config_dir().join(EXECUTORS_DIRNAME).join(lang);
        let toml_path = lang_config_path.join(TOML_CONFIG_FILENAME);
        let mut config = LanguageConfig::load_from_file(&toml_path)?;
        config.template = find_file_in_dir(lang_config_path, TEMPLATE_FILENAME)
            .map(read_to_string)
            .transpose()?;
        config.execution_script_path =
            find_file_in_dir(lang_config_path, EXECUTION_SCRIPT_FILENAME);
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
                .unwrap_or(PLACEHOLDER_DEFAULT_PATTERN.into()),
        );
        Ok(config)
    }
}

pub fn find_file_in_dir(dir: impl AsRef<Path>, filename: &str) -> Option<PathBuf> {
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
