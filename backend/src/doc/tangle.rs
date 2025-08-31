use crate::doc::macro_dependency::check_dependencies;

use crate::doc::CodeBlock;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
const MACROS_REGEX: &str = r"@\[([a-zA-Z0-9_]+)\]";

#[derive(PartialEq)]
pub enum TangleError {
    BlockNotFound(String),
    InternalError(String),
    CycleDetected(),
}

impl fmt::Display for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            TangleError::CycleDetected() => write!(f, "Cycle detected"),
        }
    }
}

impl fmt::Debug for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            TangleError::CycleDetected() => write!(f, "Cycle detected"),
        }
    }
}

pub struct CodeBlocks {
    pub blocks: HashMap<String, CodeBlock>,
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

        check_dependencies(&target_codeblock.tag, &self.blocks)?;

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

    #[test]
    // Tests that imports aren't inserted into the tangled output
    fn test_tangle_block_with_imports() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "print('Hello, world!')".to_string(),
                "main".to_string(),
                vec!["helper".to_string()],
                0,
            ),
        );
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
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
                Option::from("python".to_string()),
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
            Option::from("python".to_string()),
            "@[helper]\nprint('Hello, world!')".to_string(),
            "main".to_string(),
            vec![],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
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
            Option::from("python".to_string()),
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
