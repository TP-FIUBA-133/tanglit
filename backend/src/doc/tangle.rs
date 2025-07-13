use crate::doc::parser::code_block::{CodeBlock, Language};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;

const MACROS_REGEX: &str = r"@\[([a-zA-Z0-9_]+)\]";

#[derive(PartialEq)]
pub enum TangleError {
    BlockNotFound(String),
    InternalError(String),
}

impl fmt::Display for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl fmt::Debug for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

pub fn tangle_block(
    target_block: &str,
    mut blocks: HashMap<String, CodeBlock>,
    add_wrapper: bool,
) -> Result<(String, Language), TangleError> {
    // Get target_code_block
    let mut target_code_block = blocks
        .remove(target_block)
        .ok_or(TangleError::BlockNotFound(target_block.into()))?;

    resolve_macros(&mut target_code_block, &blocks)?;

    // Get imported blocks
    let imported_blocks: Vec<CodeBlock> = target_code_block
        .imports
        .iter()
        .map(|import| {
            blocks
                .get(import)
                .cloned()
                .ok_or(TangleError::BlockNotFound(import.clone()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Tangle imports
    let mut tangle = String::new();
    for block in imported_blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
        // TODO: lines between blocks should be configurable
        tangle.push('\n');
    }

    if add_wrapper {
        if target_code_block.language == Language::C {
            add_main_code_block(&target_code_block, &mut tangle);
        } else {
            tangle.push_str(&target_code_block.code);
        }
    } else {
        tangle.push_str(&target_code_block.code);
    }

    Ok((tangle, target_code_block.language.clone()))
}

/// Resolves macros in a code block by replacing them with the content of the referenced blocks.
fn resolve_macros(
    code_block: &mut CodeBlock,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<(), TangleError> {
    let re = Regex::new(MACROS_REGEX)
        .map_err(|e| TangleError::InternalError(format!("Failed to compile regex: {}", e)))?;

    // Collect all referenced block names
    let referenced_blocks: HashSet<_> = re
        .captures_iter(&code_block.code)
        .map(|caps| caps[1].to_string())
        .collect();

    // Check for missing blocks
    let missing: Vec<_> = referenced_blocks
        .iter()
        .filter(|tag| !blocks.contains_key(*tag))
        .collect();

    if let Some(missing) = missing.first() {
        return Err(TangleError::BlockNotFound(missing.to_string()));
    }

    // Replace imports with block content
    let code_block_with_macros = re.replace_all(&code_block.code, |caps: &regex::Captures| {
        blocks
            .get(&caps[1])
            .unwrap() // It is safe to unwrap here because we checked for missing blocks above
            .code
            .clone()
    });

    code_block.code = code_block_with_macros.into_owned();

    Ok(())
}

// TODO: this should use a template depending on the language
// TODO: handle indentation
pub fn add_main_code_block(code_block: &CodeBlock, tangle: &mut String) {
    tangle.push_str("int main() {\n");
    tangle.push_str(&code_block.code);
    tangle.push('\n');
    tangle.push_str("    return 0;");
    tangle.push_str("\n}\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::parser::code_block::Language;

    #[test]
    fn test_tangle_block_with_imports() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new(
                Language::Python,
                "print('Hello, world!')".to_string(),
                "main".to_string(),
                vec!["helper".to_string()],
                0,
            ),
        );
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new(
                Language::Python,
                "print('Helper function')".to_string(),
                "helper".to_string(),
                vec![],
                0,
            ),
        );

        let tangle = tangle_block("main", blocks, false).unwrap();
        assert_eq!(
            tangle,
            (
                "print('Helper function')\n\nprint('Hello, world!')".to_string(),
                Language::Python
            )
        );
    }

    #[test]
    fn test_tangle_block_with_missing_import() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new(
                Language::Python,
                "print('Hello, world!')".to_string(),
                "main".to_string(),
                vec!["helper".to_string()],
                0,
            ),
        );
        let result = tangle_block("main", blocks, false);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TangleError::BlockNotFound("helper".to_string())
        );
    }

    #[test]
    fn test_resolve_macros() {
        let mut blocks = HashMap::new();
        let mut main = CodeBlock::new(
            Language::Python,
            "@[helper]\nprint('Hello, world!')".to_string(),
            "main".to_string(),
            vec![],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new(
                Language::Python,
                "print('Helper function')".to_string(),
                "helper".to_string(),
                vec![],
                0,
            ),
        );

        let result = resolve_macros(&mut main, &blocks);
        assert!(result.is_ok());
        assert_eq!(
            main.code,
            "print('Helper function')\nprint('Hello, world!')".to_string()
        );
    }

    #[test]
    fn test_resolve_macros_with_missing_block() {
        let mut blocks = HashMap::new();
        let mut main = CodeBlock::new(
            Language::Python,
            "@[helper]\nprint('Hello, world!')".to_string(),
            "main".to_string(),
            vec![],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        let result = resolve_macros(&mut main, &blocks);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TangleError::BlockNotFound("helper".to_string())
        );
    }

    /// Tests that both imports and the wrapper is added to C code when wrapper is requested
    #[test]
    fn test_tangle_block_wrapper() {
        let blocks = HashMap::<String, CodeBlock>::from([
            (
                "imports".to_string(),
                CodeBlock::new(
                    Language::C,
                    "#include <stdio.h>\n".to_string(),
                    "imports".to_string(),
                    vec![],
                    0,
                ),
            ),
            (
                "main_block".to_string(),
                CodeBlock::new(
                    Language::C,
                    "printf(\"Hello, world!\");".to_string(),
                    "main_block".to_string(),
                    vec!["imports".to_string()],
                    0,
                ),
            ),
        ]);
        let tangle = tangle_block("main_block", blocks, true);
        assert!(tangle.is_ok());
        let (tangled_code, _) = tangle.unwrap();
        assert_eq!(
            tangled_code,
            r#"#include <stdio.h>


int main() {
printf("Hello, world!");
    return 0;
}
"#
        );
    }

    /// Tests that imports are added to C code but no wrapper is added when wrapper is not requested
    #[test]
    fn test_tangle_block_no_wrapper() {
        let blocks = HashMap::<String, CodeBlock>::from([
            (
                "imports".to_string(),
                CodeBlock::new(
                    Language::C,
                    "#include <stdio.h>\n".to_string(),
                    "imports".to_string(),
                    vec![],
                    0,
                ),
            ),
            (
                "main_block".to_string(),
                CodeBlock::new(
                    Language::C,
                    r#"int main() {
printf("Hello, world!");
    return 0;
}
"#
                    .to_string(),
                    "main_block".to_string(),
                    vec!["imports".to_string()],
                    0,
                ),
            ),
        ]);
        let tangle = tangle_block("main_block", blocks, false);
        assert!(tangle.is_ok());
        let (tangled_code, _) = tangle.unwrap();
        assert_eq!(
            tangled_code,
            r#"#include <stdio.h>


int main() {
printf("Hello, world!");
    return 0;
}
"#
        );
    }

    /// Tests that Python code is returned without a wrapper
    /// since Python does not require a main function.
    /// The wrapper is only added for C code.
    #[test]
    fn test_tangle_block_wrapper_python() {
        let blocks = HashMap::<String, CodeBlock>::from([(
            "main".to_string(),
            CodeBlock::new(
                Language::Python,
                "print('monty python')\n".to_string(),
                "main".to_string(),
                vec![],
                0,
            ),
        )]);
        let tangle = tangle_block("main", blocks, true);
        assert!(tangle.is_ok());
        let (tangled_code, _) = tangle.unwrap();
        assert_eq!(tangled_code, "print('monty python')\n");
    }
}
