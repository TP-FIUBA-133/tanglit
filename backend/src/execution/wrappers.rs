use crate::configuration::language_config::LanguageConfig;
use crate::doc::CodeBlock;
use crate::doc::CodeBlocks;
use crate::doc::DocError;
use crate::errors::ExecutionError;
use crate::execution::render_engine::render;
use regex::Regex;
use std::collections::HashMap;
use std::fs::write;
use std::io;
use std::path::{Path, PathBuf};

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

pub fn write_code_to_file(block: &CodeBlock, code: String, dir: String) -> io::Result<PathBuf> {
    let lang = block.language.as_deref();
    let extension = lang
        .and_then(|l| LanguageConfig::load_for_lang(l).ok())
        .and_then(|cfg| cfg.extension);

    let file_name = block.export.clone().unwrap_or(block.tag.clone());

    write_file(code, &PathBuf::from(dir), &file_name, extension.as_deref())
}

/// Loads and applies a template wrapper for the given language
fn add_wrapper(
    lang_config: &LanguageConfig,
    code: &str,
    imports: &str,
) -> Result<String, ExecutionError> {
    // TODO: This should be done in config
    let Some(pattern) = &lang_config.placeholder_regex else {
        return Err(ExecutionError::InternalError("Unable to get regex.".into()));
    };

    let regex = Regex::new(pattern).map_err(|e| ExecutionError::InternalError(e.to_string()))?;

    let template = lang_config
        .template
        .clone()
        .ok_or(ExecutionError::TemplateNotFound)?;

    render(template, &regex, imports, code)
        .map_err(|e| ExecutionError::InternalError(format!("Template render error: {}", e)))
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

fn replace_args(code: &str, args: &HashMap<String, String>) -> String {
    let pattern = args.keys()
        .map(|k| regex::escape(k))
        .collect::<Vec<_>>()
        .join("|");
    let re = Regex::new(&format!(r"\b({})\b", pattern)).unwrap();

    re.replace_all(code, |caps: &regex::Captures| {
        let key = &caps[1];
        args.get(key).cloned().unwrap_or_else(|| key.to_string())
    })
    .into_owned()
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

    let code_with_args = replace_args(&code, &code_block.args);

    add_wrapper(lang_config, &code_with_args, &imports_output)
}

#[cfg(test)]
mod tests {
    use temp_env::with_var;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_apply_wrapper() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Option::from("c".to_string()),
            "@[x]\nprintf(\"Hello, world!: %d\",x);".to_string(),
            "main".to_string(),
            vec!["io".to_string()],
            None,
            HashMap::new(),
            0,
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
                None,
                HashMap::new(),
                0,
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
                None,
                HashMap::new(),
                0,
                0,
            ),
        );
        let config_path = format!(
            "{}/resources/config",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        println!("Using config path: {}", config_path);
        with_var("TANGLIT_CONFIG_DIR", Some(config_path), || {
            let lang_config = LanguageConfig::load_for_lang("c").unwrap();
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

    #[test]
    fn test_replace_args() {
        let mut args = HashMap::new();
        args.insert("A".to_string(), "1".to_string());
        args.insert("B".to_string(), "2".to_string());

        let code = "print(A + B);";
        let expected = "print(1 + 2);";

        assert_eq!(replace_args(code, &args), expected);
    }

    #[test]
    fn test_replace_args_with_quotes() {
        let mut args = HashMap::new();
        args.insert("X".to_string(), "\"hello\"".to_string());
        args.insert("Y".to_string(), "3.14".to_string());

        let code = "let s = X; let pi = Y;";
        let expected = "let s = \"hello\"; let pi = 3.14;";

        assert_eq!(replace_args(code, &args), expected);
    }

    #[test]
    fn test_replace_args_partial_names() {
        let mut args = HashMap::new();
        args.insert("A".to_string(), "10".to_string());

        let code = "MAX = A + 5;";
        let expected = "MAX = 10 + 5;"; // solo A se reemplaza, no MAX

        assert_eq!(replace_args(code, &args), expected);
    }

    #[test]
    fn test_replace_args_missing_param_dont_replace() {
        let args = HashMap::new(); // ningún parámetro

        let code = "print(B);";
        let expected = "print(B);"; // B no existe en args, se deja igual

        assert_eq!(replace_args(code, &args), expected);
    }

    #[test]
    fn test_replace_args_multiple_occurrences() {
        let mut args = HashMap::new();
        args.insert("V".to_string(), "42".to_string());

        let code = "V + V + V;";
        let expected = "42 + 42 + 42;";

        assert_eq!(replace_args(code, &args), expected);
    }
}
