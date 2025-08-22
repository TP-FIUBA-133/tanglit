use crate::configuration::get_default_config_dir;
use crate::configuration::get_default_temp_dir;
use crate::doc::CodeBlocksDoc;
use crate::doc::DocError;
use crate::doc::{CodeBlock, Language};
use crate::errors::ExecutionError;
use crate::execution::template_engine::{Template, set_indentation};
use std::collections::HashMap;
use std::fs::write;
use std::io;

/// Writes the contents to a file to a `tmp` directory in the current directory.
pub fn write_file(contents: String, name: &str, lang: &Language) -> io::Result<std::path::PathBuf> {
    let tmp_dir = get_default_temp_dir();
    // Create the file path using the target block name
    let ext = match lang {
        Language::C => "c",
        Language::Python => "py",
        Language::Rust => "rs",
        _ => "",
    };
    let source_file = tmp_dir.join(format!("{}.{}", name, ext));

    // Write the tangled output to the file
    write(&source_file, contents)?;
    io::Result::Ok(source_file)
}

fn add_c_wrapper(code: &str, tangle: &mut String) {
    let mut code = code.to_string();
    code.push_str("\nreturn 0;");
    set_indentation(&mut code, Some(4), Some(' '));

    tangle.push_str("int main() {\n    ");
    tangle.push_str(&code);
    tangle.push('\n');
    tangle.push_str("}\n");
}

fn add_python_wrapper(code: &str, tangle: &mut String) {
    // For Python, we don't need to add a wrapper
    // but we can still set indentation
    let mut code = code.to_string();
    set_indentation(&mut code, Some(4), Some(' '));
    tangle.push_str("if __name__ == '__main__':\n    ");
    tangle.push_str(&code);
}

fn add_rust_wrapper(code: &str, tangle: &mut String) {
    let mut code = code.to_string();
    set_indentation(&mut code, Some(4), Some(' '));
    tangle.push_str("fn main() {\n    ");
    tangle.push_str(&code);
    tangle.push_str("}\n");
}

/// Loads and applies a template wrapper for the given language
fn add_wrapper(language: &Language, code: &str, imports: &str) -> Result<String, ExecutionError> {
    // Try to load language-specific template, fall back to hardcoded wrappers
    let config_dir = get_default_config_dir();
    let lang_str = language.to_string().to_lowercase();
    let template_path = config_dir
        .join("executors")
        .join(lang_str)
        .join("wrapper.template");

    // Try to load template file, fall back to hardcoded wrappers if not found
    match Template::load_from_file(template_path) {
        Ok(template) => {
            let mut replacements = HashMap::new();
            replacements.insert("IMPORTS".to_string(), imports.to_string());
            replacements.insert("BODY".to_string(), code.to_string());

            template
                .render(&replacements)
                .map_err(|e| ExecutionError::InternalError(format!("Template render error: {}", e)))
        }
        Err(_) => {
            // Fall back to hardcoded wrappers
            let mut output = imports.to_string();

            match language {
                Language::C => add_c_wrapper(code, &mut output),
                Language::Python => {
                    add_python_wrapper(code, &mut output);
                }
                Language::Rust => add_rust_wrapper(code, &mut output),
                Language::Unknown(lang) => {
                    return Err(ExecutionError::UnsupportedLanguage(lang.clone()));
                }
            }

            Ok(output)
        }
    }
}

/// Tangles code in a given codeblock, wraps it in a language-specific wrapper
/// and adds any imported blocks.
pub fn make_executable_code(
    code_block: &CodeBlock,
    blocks: &CodeBlocksDoc,
) -> Result<String, ExecutionError> {
    // Tangle blocks
    let mut imports_output = String::new();

    for import in &code_block.imports {
        if let Some(import_block) = blocks.get_block(import) {
            // Tangle the imported block
            let import_output = blocks
                .tangle_codeblock(import_block)
                .map_err(|e| ExecutionError::from(DocError::from(e)))?;
            // Append the import output to the main output
            imports_output.push_str(&import_output);
            imports_output.push('\n');
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

    // Use template-based wrapper with fallback to hardcoded ones
    add_wrapper(&code_block.language, &code, &imports_output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::Language;

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
            "import sys\nif __name__ == '__main__':\n    x = 42\n    print(f\"Hello, world!: {x}\")".to_string()
        );
    }
}
