pub mod language_config;
mod user;

use std::io;
use user::create_configuration_dirs;

use crate::configuration::language_config::LanguageConfig;
use crate::errors::ExecutionError;
pub use user::{get_config_dir, get_temp_dir};

pub fn get_config_for_lang(lang: &str) -> Result<LanguageConfig, ExecutionError> {
    let config_dir = get_config_dir();
    let config_path = &config_dir.join("executors").join(lang).join("config.toml");
    LanguageConfig::load_from_file(config_path).map_err(|e| {
        ExecutionError::UnsupportedLanguage(format!(
            "Unable to load config for language {} at {}: {}",
            lang,
            config_path.display(),
            e
        ))
    })
}

/// Creates and initializes the default configuration directories.
pub fn init_configuration() -> io::Result<()> {
    create_configuration_dirs()?;
    // any additional initialization logic should go here
    Ok(())
}
