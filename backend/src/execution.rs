use crate::doc::{CodeBlock, CodeBlocksDoc, DocError, Language, TangleError, TanglitDoc};
use crate::errors::ExecutionError;
use std::io;
use std::process::{Command, Output, Stdio};
use std::{env, fs};
use std::{fs::write, path::PathBuf};

const DEFAULT_INDENT_SIZE: usize = 4;
const DEFAULT_INDENT_CHARACTER: char = ' ';

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

/// Sets indentation for each line in the code.
///
/// # Arguments
/// * `code` - A mutable reference to the string containing the code to indent
/// * `indent_size` - Optional number of indentation characters to use per level (defaults to 4)
/// * `indent_character` - Optional character to use for indentation (defaults to space)
fn set_indentation(code: &mut String, indent_size: Option<usize>, indent_character: Option<char>) {
    let indent_str = indent_character
        .unwrap_or(DEFAULT_INDENT_CHARACTER)
        .to_string()
        .repeat(indent_size.unwrap_or(DEFAULT_INDENT_SIZE));

    *code = std::mem::take(code)
        .lines()
        .map(|line| format!("{}{}\n", indent_str, line))
        .collect::<String>();
}

fn add_c_wrapper(code: &str, tangle: &mut String) {
    let mut code = code.to_string();
    code.push('\n');
    code.push_str("return 0;");
    set_indentation(&mut code, None, None);

    tangle.push_str("int main() {\n");
    tangle.push_str(&code);
    // tangle.push('\n');
    tangle.push_str("}\n");
}

fn add_python_wrapper(code: &str, tangle: &mut String) {
    // For Python, we don't need to add a wrapper
    // but we can still set indentation
    let mut code = code.to_string();
    set_indentation(&mut code, None, None);
    tangle.push_str("if __name__ == '__main__':\n");
    tangle.push_str(&code);
}

// fn add_rust_wrapper(code: &str, tangle: &mut String) {
//     let mut code = code.clone();
//     set_indentation(&mut code, None, None);
//     tangle.push_str("fn main() {\n");
//     tangle.push_str(&code);
//     tangle.push('\n');
//     tangle.push_str("}\n");
// }

// fn execute_rust_file(source_file_path: PathBuf) -> Output {
//     todo!()
// }

/// Tangles code in a given codeblock, wraps it in a language-specific wrapper
/// and adds any imported blocks. Indentation is applied.
fn make_executable_code(
    code_block: &CodeBlock,
    blocks: &CodeBlocksDoc,
) -> Result<String, ExecutionError> {
    // Tangle blocks
    let mut output = String::new();

    for import in &code_block.imports {
        if let Some(import_block) = blocks.get_block(import) {
            // Tangle the imported block
            let import_output = blocks
                .tangle_codeblock(import_block)
                .map_err(DocError::from)?;
            // Append the import output to the main output
            output.push_str(&import_output);
            output.push('\n');
        } else {
            return Err(ExecutionError::InternalError(format!(
                "Import '{}' not found in blocks",
                import
            )));
        }
    }

    let code = blocks
        .tangle_codeblock(code_block)
        .map_err(|e| ExecutionError::from(DocError::from(e)))?;

    match &code_block.language {
        Language::C => add_c_wrapper(&code, &mut output),
        Language::Python => {
            add_python_wrapper(&code, &mut output);
        }
        Language::Unknown(lang) => return Err(ExecutionError::UnsupportedLanguage(lang.clone())),
        Language::Rust => return Err(ExecutionError::UnsupportedLanguage("Rust".into())),
    }

    Ok(output)
}

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

    let handles = match &block.language {
        Language::C => crate::execution::execute_c_file(block_file_path),
        Language::Python => crate::execution::execute_python_file(block_file_path),
        Language::Rust => {
            return Err(ExecutionError::UnsupportedLanguage("Rust".into()));
        }
        Language::Unknown(lang) => {
            return Err(ExecutionError::UnsupportedLanguage(lang.clone()));
        }
    };

    Ok(handles)
}

//TODO: execution tests (probably require mocking or to be integration type tests)

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_make_executable_code_c() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Language::C,
            "@[x]\nprintf(\"Hello, world!: %d\",x);".to_string(),
            "main".to_string(),
            vec!["io".to_string()],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        blocks.insert(
            "io".to_string(),
            CodeBlock::new(
                Language::C,
                "#include <stdio.h>".to_string(),
                "io".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "x".to_string(),
            CodeBlock::new(
                Language::C,
                "int x;\nx = 42;".to_string(),
                "x".to_string(),
                vec![],
                0,
            ),
        );
        let tangle = make_executable_code(&main, &CodeBlocksDoc::from_codeblocks(blocks)).unwrap();
        assert_eq!(
            tangle,
            "#include <stdio.h>\nint main() {\n    int x;\n    x = 42;\n    printf(\"Hello, world!: %d\",x);\n    return 0;\n}\n"
                .to_string()
        );
    }

    #[test]
    fn test_make_executable_code_python() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Language::Python,
            "@[x]\nprint(f\"Hello, world!: {x}\")".to_string(),
            "main".to_string(),
            vec!["io".to_string()],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        blocks.insert(
            "io".to_string(),
            CodeBlock::new(
                Language::Python,
                "import sys".to_string(),
                "io".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "x".to_string(),
            CodeBlock::new(
                Language::Python,
                "x = 42".to_string(),
                "x".to_string(),
                vec![],
                0,
            ),
        );
        let tangle = make_executable_code(&main, &CodeBlocksDoc::from_codeblocks(blocks)).unwrap();
        assert_eq!(
            tangle,
            "import sys\nif __name__ == '__main__':\n    x = 42\n    print(f\"Hello, world!: {x}\")\n".to_string()
        );
    }
}
