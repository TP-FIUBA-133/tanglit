use crate::configuration::language_config::LanguageConfig;
use crate::doc::CodeBlock;
use crate::doc::CodeBlocks;
use crate::doc::DocError;
use crate::errors::ExecutionError;
use crate::execution::template_engine::Template;
use crate::execution::util::find_file_in_dir;
use std::fs::write;
use std::io;
use std::path::Path;

const TEMPLATE_FILENAME: &str = "template";

pub fn full_filename(name: &str, ext: Option<&str>) -> String {
    ext.as_ref()
        .map_or(name.to_string(), |ext| format!("{}.{}", name, ext))
}

/// Writes the contents to a file to a `tmp` directory in the current directory.
pub fn write_file(
    contents: String,
    dir: &Path,
    name: &str,
    ext: Option<&str>,
) -> io::Result<std::path::PathBuf> {
    let dst_filename = full_filename(name, ext);
    let dst_path = dir.join(dst_filename);
    // Write the tangled output to the file
    write(&dst_path, contents)?;
    io::Result::Ok(dst_path)
}

/// Loads and applies a template wrapper for the given language
fn add_wrapper(
    template_path: &Path,
    code: &str,
    imports: &str,
    placeholder_regex: Option<&str>,
) -> Result<String, ExecutionError> {
    // Try to load template file or error out if not found
    Template::load_from_file(template_path, placeholder_regex)
        .map_err(|e| {
            ExecutionError::UnsupportedLanguage(format!(
                "Unable to load template at path {}: {}",
                template_path.display(),
                e
            ))
        })
        .and_then(|t| {
            t.render(imports, code)
                .map_err(|e| ExecutionError::InternalError(format!("Template render error: {}", e)))
        })
}

pub fn tangle_imports(
    code_block: &CodeBlock,
    blocks: &CodeBlocks,
) -> Result<String, ExecutionError> {
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
    Ok(imports_output)
}

/// Tangles code in a given codeblock, wraps it in a language-specific wrapper
/// and adds any imported blocks.
pub fn make_executable_code(
    code_block: &CodeBlock,
    blocks: &CodeBlocks,
    lang_config: &LanguageConfig,
) -> Result<String, ExecutionError> {
    // Tangle blocks
    let imports_output = tangle_imports(code_block, blocks)?;

    let code = blocks
        .tangle_codeblock(code_block)
        .map_err(|e| ExecutionError::from(DocError::from(e)))?;

    let template_path = find_file_in_dir(&lang_config.config_dir, TEMPLATE_FILENAME).ok_or(
        ExecutionError::InternalError("No template file found in language config dir".to_string()),
    )?;

    add_wrapper(
        &template_path,
        &code,
        &imports_output,
        lang_config.placeholder_regex.as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use temp_env::with_var;

    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    #[ignore = "the implementation of get_config_dir must be changed before this can work"]
    fn test_apply_wrapper() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Option::from("c".to_string()),
            "@[x]\nprintf(\"Hello, world!: %d\",x);".to_string(),
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
        let lang_config = LanguageConfig::load_from_file(&PathBuf::from(&config_path)).unwrap();
        with_var("TANGLIT_CONFIG_DIR", Some(config_path), || {
            let tangle =
                make_executable_code(&main, &CodeBlocks::from_codeblocks(blocks), &lang_config)
                    .unwrap();
            assert_eq!(
                tangle,
                "#include <stdio.h>\n\n\nint main(){\n    int x;\n    x = 42;\n    printf(\"Hello, world!: %d\",x);\n    return 0;\n}\n"
                .to_string()
            );
        });
    }
}
