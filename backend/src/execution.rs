use crate::parser::code_block::{CodeBlock, Language};
use crate::tangle::tangle_block;
use serde::Serialize;
use std::collections::HashMap;
use std::io;
use std::process::{Command, Output, Stdio};
use std::{env, fs};
use std::{fs::write, path::PathBuf};

/// Writes the contents to a file to a `tmp` directory in the current directory.
pub fn write_file(contents: String, name: &str, lang: &Language) -> io::Result<std::path::PathBuf> {
    let current_dir = env::current_dir()?;
    let tmp_dir = current_dir.join("tmp");
    if !tmp_dir.exists() {
        fs::create_dir_all(&tmp_dir)?;
    }
    // Create the file path using the target block name
    let ext = match lang {
        Language::C => "c",
        Language::Python => "py",
        _ => "txt",
    };
    let source_file = tmp_dir.join(format!("{}.{}", name, ext));

    // Write the tangled output to the C file
    write(&source_file, contents)?;
    io::Result::Ok(source_file)
}

/// Runs a binary file and captures its output.
fn run_binary(binary_path: &str) -> Output {
    Command::new(binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute binary")
}

/// Executes a C file by compiling it and then running the resulting binary.
/// This function assumes that the `gcc` compiler is available in the system's PATH.
pub fn execute_c_file(source_file_path: PathBuf) -> Output {
    let output_binary = source_file_path.with_extension("");

    // Compile the C program
    // TODO: make compilation configurable at runtime
    let compile_status = Command::new("gcc")
        .arg(&source_file_path)
        .arg("-o")
        .arg(&output_binary)
        .status()
        .expect("Failed to start gcc");

    if !compile_status.success() {
        eprintln!("Failed to compile C program.");
        std::process::exit(1);
    }

    let binary = output_binary.to_str().unwrap();

    // Step 2: Run the compiled C binary and capture output
    run_binary(binary)
}

/// Executes a Python file by running it with the Python interpreter.
/// This function assumes that the Python interpreter is available in the system's PATH.
pub fn execute_python_file(source_file_path: PathBuf) -> Output {
    // Run the Python script and capture output
    Command::new("python3")
        .arg(source_file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute Python script")
}

// fn execute_rust_file(source_file_path: PathBuf) -> Output {
//     todo!()
// }

#[derive(Serialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub fn execute(
    blocks: HashMap<String, CodeBlock>,
    target_block: &str,
) -> Result<ExecutionResult, String> {
    // Tangle blocks
    let (output, lang) = tangle_block(target_block, blocks, true)
        .map_err(|e| format!("Error tangling blocks: {e}"))?;

    // Write the output to a file
    let block_file_path = write_file(output, target_block, &lang)
        .map_err(|e| format!("Error writing to file: {e}"))?;

    let handles = match lang {
        crate::parser::code_block::Language::C => crate::execution::execute_c_file(block_file_path),
        crate::parser::code_block::Language::Python => {
            crate::execution::execute_python_file(block_file_path)
        }
        _ => {
            return Err("Unsupported language".to_string());
        }
    };

    let ex_stdout = String::from_utf8_lossy(&handles.stdout);
    let ex_stderr = String::from_utf8_lossy(&handles.stderr);

    Ok(ExecutionResult {
        stdout: ex_stdout.to_string(),
        stderr: ex_stderr.to_string(),
        exit_code: handles.status.code().unwrap_or(1),
    })
}

//TODO: tests (probably require mocking or to be integration type tests)
