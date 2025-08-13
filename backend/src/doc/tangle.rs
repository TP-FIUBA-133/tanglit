use crate::doc::CodeBlock;
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

fn get_codeblock(
    target_block: &str,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<CodeBlock, TangleError> {
    blocks
        .get(target_block)
        .cloned()
        .ok_or(TangleError::BlockNotFound(target_block.into()))
}

fn tangle_block(
    target_block: &str,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<String, TangleError> {
    // Get target_code_block
    let target_code_block = get_codeblock(target_block, blocks)?;
    resolve_macros(&target_code_block, blocks)
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

    Ok(code_block_with_macros.into_owned())
}

pub struct CodeBlocksDoc {
    blocks: HashMap<String, CodeBlock>,
}

pub fn from_codeblocks(blocks: HashMap<String, CodeBlock>) -> CodeBlocksDoc {
    CodeBlocksDoc { blocks }
}

impl CodeBlocksDoc {
    #[cfg(test)]
    /// This constructor is for testing purposes only
    /// User code should either use from_codeblocks (if available) or
    /// obtain one from a TanglitDoc instance via `tangle()` method
    pub fn from_codeblocks(blocks: std::collections::HashMap<String, CodeBlock>) -> Self {
        Self { blocks }
    }

    pub fn tangle_block(&self, target_block: &str) -> Result<String, TangleError> {
        tangle_block(target_block, &self.blocks)
    }

    pub fn tangle_codeblock(&self, target_codeblock: &CodeBlock) -> Result<String, TangleError> {
        resolve_macros(target_codeblock, &self.blocks)
    }

    pub fn get_block(&self, name: &str) -> Option<&CodeBlock> {
        self.blocks.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::parser::code_block::Language;

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

        let tangle = tangle_block("main", &blocks).unwrap();
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
        let result = tangle_block("main", &blocks);
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
