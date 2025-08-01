mod executors;
mod template_engine;
mod wrappers;

use crate::doc::CodeBlocksDoc;
use crate::doc::DocError;
use crate::doc::TangleError;
use crate::doc::TanglitDoc;
use crate::doc::{CodeBlock, Language};
use crate::errors::ExecutionError;
use std::process::Output;

use executors::execute_file;
pub use wrappers::{make_executable_code, write_file};

/// Executes a code block by tangling it and adding necessary wrappers to make it executable.
/// Prints both the resulting stdout and sterr from the execution and returns the stdout as a String.
/// # Arguments
/// * `doc` - A reference to the TanglitDoc containing the code blocks
/// * `target_block` - The name of the target code block to execute
/// # Returns
/// * Result containing the stdout of the execution or an error if something goes wrong
pub fn execute(doc: &TanglitDoc, target_block: &str) -> Result<Output, ExecutionError> {
    let blocks = doc.tangle()?;

    let block =
        blocks
            .get_block(target_block)
            .ok_or(DocError::from(TangleError::BlockNotFound(
                target_block.to_string(),
            )))?;

    // create the executable source code
    let output = make_executable_code(block, &blocks)?;

    // Write the output to a file
    let block_file_path = write_file(output, target_block, &block.language)
        .map_err(|e| ExecutionError::WriteError(e.to_string()))?;

    // Execute the file based on language
    execute_file(&block.language, block_file_path)
}
