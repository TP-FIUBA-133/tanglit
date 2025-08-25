use crate::configuration::get_config_dir;
use crate::configuration::get_temp_dir;
use crate::doc::CodeBlock;
use crate::doc::CodeBlocks;
use crate::doc::DocError;
use crate::errors::ExecutionError;
use crate::execution::template_engine::Template;
use std::fs::write;
use std::io;

/// Writes the contents to a file to a `tmp` directory in the current directory.
pub fn write_file(contents: String, name: &str, lang: &str) -> io::Result<std::path::PathBuf> {
    let tmp_dir = get_temp_dir();
    // Create the file path using the target block name
    let ext = match lang {
        "c" => "c",
        "python" => "py",
        "rust" => "rs",
        _ => "",
    };
    let source_file = tmp_dir.join(format!("{}.{}", name, ext));

    // Write the tangled output to the file
    write(&source_file, contents)?;
    io::Result::Ok(source_file)
}

/// Loads and applies a template wrapper for the given language
fn add_wrapper(
    language: &Option<String>,
    code: &str,
    imports: &str,
) -> Result<String, ExecutionError> {
    // Try to load language-specific template, fall back to hardcoded wrappers
    let config_dir = get_config_dir();
    let lang_str = language
        .as_deref()
        .ok_or(ExecutionError::UnsupportedLanguage(
            "No language specified".to_string(),
        ))?
        .to_lowercase();
    let template_path = config_dir
        .join("executors")
        .join(lang_str.as_str())
        .join("wrapper.template");

    // Try to load template file or error out if not found
    Template::load_from_file(template_path)
        .map_err(|e| {
            ExecutionError::UnsupportedLanguage(format!(
                "Template file for {} language not found: {}",
                lang_str, e
            ))
        })
        .and_then(|t| {
            t.render(imports, code)
                .map_err(|e| ExecutionError::InternalError(format!("Template render error: {}", e)))
        })
}

/// Tangles code in a given codeblock, wraps it in a language-specific wrapper
/// and adds any imported blocks.
pub fn make_executable_code(
    code_block: &CodeBlock,
    blocks: &CodeBlocks,
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
                "Import '{
}' not found in blocks",
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
    use temp_env::with_var;

    use super::*;
    use std::collections::HashMap;

    #[test]
    #[ignore = "the implementation of get_config_dir must be changed before this can work"]
    fn test_apply_wrapper() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Option::from("c".to_string()),
            " @ [x]\nprintf(\"Hello, world!: %d\",x);".to_string(),
            "main".to_string(),
            vec!["io".to_string()],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        blocks.insert(
            "io".to_string(),
            CodeBlock::new(
                Option::from("c".to_string()),
                "#include <stdio.h>".to_string(),
                "io".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "x".to_string(),
            CodeBlock::new(
                Option::from("c".to_string()),
                "int x;\nx = 42;".to_string(),
                "x".to_string(),
                vec![],
                0,
            ),
        );
        let config_path = format!(
            "{}/resources/config",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        println!("Using config path: {}", config_path);
        with_var("TANGLIT_CONFIG_DIR", Some(config_path), || {
            let tangle = make_executable_code(&main, &CodeBlocks::from_codeblocks(blocks)).unwrap();
            assert_eq ! (
    tangle,
    "#include <stdio.h>\n\n\nint main(){\n    int x;\n    x = 42;\n    printf(\"Hello, world!: %d\",x);\n    return 0;\n}\n"
    .to_string()
    );
        });
    }
}
