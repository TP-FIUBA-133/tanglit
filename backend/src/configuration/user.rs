use std::{fs, io, path::PathBuf, sync::OnceLock};

use directories;

const DEFAULT_PROJECT_NAME: &str = "tanglit";
static DEFAULT_CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();
static DEFAULT_TEMP_DIR: OnceLock<PathBuf> = OnceLock::new();
const TEMP_DIR_ENVVAR: &str = "TANGLIT_TEMP_DIR";
const CONFIG_DIR_ENVVAR: &str = "TANGLIT_CONFIG_DIR";

/// Returns the default configuration directory path, either from the environment variable
/// or from the default project directory structure.
/// If the environment variable is not set, it defaults to a platform-specific config directory
/// under the user's home directory.
/// If it cannot be determined, it falls back to the current directory in a `.tanglit` subdirectory.
pub fn get_default_config_dir() -> &'static PathBuf {
    DEFAULT_CONFIG_DIR.get_or_init(|| {
        std::env::var(CONFIG_DIR_ENVVAR)
            .map(|v| PathBuf::from(v).join(DEFAULT_PROJECT_NAME))
            .unwrap_or_else(|_| {
                directories::ProjectDirs::from("", "", DEFAULT_PROJECT_NAME)
                    .map(|dirs| dirs.config_dir().to_path_buf())
                    .unwrap_or_else(|| {
                        PathBuf::from(".").join(format!(".{}", DEFAULT_PROJECT_NAME))
                    })
            })
    })
}
/// Returns the default temporary directory path used for any intermediate files generated during execution.
/// It uses the `TANGLIT_TEMP_DIR` environment variable if set, otherwise it
/// defaults to the system's temporary directory with a subdirectory named after the project.
/// It also follows the implementation of rust's `std::env::temp_dir()`, meaning that `TMPDIR` is also
/// a valid environment variable to use in unix systems
pub fn get_default_temp_dir() -> &'static PathBuf {
    DEFAULT_TEMP_DIR.get_or_init(|| {
        std::env::var(TEMP_DIR_ENVVAR)
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir().join(DEFAULT_PROJECT_NAME))
    })
}

pub fn create_configuration_dirs() -> io::Result<()> {
    let temp_dir = get_default_temp_dir();
    fs::create_dir_all(temp_dir)?;
    let config_dir = get_default_config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_config_dir() {
        use temp_env::with_var;
        let random_dir = "/tmp/tanglit_test_config";
        with_var(super::CONFIG_DIR_ENVVAR, Some(random_dir), || {
            let config_dir = super::get_default_config_dir();
            assert_eq!(
                config_dir.as_path(),
                PathBuf::from(random_dir)
                    .join(DEFAULT_PROJECT_NAME)
                    .as_path()
            );
        });
    }
}
