pub mod code_block;
pub mod exclude;
pub mod slides;

use crate::errors::ParserError;
use code_block::CodeBlock;
use markdown::{
    ParseOptions,
    mdast::{Code, Node},
};
use std::collections::HashMap;

pub fn parse_blocks_from_file(file_path: &str) -> Result<HashMap<String, CodeBlock>, ParserError> {
    // Read the file content
    let input = std::fs::read_to_string(file_path)
        .map_err(|e| ParserError::InvalidInput(format!("Failed to read file: {}", e)))?;

    parse_code_blocks(input)
}

/// Parses code blocks from a given input string
/// Returns a HashMap where the key is the tag of the code block and the value is the CodeBlock struct
/// If a code block does not have a tag, a default tag is assigned based on their line number in the input
pub fn parse_code_blocks(input: String) -> Result<HashMap<String, CodeBlock>, ParserError> {
    // Parse the input to an MDast tree
    let mdast = input_to_mdast(&input)?;

    // Extract code nodes from the tree
    let code_nodes = get_code_nodes_from_mdast(mdast)?;

    // Convert code nodes to CodeBlocks
    let code_blocks: Vec<CodeBlock> = code_nodes
        .into_iter()
        .map(CodeBlock::from_code_node)
        .collect::<Result<_, _>>()?;

    // Create a HashMap from the code blocks
    let code_block_map = code_blocks
        .into_iter()
        .map(|cb| (cb.tag.clone(), cb))
        .collect();

    Ok(code_block_map)
}

pub fn input_to_mdast(input: &str) -> Result<Node, ParserError> {
    markdown::to_mdast(input, &ParseOptions::mdx())
        .map_err(|e| ParserError::InvalidInput(format!("Failed to parse input: {}", e)))
}

fn get_code_nodes_from_mdast(mdast: Node) -> Result<Vec<Code>, ParserError> {
    let mut code_nodes = Vec::new();
    let Some(children) = mdast.children() else {
        return Ok(code_nodes);
    };
    for child in children {
        if let Node::Code(code_block) = child {
            code_nodes.push(code_block.clone());
        }
    }
    Ok(code_nodes)
}

#[cfg(test)]
mod tests {
    use crate::parser::code_block::Language;

    use super::*;

    #[test]
    fn test_parse_input_no_blocks() {
        let input = r#"
        This is just plain text.
        No code blocks here.
        "#;

        let blocks = parse_code_blocks(input.to_string()).unwrap();

        assert!(blocks.is_empty());
    }

    #[test]
    fn test_parse_input_block_and_text() {
        let input = r#"
        This is some text.
        ```python hello
        print("Hello, world!")
        ```
        More text here.
        "#;
        let blocks = parse_code_blocks(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks.get("hello").unwrap().code.trim(),
            r#"print("Hello, world!")"#
        );
    }

    #[test]
    fn test_parse_input_malformed_blocks() {
        let input = r#"
        ```python hello_python
        print("Hello, world!")
        ```
        ```Rust hello_rust
        fn main() {
            println!("Hello, world!");
        "#; // Missing closing backticks

        let blocks = parse_code_blocks(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(
            blocks.get("hello_python").unwrap().code.trim(),
            r#"print("Hello, world!")"#
        );
        assert_eq!(
            blocks.get("hello_rust").unwrap().code.trim(),
            r#"fn main() {
    println!("Hello, world!");"#
        );
    }

    #[test]
    fn test_parse_input_multiple_blocks() {
        let input = r#"
```python block_1
print("Block 1")
```
```javascript block_2
console.log("Block 2");
```
```Rust block_3
fn main() {
    println!("Block 3");
}
```"#;

        let blocks = parse_code_blocks(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks.get("block_1").unwrap().code, r#"print("Block 1")"#);
        assert_eq!(
            blocks.get("block_2").unwrap().code,
            r#"console.log("Block 2");"#
        );
        assert_eq!(
            blocks.get("block_3").unwrap().code,
            r#"fn main() {
    println!("Block 3");
}"#
            .trim()
        );
    }

    #[test]
    fn test_parse_integration() {
        let input = r#"
```python use=[block1, block2] block3
print("Hello, world!")
```"#;
        let blocks = parse_code_blocks(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks.get("block3").unwrap().code.trim(),
            r#"print("Hello, world!")"#
        );
        assert_eq!(blocks.get("block3").unwrap().tag, "block3".to_string());
        assert_eq!(
            blocks.get("block3").unwrap().imports,
            vec!["block1".to_string(), "block2".to_string()]
        );
        assert_eq!(blocks.get("block3").unwrap().language, Language::Python);
    }
}
