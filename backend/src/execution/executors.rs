use crate::configuration::get_config_dir;
use crate::doc::Language;
use crate::errors::ExecutionError;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

/// Executes a file based on its language
pub fn execute_file(
    language: &Language,
    source_file_path: PathBuf,
) -> Result<Output, ExecutionError> {
    let config_dir = get_config_dir();
    let lang_str = language.to_string().to_lowercase();
    let executor_dir = config_dir.join("executors").join(lang_str);

    // Look for a file named "execute" (with or without extension) in the directory
    let execution_script_path = std::fs::read_dir(&executor_dir).ok().and_then(|entries| {
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|path| {
                path.file_stem()
                    .map(|stem| stem == "execute")
                    .unwrap_or(false)
            })
    });

    // If an execution script exists, run it
    if let Some(script_path) = execution_script_path {
        return Command::new(script_path)
            .arg(source_file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| ExecutionError::InternalError(e.to_string()));
    }

    // Otherwise, we indicate that we can't execute this language
    // TODO: remove variants from Language struct
    match language {
        Language::C => Err(ExecutionError::UnsupportedLanguage("C".to_string())),
        Language::Python => Err(ExecutionError::UnsupportedLanguage("Python".to_string())),
        Language::Rust => Err(ExecutionError::UnsupportedLanguage("Rust".to_string())),
        Language::Unknown(lang) => Err(ExecutionError::UnsupportedLanguage(lang.clone())),
    }
}
