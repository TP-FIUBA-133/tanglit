use crate::errors::ParserError;
use regex::Regex;

const BLOCK_REGEX: &str = r"```(?:\w*\n)?([^`]*)```";

// Currently, blocks are strings, but they should later be a struct
pub fn parse_blocks_from_file(file_path: &str) -> Result<Vec<String>, ParserError> {
    let input = std::fs::read_to_string(file_path)
        .map_err(|e| ParserError::InvalidInput(format!("Failed to read file: {}", e)))?;

    parse_input(input.as_str())
}

fn parse_input(input: &str) -> Result<Vec<String>, ParserError> {
    let re = Regex::new(BLOCK_REGEX).unwrap();
    Ok(re
        .captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect())
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

        let blocks = parse_input(input).unwrap();
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

        let blocks = parse_input(input).unwrap();
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

        let blocks = parse_input(input).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].trim(), r#"print("Hello, world!")"#);
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

        let blocks = parse_input(input).unwrap();
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].trim(), r#"print("Block 1")"#);
        assert_eq!(blocks[1].trim(), r#"console.log("Block 2");"#);
        assert_eq!(
            blocks[2].trim(),
            r#"fn main() {
            println!("Block 3");
        }"#
        );
    }
}
