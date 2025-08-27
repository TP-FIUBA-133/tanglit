use crate::errors::ExecutionError;
use std::path::Path;
use std::process::{Command, Output, Stdio};

/// Executes a file based on its language
pub fn execute_file(script_path: &Path, source_file_path: &Path) -> Result<Output, ExecutionError> {
    Command::new(script_path)
        .arg(source_file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ExecutionError::InternalError(e.to_string()))
}
