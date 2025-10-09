mod render_engine;
mod wrappers;

use crate::configuration::get_temp_dir;
use crate::configuration::language_config::LanguageConfig;
use crate::doc::TangleError;
use crate::doc::TanglitDoc;
use crate::errors::ExecutionError;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
pub use wrappers::{make_executable_code, write_code_to_file, write_file};

/// Executes a code block by tangling it and adding necessary wrappers to make it executable.
/// Prints both the resulting stdout and sterr from the execution and returns the stdout as a String.
/// # Arguments
/// * `doc` - A reference to the TanglitDoc containing the code blocks
/// * `target_block` - The name of the target code block to execute
/// # Returns
/// * Result containing the stdout of the execution or an error if something goes wrong

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOutput {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn execute(doc: &TanglitDoc, target_block: &str) -> Result<ExecutionOutput, ExecutionError> {
    let blocks = doc.get_code_blocks()?;

    let block = blocks
        .get_block(target_block)
        .ok_or(TangleError::BlockNotFound(target_block.to_string()))?;

    let lang = block
        .language
        .as_deref()
        .ok_or(ExecutionError::UnsupportedLanguage(
            "No language specified".to_string(),
        ))?;

    let lang_config = LanguageConfig::load_for_lang(lang)?;

    // create the executable source code
    let output = make_executable_code(block, &blocks, &lang_config)?;

    // Write the output to a file
    let tmp_dir = &get_temp_dir();

    let block_file_path = write_file(
        output,
        tmp_dir,
        target_block,
        lang_config.extension.as_deref(),
    )
    .map_err(|e| ExecutionError::WriteError(e.to_string()))?;

    debug!("Wrote tangled code to file: {}", block_file_path.display());

    let execution_script_path = lang_config
        .execution_script_path
        .as_ref()
        .ok_or(ExecutionError::ExecutionScriptNotFound)?;

    execute_block(&block_file_path, execution_script_path)
}

pub fn execute_block(
    block_file_path: &Path,
    execution_script_path: &PathBuf,
) -> Result<ExecutionOutput, ExecutionError> {
    let output = Command::new(execution_script_path)
        .arg(block_file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ExecutionError::InternalError(e.to_string()))?;

    Ok(ExecutionOutput {
        status: output.status.code(),
        stdout: String::from_utf8(output.stdout).expect("Error reading stdout"),
        stderr: String::from_utf8(output.stderr).expect("Error reading stderr"),
    })
}
