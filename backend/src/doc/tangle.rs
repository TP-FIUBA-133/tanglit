use crate::doc::CodeBlock;
use crate::utils::{get_indentation_at_offset, set_indentation};
use indexmap::IndexSet;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
const MACROS_REGEX: &str = r"@\[([a-zA-Z0-9_]+)\]";

#[derive(PartialEq)]
pub enum TangleError {
    BlockNotFound(String),
    InternalError(String),
    CycleDetected(Vec<String>),
}

impl fmt::Display for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            TangleError::CycleDetected(cycle) => {
                write!(f, "Cycle detected: {}", cycle.join(" -> "))
            }
        }
    }
}

impl fmt::Debug for TangleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TangleError::BlockNotFound(msg) => write!(f, "Block tag not found: {}", msg),
            TangleError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            TangleError::CycleDetected(cycle) => {
                write!(f, "Cycle detected: {}", cycle.join(" -> "))
            }
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
        let mut visited = IndexSet::new();
        let regex = &Regex::new(MACROS_REGEX)
            .map_err(|e| TangleError::InternalError(format!("Failed to compile regex: {}", e)))?;

        self.expand_block(target_codeblock.tag.clone(), &mut visited, regex)
    }

    /// Recursively expands a code block by resolving its macros.
    /// It keeps track of visited blocks to detect cycles and avoid infinite recursion.
    /// It also adjusts the indentation of inserted blocks to match the context.
    fn expand_block(
        &self,
        target_codeblock_name: String,
        visited: &mut IndexSet<String>,
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

            let mut macro_block_code =
                self.expand_block(block_called.to_string(), visited, regex)?;

            let placeholder_offset = macro_reference.start();
            let indent_size = get_indentation_at_offset(&target_block.code, placeholder_offset);
            set_indentation(&mut macro_block_code, Some(indent_size), Some(' '));

            expanded_block_code.push_str(&macro_block_code);

            final_index = macro_reference.end();
        }
        expanded_block_code.push_str(&target_block.code[final_index..]);

        visited.pop();

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

    fn assert_no_cycle(visited: &IndexSet<String>, node: &str) -> Result<(), TangleError> {
        if let Some(start_idx) = visited.get_index_of(node) {
            let cycle_path: Vec<String> = visited.iter().skip(start_idx).cloned().collect();
            let displayed_cycle = format_cycle_path(node, cycle_path);

            return Err(TangleError::CycleDetected(displayed_cycle));
        }
        Ok(())
    }
}

fn format_cycle_path(node: &str, mut cycle_path: Vec<String>) -> Vec<String> {
    // to complete the cycle
    cycle_path.push(node.to_string());

    let cycle_len = cycle_path.len();
    let preview_len = 3;

    let displayed_cycle: Vec<String> = if cycle_len <= 2 * preview_len {
        // short path: show entire
        cycle_path
    } else {
        // large path: first 3 + "..." + last 3
        let mut v = cycle_path[..preview_len].to_vec();
        v.push("...".to_string());
        v.extend_from_slice(&cycle_path[cycle_len - preview_len..]);
        v
    };
    displayed_cycle
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
    fn test_insert_macro_with_correct_indentation() {
        let mut blocks = HashMap::new();
        let main = CodeBlock::new(
            Option::from("python".to_string()),
            "for i in range(2):\n    @[helper]\n    print('Hello, world!')".to_string(),
            "main".to_string(),
            vec![],
            0,
        );
        blocks.insert("main".to_string(), main.clone());
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "print('Helper function')\nprint('second helper function')".to_string(),
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
            r#"for i in range(2):
    print('Helper function')
    print('second helper function')
    print('Hello, world!')"#
                .to_string()
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
        assert!(matches!(tangle.unwrap_err(), TangleError::CycleDetected(_)));
    }

    #[test]
    fn test_cycle_detection_two_level_indirect() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[B]\nprint('Block A')".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[C]\nprint('Block B')".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "C".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[A]\nprint('Block C')".to_string(),
                "C".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("A").unwrap();
        let tangle = codeblocks.tangle_codeblock(block);

        assert!(tangle.is_err());
        assert!(matches!(tangle.unwrap_err(), TangleError::CycleDetected(_)));
    }

    #[test]
    fn test_cycle_detection_one_level_indirect() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[B]\nprint('Block A')".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[A]\nprint('Block B')".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("A").unwrap();
        let tangle = codeblocks.tangle_codeblock(block);

        assert!(tangle.is_err());
        assert!(matches!(tangle.unwrap_err(), TangleError::CycleDetected(_)));
    }

    #[test]
    fn test_cycle_detection_two_level_indirect_with_other_blocks() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[B]\nprint('Block A')".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[C]\nprint('Block B')\n@[D]".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "C".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[A]\nprint('Block C')".to_string(),
                "C".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "D".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "print('Block D')".to_string(),
                "D".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("A").unwrap();
        let tangle = codeblocks.tangle_codeblock(block);

        assert!(tangle.is_err());
        assert!(matches!(tangle.unwrap_err(), TangleError::CycleDetected(_)));
    }

    #[test]
    fn test_no_cycle_repeated_dependency() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[B]\nprint('Block A')\n@[B]".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "print('Block B')".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);

        let block = codeblocks.get_block("A").unwrap();
        let tangle = codeblocks.tangle_codeblock(block).unwrap();

        assert_eq!(
            tangle,
            "print('Block B')\nprint('Block A')\nprint('Block B')".to_string()
        );
    }

    #[test]
    fn test_no_cycle_shared_dependency() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[B]\nprint('Block A')\n@[C]".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "print('Block B')".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "C".to_string(),
            CodeBlock::new(
                Option::from("python".to_string()),
                "@[B]\nprint('Block C')".to_string(),
                "C".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);
        let block = codeblocks.get_block("A").unwrap();
        let tangle = codeblocks.tangle_codeblock(block).unwrap();

        assert_eq!(
            tangle,
            "print('Block B')\nprint('Block A')\nprint('Block B')\nprint('Block C')".to_string()
        );
    }

    #[test]
    fn test_cycle_detection_short_cycle_message() {
        let mut blocks = HashMap::new();

        // Ciclo: A -> B -> A
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[B]".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[A]".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);
        let block = codeblocks.get_block("A").unwrap();
        let err = codeblocks.tangle_codeblock(block).unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("Cycle detected"));
        assert!(
            msg.contains("A -> B -> A"),
            "Unexpected cycle message: {}",
            msg
        );
    }

    #[test]
    fn test_cycle_detection_long_cycle_message() {
        let mut blocks = HashMap::new();

        // Ciclo: A -> B -> C -> D -> E -> F -> G -> A
        blocks.insert(
            "A".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[B]".to_string(),
                "A".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[C]".to_string(),
                "B".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "C".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[D]".to_string(),
                "C".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "D".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[E]".to_string(),
                "D".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "E".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[F]".to_string(),
                "E".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "F".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[G]".to_string(),
                "F".to_string(),
                vec![],
                0,
            ),
        );
        blocks.insert(
            "G".to_string(),
            CodeBlock::new(
                Some("rust".to_string()),
                "@[A]".to_string(),
                "G".to_string(),
                vec![],
                0,
            ),
        );

        let codeblocks = CodeBlocks::from_codeblocks(blocks);
        let block = codeblocks.get_block("A").unwrap();
        let err = codeblocks.tangle_codeblock(block).unwrap_err();

        let msg = err.to_string();
        assert!(msg.contains("Cycle detected"));
        assert!(msg.contains("A -> B -> C"), "Should show first 3 nodes");
        assert!(msg.contains("F -> G -> A"), "Should show last 3 nodes");
        assert!(
            msg.contains("..."),
            "Should show ellipsis for skipped nodes"
        );
    }
}
