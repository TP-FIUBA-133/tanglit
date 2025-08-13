pub mod code_block;
pub mod exclude;
pub mod slides;

use code_block::CodeBlock;
use comrak::{Plugins, markdown_to_html_with_plugins, plugins::syntect::SyntectAdapterBuilder};
use markdown::{
    ParseOptions,
    mdast::{Code, Node},
};
use std::collections::HashMap;
use std::fmt;

// Taken from https://github.com/sindresorhus/github-markdown-css/blob/bedb4b771f5fa1ae117df597c79993fd1eb4dff0/github-markdown-light.css
const GITHUB_MARKDOWN_LIGHT_CSS: &str = include_str!("../../resources/github-markdown-light.css");

pub enum ParserError {
    InvalidInput(String),
    CodeBlockError(String),
    AstConversionError(String),
    HtmlConversionError(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::CodeBlockError(msg) => write!(f, "Error parsing Code Block: {}", msg),
            ParserError::AstConversionError(msg) => {
                write!(f, "Error converting AST back to markdown: {}", msg)
            }
            ParserError::HtmlConversionError(msg) => {
                write!(f, "Error converting markdown to HTML: {}", msg)
            }
        }
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ParserError::CodeBlockError(msg) => write!(f, "Error parsing Code Block: {}", msg),
            ParserError::AstConversionError(msg) => {
                write!(f, "Error converting AST back to markdown: {}", msg)
            }
            ParserError::HtmlConversionError(msg) => {
                write!(f, "Error converting markdown to HTML: {}", msg)
            }
        }
    }
}

pub fn parse_from_string(input: &str) -> Result<Node, ParserError> {
    markdown::to_mdast(input, &ParseOptions::mdx())
        .map_err(|e| ParserError::InvalidInput(format!("Failed to parse input: {}", e)))
}

pub fn ast_to_markdown(ast: &Node) -> Result<String, ParserError> {
    let default_options = mdast_util_to_markdown::Options::default();
    let options = mdast_util_to_markdown::Options {
        bullet: '-',
        rule: '-',
        ..default_options
    };
    mdast_util_to_markdown::to_markdown_with_options(ast, &options).map_err(|e| {
        ParserError::AstConversionError(format!("Error converting AST to markdown: {}", e))
    })
}

/// Parses code blocks from a given input string
/// Returns a HashMap where the key is the tag of the code block and the value is the CodeBlock struct
/// If a code block does not have a tag, a default tag is assigned based on their line number in the input
pub fn parse_code_blocks_from_ast(mdast: &Node) -> Result<HashMap<String, CodeBlock>, ParserError> {
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

fn get_code_nodes_from_mdast(mdast: &Node) -> Result<Vec<Code>, ParserError> {
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

// TODO: Make all options configurable
pub fn markdown_to_html(input: &str) -> String {
    // InspiredGitHub
    // Solarized (dark)
    // Solarized (light)
    // base16-eighties.dark
    // base16-mocha.dark
    // base16-ocean.dark
    // base16-ocean.light
    let adapter = SyntectAdapterBuilder::new()
        .theme("base16-ocean.light")
        .build();

    let mut options = comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tagfilter = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.extension.header_ids = Some("user-content-".to_string()); // mimics GitHub's prefix
    options.render.github_pre_lang = true;

    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let inner_html = markdown_to_html_with_plugins(input, &options, &plugins);
    format!(
        r#"<style>{}</style><div class="markdown-body">{}</div>"#,
        GITHUB_MARKDOWN_LIGHT_CSS, inner_html
    )
}

#[cfg(test)]
mod tests {
    use crate::doc::parser::code_block::Language;

    use super::*;

    fn parse_code_blocks_from_string(
        input: &str,
    ) -> Result<HashMap<String, CodeBlock>, ParserError> {
        let mdast = parse_from_string(input)?;
        parse_code_blocks_from_ast(&mdast)
    }

    #[test]
    fn test_parse_code_blocks_no_blocks() {
        let input = r#"
        This is just plain text.
        No code blocks here.
        "#;

        let blocks = parse_code_blocks_from_string(input).unwrap();

        assert!(blocks.is_empty());
    }

    #[test]
    fn test_parse_code_blocks_block_and_text() {
        let input = r#"
        This is some text.
        ```python hello
        print("Hello, world!")
        ```
        More text here.
        "#;
        let blocks = parse_code_blocks_from_string(input).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks.get("hello").unwrap().code.trim(),
            r#"print("Hello, world!")"#
        );
    }

    #[test]
    fn test_parse_code_blocks_malformed_blocks() {
        let input = r#"
        ```python hello_python
        print("Hello, world!")
        ```
        ```Rust hello_rust
        fn main() {
            println!("Hello, world!");
        "#; // Missing closing backticks

        let blocks = parse_code_blocks_from_string(input).unwrap();

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
    fn test_parse_code_blocks_multiple_blocks() {
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

        let blocks = parse_code_blocks_from_string(input).unwrap();

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
        let blocks = parse_code_blocks_from_string(input).unwrap();

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

    #[test]
    fn test_parse_code_blocks_without_tag() {
        let input = r#"
```python
print("Hello, world!")
```"#;
        let blocks = parse_code_blocks_from_string(input).unwrap();
        assert_eq!(blocks.len(), 1);
        let block = blocks.get("2").unwrap(); // Default tag based on line number
        assert_eq!(block.code.trim(), r#"print("Hello, world!")"#);
        assert_eq!(block.tag, "2".to_string());
        assert!(block.imports.is_empty());
        assert_eq!(block.language, Language::Python);
    }

    #[test]
    fn test_parse_code_blocks_various_blocks() {
        let input = r#"
```python
print("Hello, world!")
```
```javascript
console.log("Hello, world!");
```
```Rust rust
println!("Hello, world!");
```"#;
        let blocks = parse_code_blocks_from_string(input).unwrap();

        assert_eq!(blocks.len(), 3);
        assert_eq!(
            blocks.get("2").unwrap().code.trim(),
            r#"print("Hello, world!")"#
        );
        assert_eq!(
            blocks.get("5").unwrap().code.trim(),
            r#"console.log("Hello, world!");"#
        );
        assert_eq!(
            blocks.get("rust").unwrap().code.trim(),
            r#"println!("Hello, world!");"#
        );
    }
}
