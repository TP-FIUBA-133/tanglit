use crate::errors::ParserError;

// Currently, blocks are strings, but they should later be a struct
pub fn parse_blocks_from_file(file_path: &str) -> Result<Vec<String>, ParserError> {
    let input = std::fs::read_to_string(file_path)
        .map_err(|e| ParserError::InvalidInput(format!("Failed to read file: {}", e)))?;

    parse_markdown(input.as_str())
}

fn parse_markdown(input: &str) -> Result<Vec<String>, ParserError> {
    // TODO: remove unwrap
    let mdast = markdown::to_mdast(input, &markdown::ParseOptions::mdx()).unwrap();
    get_blocks(mdast)
}

fn get_blocks(mdast: markdown::mdast::Node) -> Result<Vec<String>, ParserError> {
    let mut blocks = Vec::new();
    let Some(children) = mdast.children() else {
        return Ok(blocks);
    };
    for child in children {
        if let markdown::mdast::Node::Code(code_block) = child {
            blocks.push(code_block.value.clone());
        }
        // blocks.extend(get_blocks(child.clone())?);
    }
    Ok(blocks)
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

        let blocks = parse_markdown(input).unwrap();
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

        let blocks = parse_markdown(input).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].trim(), r#"print("Hello, world!")"#);
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

        let blocks = parse_markdown(input).unwrap();
        println!("{:?}", blocks);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].trim(), r#"print("Hello, world!")"#);
        assert_eq!(
            blocks[1].trim(),
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

        let blocks = parse_markdown(input).unwrap();
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0], r#"print("Block 1")"#);
        assert_eq!(blocks[1], r#"console.log("Block 2");"#);
        assert_eq!(
            blocks[2],
            r#"fn main() {
    println!("Block 3");
}"#
            .trim()
        );
    }
}
