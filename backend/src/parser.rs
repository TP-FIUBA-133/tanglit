use crate::errors::ParserError;
use regex::Regex;

const BLOCK_REGEX: &str = r"```(?:\w*\n)?([^`]*)```";
const TAG_REGEX: &str = r"@tanglit-block-def:(\w+)";

// Currently, blocks are strings, but they should later be a struct
pub fn parse_blocks_from_file(file_path: &str) -> Result<Vec<String>, ParserError> {
    let input = std::fs::read_to_string(file_path)
        .map_err(|e| ParserError::InvalidInput(format!("Failed to read file: {}", e)))?;

    parse_markdown(input.as_str())
}

fn parse_input(input: &str) -> Result<Vec<String>, ParserError> {
    let re = Regex::new(BLOCK_REGEX).unwrap();
    Ok(re
        .captures_iter(input)
        .map(|cap| cap[1].to_string())
        .collect())
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

fn get_block_tags(blocks: Vec<String>) -> Result<Vec<(String, String)>, ParserError> {
    let mut block_with_tags = Vec::new();
    let re = Regex::new(TAG_REGEX).unwrap();

    for (index, block) in blocks.iter().enumerate() {
        if let Some(caps) = re.captures(block) {
            if let Some(tag) = caps.get(1) {
                // Remove the matched tag line from the block
                let cleaned_block = re.replace(block, "").trim_start_matches('\n').to_string();
                block_with_tags.push((tag.as_str().to_string(), cleaned_block));
                continue;
            }
        }
        // Fallback to auto-generated tag if no match
        block_with_tags.push((format!("block_{}", index), block.clone()));
    }

    Ok(block_with_tags)
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

    #[test]
    fn test_parse_input_block_with_macro() {
        let input = r#"
        ```python
        @tanglit-block-def:hello_world
        print("Hello, world!")
        ```
        "#;

        let blocks = parse_markdown(input).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0].trim(),
            r#"@tanglit-block-def:hello_world
print("Hello, world!")"#
        );
        let block_tags = get_block_tags(vec![
            "@tanglit-block-def:hello_world\nprint('Hello, world!')".to_string(),
            "print('Hello, world!')".to_string(),
        ])
        .unwrap();
        assert_eq!(block_tags.len(), 2);
        assert_eq!(block_tags[0].0, "hello_world");
        assert_eq!(block_tags[0].1, "print('Hello, world!')");
        assert_eq!(block_tags[1].0, "block_1");
        assert_eq!(block_tags[1].1, "print('Hello, world!')");
    }
}
