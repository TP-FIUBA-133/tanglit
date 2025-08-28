mod template_engine;
mod util;
mod wrappers;

use crate::configuration::language_config::LanguageConfig;
use crate::configuration::{get_config_for_lang, get_temp_dir};
use crate::doc::TangleError;
use crate::doc::TanglitDoc;
use crate::errors::ExecutionError;
use crate::execution::util::find_file_in_dir;
use executors::execute_file;
use log::debug;
use std::path::Path;
use std::process::{Command, Output, Stdio};
pub use wrappers::{make_executable_code, write_file};

const EXECUTE_SCRIPT_FILENAME: &str = "execute";

/// Executes a code block by tangling it and adding necessary wrappers to make it executable.
/// Prints both the resulting stdout and sterr from the execution and returns the stdout as a String.
/// # Arguments
/// * `doc` - A reference to the TanglitDoc containing the code blocks
/// * `target_block` - The name of the target code block to execute
/// # Returns
/// * Result containing the stdout of the execution or an error if something goes wrong
pub fn execute(doc: &TanglitDoc, target_block: &str) -> Result<Output, ExecutionError> {
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

    let lang_config = get_config_for_lang(lang)?;

    // create the executable source code
    let output = make_executable_code(block, &blocks, &lang_config)?;

    // Write the output to a file
    let tmp_dir = get_temp_dir();

    let block_file_path = write_file(
        output,
        tmp_dir,
        target_block,
        lang_config.extension.as_deref(),
    )
    .map_err(|e| ExecutionError::WriteError(e.to_string()))?;

    debug!("Wrote tangled code to file: {}", block_file_path.display());

    execute_block(&block_file_path, &lang_config)
}

pub fn execute_block(
    block_file_path: &Path,
    lang_config: &LanguageConfig,
) -> Result<Output, ExecutionError> {
    let execution_script_path = find_file_in_dir(&lang_config.config_dir, EXECUTE_SCRIPT_FILENAME)
        .ok_or(ExecutionError::InternalError(
            "Execution script not found".to_string(),
        ))?;

    Command::new(execution_script_path)
        .arg(block_file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ExecutionError::InternalError(e.to_string()))
}
