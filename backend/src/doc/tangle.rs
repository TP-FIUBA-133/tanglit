//use crate::doc::macro_dependency::check_dependencies;

use crate::doc::CodeBlock;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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
        let visited = &mut HashSet::new();
        let regex = &Regex::new(MACROS_REGEX)
            .map_err(|e| TangleError::InternalError(format!("Failed to compile regex: {}", e)))?;

        self.expand_block(target_codeblock.tag.clone(), visited, regex)
    }

    fn expand_block(
        &self,
        target_codeblock_name: String,
        visited: &mut HashSet<String>,
        regex: &Regex,
    ) -> Result<String, TangleError> {
        let target_block = self.get_code_block(&target_codeblock_name)?;

        Self::assert_no_cycle(visited, &target_codeblock_name)?;

        visited.insert(target_codeblock_name.clone());

        let mut expanded_block_code = String::new();
        let mut final_index = 0;

        for macro_references in regex.captures_iter(&target_block.code) {
            let macro_reference = macro_references.get(0).unwrap(); // @[A]
            let block_called = &macro_references[1]; // "A"

            expanded_block_code.push_str(&target_block.code[final_index..macro_reference.start()]);

            let macro_block_code = self.expand_block(block_called.to_string(), visited, regex)?;

            expanded_block_code.push_str(&macro_block_code);

            final_index = macro_reference.end();
        }
        expanded_block_code.push_str(&target_block.code[final_index..]);

        visited.remove(&target_codeblock_name);

        Ok(expanded_block_code)
    }

    /// Find and return the specified code block by name.
    /// Returns `None` if the block can't be found within its collection.
    pub fn get_block(&self, name: &str) -> Option<&CodeBlock> {
        self.blocks.get(name)
    }

    fn get_code_block(&self, code_name: &str) -> Result<&CodeBlock, TangleError> {
        self.blocks
            .get(code_name)
            .ok_or_else(|| TangleError::BlockNotFound(code_name.to_string()))
    }

    fn assert_no_cycle(visited: &HashSet<String>, code_name: &str) -> Result<(), TangleError> {
        (!visited.contains(code_name))
            .then_some(())
            .ok_or(TangleError::CycleDetected())
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
                "@[config]\nprint('Helper function')".to_string(),
                "helper".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "config".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "print('config function')".to_string(),
                "config".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("main").unwrap();
        let tangle = codeblocks.tangle_codeblock(block).unwrap();

        assert_eq!(
            tangle,
            "print('config function')\nprint('Helper function')\nprint('Hello, world!')"
                .to_string()
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

    #[test]
    fn test_cycle_detection() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Option::from("python".to_string()),
            "@[main]\nprint('Hello, world!')".to_string(),
            "main".to_string(),
            vec![],
            0,
        );
        blocks.insert("main".to_string(), main);
        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("main").unwrap();
        let tangle = codeblocks.tangle_codeblock(block);

        assert!(tangle.is_err());
        assert_eq!(tangle.unwrap_err(), TangleError::CycleDetected());
    }
}
