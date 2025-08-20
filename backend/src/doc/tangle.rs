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

pub struct CodeBlocks {
    blocks: HashMap<String, CodeBlock>,
}

pub fn from_codeblocks(blocks: HashMap<String, CodeBlock>) -> CodeBlocks {
    CodeBlocks { blocks }
}

impl CodeBlocks {
    /// This constructor is for testing purposes only
    /// User code should either use from_codeblocks (if available) or
    /// obtain one from a TanglitDoc instance via `tangle()` method
    pub fn from_codeblocks(blocks: std::collections::HashMap<String, CodeBlock>) -> Self {
        Self { blocks }
    }

    /// Tangles a code block by resolving its macros and producing a
    /// string with all referenced blocks inlined.
    pub fn tangle_codeblock(&self, target_codeblock: &CodeBlock) -> Result<String, TangleError> {
        let re = Regex::new(MACROS_REGEX)
            .map_err(|e| TangleError::InternalError(format!("Failed to compile regex: {}", e)))?;

        // Collect all referenced block names
        let referenced_blocks: HashSet<_> = re
            .captures_iter(&target_codeblock.code)
            .map(|caps| caps[1].to_string())
            .collect();

        // Check for missing blocks
        let missing: Vec<_> = referenced_blocks
            .iter()
            .filter(|tag| !&self.blocks.contains_key(*tag))
            .collect();

        if let Some(missing) = missing.first() {
            return Err(TangleError::BlockNotFound(missing.to_string()));
        }

        // Replace imports with block content
        let code_block_with_macros =
            re.replace_all(&target_codeblock.code, |caps: &regex::Captures| {
                self.blocks
                    .get(&caps[1])
                    .unwrap() // It is safe to unwrap here because we checked for missing blocks above
                    .code
                    .clone()
            });

        Ok(code_block_with_macros.into_owned())
    }

    /// Find and return the specified code block by name.
    /// Returns `None` if the block can't be found within its collection.
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

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("main").unwrap();
        let tangle = codeblocks.tangle_codeblock(block).unwrap();
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
        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("main").unwrap();
        let tangle = codeblocks.tangle_codeblock(block).unwrap();
        assert_eq!(tangle, "print('Hello, world!')".to_string());
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

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("main").unwrap();
        let tangle = codeblocks.tangle_codeblock(block).unwrap();

        assert_eq!(
            tangle,
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

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("main").unwrap();
        let tangle = codeblocks.tangle_codeblock(block);

        assert!(tangle.is_err());
        assert_eq!(
            tangle.unwrap_err(),
            TangleError::BlockNotFound("helper".to_string())
        );
    }
}
