use crate::configuration::get_default_config_dir;
use crate::doc::Language;
use crate::errors::ExecutionError;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

/// Executes a file based on its language
pub fn execute_file(
    language: &Language,
    source_file_path: PathBuf,
) -> Result<Output, ExecutionError> {
    let config_dir = get_default_config_dir();
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

    // Fall back to built-in language executors
    let output = match language {
        Language::C => execute_c_file(source_file_path),
        Language::Python => execute_python_file(source_file_path),
        Language::Rust => execute_rust_file(source_file_path),
        Language::Unknown(lang) => {
            return Err(ExecutionError::UnsupportedLanguage(lang.clone()));
        }
    };
    Ok(output)
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

/// Executes a Rust file by compiling it and then running the resulting binary.
/// This function assumes that the `rustc` compiler is available in the system's PATH.
fn execute_rust_file(source_file_path: PathBuf) -> Output {
    let output_binary = source_file_path.with_extension("");

    // Compile the Rust program
    let compile_status = Command::new("rustc")
        .arg(&source_file_path)
        .arg("-o")
        .arg(&output_binary)
        .status()
        .expect("Failed to start rustc");

    if !compile_status.success() {
        eprintln!("Failed to compile Rust program.");
        std::process::exit(1);
    }

    let binary = output_binary.to_str().unwrap();

    // Step 2: Run the compiled Rust binary and capture output
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
