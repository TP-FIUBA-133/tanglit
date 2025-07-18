use crate::{errors::TangleError, parser::code_block::CodeBlock};
use regex::Regex;
use std::collections::{HashMap, HashSet};

const MACROS_REGEX: &str = r"@\[([a-zA-Z0-9_]+)\]";

pub fn tangle_blocks(blocks: Vec<CodeBlock>) -> String {
    let mut tangle = String::new();
    for block in blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
    }
    tangle
}

pub fn tangle_codeblock(
    block: &CodeBlock,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<String, TangleError> {
    resolve_macros(block, &blocks)
}

pub fn get_codeblock(
    target_block: &str,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<CodeBlock, TangleError> {
    blocks
        .get(target_block)
        .cloned()
        .ok_or(TangleError::BlockNotFound(target_block.into()))
}

pub fn tangle_block(
    target_block: &str,
    blocks: HashMap<String, CodeBlock>,
) -> Result<String, TangleError> {
    // Get target_code_block
    let target_code_block = get_codeblock(target_block, &blocks)?;

    tangle_codeblock(&target_code_block, &blocks)
}

/// Resolves macros in a code block by replacing them with the content of the referenced blocks.
fn resolve_macros(
    code_block: &CodeBlock,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<String, TangleError> {
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

    let code = code_block_with_macros.into_owned();

    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::code_block::Language;

    #[test]
    fn test_tangle_blocks() {
        let blocks = vec![
            CodeBlock::new_with_code("print('Hello, world!')".to_string()),
            CodeBlock::new_with_code("console.log('Hello, world!')".to_string()),
        ];
        let tangle = tangle_blocks(blocks);
        assert_eq!(
            tangle,
            "print('Hello, world!')\nconsole.log('Hello, world!')\n"
        );
    }

    #[test]
    fn test_tangle_blocks_single() {
        let blocks = vec![CodeBlock::new_with_code(
            "print('Hello, world!')".to_string(),
        )];
        let tangle = tangle_blocks(blocks);
        assert_eq!(tangle, "print('Hello, world!')\n");
    }

    #[test]
    // Tests that imports aren't inserted into the tangled output
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

        let tangle = tangle_block("main", blocks).unwrap();
        assert_eq!(tangle, "print('Hello, world!')".to_string());
    }

    // tests that missing imports blocks don't cause an error
    // This is a regression test for a bug where missing imports would cause an error
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
        let result = tangle_block("main", blocks);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_macros() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
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

        let result = resolve_macros(&main, &blocks);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "print('Helper function')\nprint('Hello, world!')".to_string()
        );
    }

    #[test]
    fn test_resolve_macros_with_missing_block() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Language::Python,
            "@[helper]\nprint('Hello, world!')".to_string(),
            "main".to_string(),
            vec![],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        let result = resolve_macros(&main, &blocks);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TangleError::BlockNotFound("helper".to_string())
        );
    }
}
