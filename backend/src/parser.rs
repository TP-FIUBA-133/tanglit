pub mod code_block;

use crate::errors::ParserError;
use code_block::CodeBlock;
use markdown::{
    ParseOptions,
    mdast::{Code, Node},
};

// TODO: We should read the file outside of this function
// Currently, blocks are strings, but they should later be a struct
pub fn parse_blocks_from_file(file_path: &str) -> Result<Vec<CodeBlock>, ParserError> {
    // Read the file content
    let input = std::fs::read_to_string(file_path)
        .map_err(|e| ParserError::InvalidInput(format!("Failed to read file: {}", e)))?;

    parse_input(input)
}

pub fn parse_input(input: String) -> Result<Vec<CodeBlock>, ParserError> {
    // Parse the input to an MDast tree
    let mdast = input_to_mdast(&input)?;

    // Extract code nodes from the tree
    let code_nodes = get_code_nodes_from_mdast(mdast)?;

    // Convert code nodes to CodeBlocks
    let code_blocks = code_nodes
        .into_iter()
        .map(CodeBlock::from_code_node)
        .collect();

    Ok(code_blocks)
}

fn input_to_mdast(input: &str) -> Result<Node, ParserError> {
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
        // Not sure if nested code_nodes are supported
        // code_nodes.extend(get_code_nodes_from_mdast(child.clone())?);
    }
    Ok(code_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_no_blocks() {
        let input = r#"
        This is just plain text.
        No code blocks here.
        "#;

        let blocks = parse_input(input.to_string()).unwrap();

        assert!(blocks.is_empty());
    }

    #[test]
    fn test_parse_input_block_and_text() {
        let input = r#"
        This is some text.
        ```python
        print("Hello, world!")
        ```
        More text here.
        "#;
        let blocks = parse_input(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].code.trim(), r#"print("Hello, world!")"#);
    }

    #[test]
    fn test_parse_input_malformed_blocks() {
        let input = r#"
        ```python
        print("Hello, world!")
        ```
        ```Rust
        fn main() {
            println!("Hello, world!");
        "#; // Missing closing backticks

        let blocks = parse_input(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].code.trim(), r#"print("Hello, world!")"#);
        assert_eq!(
            blocks[1].code.trim(),
            r#"fn main() {
    println!("Hello, world!");"#
        );
    }

    #[test]
    fn test_parse_input_multiple_blocks() {
        let input = r#"
```python
print("Block 1")
```
```javascript
console.log("Block 2");
```
```Rust
fn main() {
    println!("Block 3");
}
```"#;

        let blocks = parse_input(input.to_string()).unwrap();

        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].code, r#"print("Block 1")"#);
        assert_eq!(blocks[1].code, r#"console.log("Block 2");"#);
        assert_eq!(
            blocks[2].code,
            r#"fn main() {
    println!("Block 3");
}"#
            .trim()
        );
    }
}
