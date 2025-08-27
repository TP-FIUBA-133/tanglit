use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Clone)]
pub struct LanguageConfig {
    pub extension: Option<String>,
    #[serde(skip)]
    pub config_dir: PathBuf,
    pub placeholder_regex: Option<String>, // If empty, we'll use the default
}

const TEMPLATE_FILENAME: &str = "template";
const EXECUTE_SCRIPT_FILENAME: &str = "execute";

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

impl LanguageConfig {
    pub fn get_template_path(&self) -> Option<PathBuf> {
        // Look for a file named TEMPLATE_FILENAME (with or without extension) in the config directory
        find_file_in_dir(&self.config_dir, TEMPLATE_FILENAME)
    }
    pub fn get_execution_script_path(&self) -> Option<PathBuf> {
        // Look for a file named EXECUTE_SCRIPT_FILENAME (with or without extension) in the config directory
        find_file_in_dir(&self.config_dir, EXECUTE_SCRIPT_FILENAME)
    }
}

impl LanguageConfig {
    pub fn load_from_str(toml_str: &str) -> Result<LanguageConfig, Box<dyn std::error::Error>> {
        let config: LanguageConfig = toml::from_str(toml_str)?;
        Ok(config)
    }
    pub fn load_from_file(path: &PathBuf) -> Result<LanguageConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut lang_config = LanguageConfig::load_from_str(&content)?;
        lang_config.config_dir = path.parent().unwrap().to_path_buf();
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
