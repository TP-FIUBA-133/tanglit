use std::fs;
use std::process::{Command, Output, Stdio};
use std::{env, fs::write, path::PathBuf};

pub fn write_file(contents: String, name: &str, ext: &str) -> std::path::PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let tmp_dir = current_dir.join("tmp");
    if !tmp_dir.exists() {
        fs::create_dir_all(&tmp_dir).expect("Failed to create temp directory");
    }
    // Create the file path using the target block name
    let c_file = tmp_dir.join(format!("{}.{}", name, ext));

    // Write the tangled output to the C file
    write(&c_file, contents).expect("Failed to write C file");
    c_file
}

fn run_binary(binary_path: &str) -> Output {
    Command::new(binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute binary")
}

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

//TODO: tests (probably require mocking or to be integration type tests)
