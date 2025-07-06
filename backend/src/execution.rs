use crate::errors::ExecutionError;
use crate::parser::code_block::Language;
use crate::parser::parse_blocks_from_file;
use crate::tangle::tangle_block;
use std::process::{Command, Output, Stdio};
use std::{env, fs};
use std::{fmt, io};
use std::{fs::write, path::PathBuf};

pub struct ExecutionOutput {
    pub stdout: String,
    pub stderr: String,
}

impl fmt::Display for ExecutionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.stderr.is_empty() {
            write!(f, "{}", self.stdout)
        } else {
            write!(f, "STDERR:\n{}", self.stderr)
        }
    }
}

pub fn execute(
    input_file_path: &str,
    target_block: &str,
) -> Result<ExecutionOutput, ExecutionError> {
    // Parse blocks from the input file
    let blocks = parse_blocks_from_file(input_file_path)?;

    // Tangle blocks
    let (output, lang) = tangle_block(target_block, blocks, true)?;

    // Write the output to a file
    let block_file_path = write_file(output, target_block, &lang)
        .map_err(|e| ExecutionError::WriteError(e.to_string()))?;

    let handles = match lang {
        crate::parser::code_block::Language::C => crate::execution::execute_c_file(block_file_path),
        crate::parser::code_block::Language::Python => {
            crate::execution::execute_python_file(block_file_path)
        }
        other => {
            return Err(ExecutionError::UnsupportedLanguage(other.to_string()));
        }
    };

    let output = ExecutionOutput {
        stdout: String::from_utf8_lossy(&handles.stdout).to_string(),
        stderr: String::from_utf8_lossy(&handles.stderr).to_string(),
    };

    Ok(output)
}

/// Writes the contents to a file to a `tmp` directory in the current directory.
fn write_file(contents: String, name: &str, lang: &Language) -> io::Result<std::path::PathBuf> {
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

/// Executes a C file by compiling it and then running the resulting binary.
/// This function assumes that the `gcc` compiler is available in the system's PATH.
fn execute_c_file(source_file_path: PathBuf) -> Output {
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

/// Runs a binary file and captures its output.
fn run_binary(binary_path: &str) -> Output {
    Command::new(binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute binary")
}

/// Executes a Python file by running it with the Python interpreter.
/// This function assumes that the Python interpreter is available in the system's PATH.
fn execute_python_file(source_file_path: PathBuf) -> Output {
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

//TODO: tests (probably require mocking or to be integration type tests)
