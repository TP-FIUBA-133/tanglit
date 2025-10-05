mod default;
use serde::Deserialize;
use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};

use crate::configuration::get_config_dir;
use crate::configuration::language_config::default::{
    get_default_execution_script, get_default_template, get_default_toml,
};
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
    pub execution_script: Option<String>,
}

impl LanguageConfig {
    pub fn load_for_lang(lang: &str) -> Result<LanguageConfig, ConfigError> {
        let lang_config_path = &get_config_dir().join(EXECUTORS_DIRNAME).join(lang);
        let toml_path = lang_config_path.join(TOML_CONFIG_FILENAME);
        let mut config = LanguageConfig::load_from_file(&toml_path, lang)?;
        config.template = match find_file_in_dir(lang_config_path, TEMPLATE_FILENAME) {
            Some(path) => read_to_string(path).ok(),
            None => get_default_template(lang),
        };
        config.execution_script =
            match find_file_in_dir(lang_config_path, EXECUTION_SCRIPT_FILENAME) {
                Some(path) => read_to_string(path).ok(),
                None => get_default_execution_script(lang),
            };

        Ok(config)
    }

    pub fn load_from_file(path: &PathBuf, lang: &str) -> Result<LanguageConfig, ConfigError> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => get_default_toml(lang).ok_or_else(|| {
                ConfigError::ConfigMissingForLanguage(
                    lang.to_string(),
                    path.to_string_lossy().to_string(),
                )
            })?,
        };

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
    use temp_env::with_var;

    use crate::configuration::language_config::LanguageConfig;
    use crate::configuration::user::CONFIG_DIR_ENVVAR;
    use crate::errors::ConfigError;

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

    #[test]
    fn test_config_missing_for_language_error() {
        let result = LanguageConfig::load_for_lang("nonexistent_language_12345");
        assert!(matches!(
            result,
            Err(ConfigError::ConfigMissingForLanguage(lang, _))
            if lang == "nonexistent_language_12345"
        ));
    }

    #[test]
    fn test_load_default_config_rust() {
        // Use a random directory to ensure it doesn't exist
        let random_dir = "/tmp/tanglit_test_config";
        with_var(CONFIG_DIR_ENVVAR, Some(random_dir), || {
            let config = LanguageConfig::load_for_lang("rust").unwrap();
            assert_eq!(config.extension, Some("rs".to_string()));
            assert_eq!(config.placeholder_regex, Some("#<([A-Z]+)>#".to_string()));
            assert_eq!(
                config.template.unwrap(),
                include_str!("../../resources/config/executors/rust/template")
            );
            assert_eq!(
                config.execution_script.unwrap(),
                include_str!("../../resources/config/executors/rust/execute.sh")
            );
        });
    }
    #[test]
    fn test_load_default_config_python() {
        // Use a random directory to ensure it doesn't exist
        let random_dir = "/tmp/tanglit_test_config";
        with_var(CONFIG_DIR_ENVVAR, Some(random_dir), || {
            let config = LanguageConfig::load_for_lang("python").unwrap();
            assert_eq!(config.extension, Some("rs".to_string()));
            assert_eq!(config.placeholder_regex, Some("#<([A-Z]+)>#".to_string()));
            assert_eq!(
                config.template.unwrap(),
                include_str!("../../resources/config/executors/python/template")
            );
            assert_eq!(
                config.execution_script.unwrap(),
                include_str!("../../resources/config/executors/python/execute.sh")
            );
        });
    }
    #[test]
    fn test_load_default_config_c() {
        // Use a random directory to ensure it doesn't exist
        let random_dir = "/tmp/tanglit_test_config";
        with_var(CONFIG_DIR_ENVVAR, Some(random_dir), || {
            let config = LanguageConfig::load_for_lang("c").unwrap();
            assert_eq!(config.extension, Some("rs".to_string()));
            assert_eq!(config.placeholder_regex, Some("#<([A-Z]+)>#".to_string()));
            assert_eq!(
                config.template.unwrap(),
                include_str!("../../resources/config/executors/c/template")
            );
            assert_eq!(
                config.execution_script.unwrap(),
                include_str!("../../resources/config/executors/c/execute.sh")
            );
        });
    }
}
