mod error;
mod generate_pdf;
mod parser;
mod tangle;

use crate::doc::generate_pdf::generate_pdf;
use crate::doc::parser::slides::parse_slides_from_ast;
use crate::doc::parser::{
    ast_to_markdown, markdown_to_html, parse_code_blocks_from_ast, parse_from_string,
};
use crate::execution::ExecutionOutput;
pub use error::DocError;
use markdown::mdast::Node;
pub use parser::ParserError;
pub use parser::code_block::CodeBlock;
use parser::exclude::exclude_from_ast;
pub use parser::slides::SlideByIndex;
use parser::slides::parse_slides_index_from_ast;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
pub use tangle::CodeBlocks;
pub use tangle::TangleError;

pub struct TanglitDoc {
    raw_markdown: String,
    ast: Node,
}

#[derive(Debug, Clone, Serialize)]
pub struct Edit {
    pub content: String,
    pub line: usize,
    pub offset: usize,
}

impl TanglitDoc {
    pub fn new_from_string(raw_markdown: &str) -> Result<TanglitDoc, DocError> {
        let ast = parse_from_string(raw_markdown)?;
        Ok(TanglitDoc {
            raw_markdown: raw_markdown.to_string(),
            ast,
        })
    }

    pub fn new_from_file(file_path: &str) -> Result<TanglitDoc, DocError> {
        let input = std::fs::read_to_string(file_path)
            .map_err(|e| ParserError::InvalidInput(format!("Failed to read file: {}", e)))?;
        Self::new_from_string(&input)
    }

    fn parse_blocks(&self) -> Result<HashMap<String, CodeBlock>, DocError> {
        Ok(parse_code_blocks_from_ast(&self.ast)?)
    }

    pub fn get_block(
        &self,
        block_name: &str,
        blocks: &HashMap<String, CodeBlock>,
    ) -> Result<CodeBlock, DocError> {
        blocks
            .get(block_name)
            .cloned()
            .ok_or(DocError::TangleError(TangleError::BlockNotFound(
                block_name.to_string(),
            )))
    }

    pub fn parse_slides_index(&self) -> Vec<SlideByIndex> {
        parse_slides_index_from_ast(&self.ast, &self.raw_markdown)
    }

    pub fn generate_md_slides(&self, output_dir: String) -> Result<(), DocError> {
        let slides = parse_slides_from_ast(&self.ast, &self.raw_markdown);

        for (i, slide) in slides.iter().enumerate() {
            let slide_md = slide.to_markdown()?;
            fs::write(format!("{}/slide_{}.md", output_dir, i), slide_md)?;
        }

        Ok(())
    }

    pub fn format_output(
        &self,
        block_id: &str,
        output: &ExecutionOutput,
    ) -> Result<Edit, DocError> {
        let binding = self.get_code_blocks()?;
        let code_block = binding
            .get_block(block_id)
            .ok_or_else(|| TangleError::BlockNotFound(block_id.to_string()))?;

        let output_content = format!(
            "```output\nOutput:\n{}\n\nStderr:\n{}\n\nExit code: {}\n```",
            output.stdout,
            output.stderr,
            output.status.map_or("None".to_string(), |s| s.to_string())
        );

        let lines: Vec<&str> = self.raw_markdown.lines().collect();
        let code_end_line = code_block.end_line;

        // Look for an existing output block starting after the code block
        let mut output_start_line = None;
        let mut output_end_line = None;

        // Start searching from the line immediately after the code block
        for (line_idx, line) in lines.iter().enumerate().skip(code_end_line) {
            let trimmed = line.trim();

            if trimmed == "```output" {
                output_start_line = Some(line_idx);

                // Find the closing ``` for this output block
                for (end_idx, end_line) in lines.iter().enumerate().skip(line_idx + 1) {
                    if end_line.trim() == "```" {
                        output_end_line = Some(end_idx);
                        break;
                    }
                }
                break;
            } else if !trimmed.is_empty() {
                // Hit non-empty content that's not an output block, stop looking
                break;
            }
        }

        match (output_start_line, output_end_line) {
            (Some(start), Some(end)) => {
                // Replace existing output block
                // Calculate how many lines to replace (inclusive of both start and end lines)
                let lines_to_replace = end - start + 1;
                Ok(Edit {
                    content: output_content,
                    line: start + 1, // Monaco uses 1-based line numbers
                    offset: lines_to_replace,
                })
            }
            _ => {
                // Insert new output block after the code block
                Ok(Edit {
                    content: format!("\n{}", output_content),
                    line: code_end_line + 1,
                    offset: 0, // 0 means insert, don't replace
                })
            }
        }
    }

    pub fn exclude(&self) -> Result<String, DocError> {
        let ast_with_exclusions = exclude_from_ast(&self.ast);
        Ok(ast_to_markdown(&ast_with_exclusions)?)
    }

    pub fn get_code_blocks(&self) -> Result<CodeBlocks, DocError> {
        let blocks = self.parse_blocks()?;
        Ok(CodeBlocks::from_codeblocks(blocks))
    }

    pub fn generate_html(&self) -> Result<String, DocError> {
        let markdown_with_exclusions = self.exclude()?;
        Ok(markdown_to_html(&markdown_with_exclusions))
    }
    pub fn generate_pdf(&self, output_file_path: &str) -> Result<(), DocError> {
        let html_with_exclusions = self.generate_html()?;
        generate_pdf(&html_with_exclusions, output_file_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::ExecutionOutput;

    #[test]
    fn test_format_output_insert_new_block() {
        let markdown = r#"# Test Document

```rust hello
println!("Hello, world!");
```

Some other content here.
"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "Hello, world!\n".to_string(),
            stderr: "".to_string(),
            status: Some(0),
        };

        let edit = doc.format_output("hello", &output).unwrap();

        assert_eq!(edit.line, 6); // Line after the code block
        assert_eq!(edit.offset, 0); // Insert, don't replace
        assert!(edit.content.contains("```output"));
        assert!(edit.content.contains("Hello, world!"));
        assert!(edit.content.contains("Exit code: 0"));
    }

    #[test]
    fn test_format_output_replace_existing_block() {
        let markdown = r#"# Test Document

```rust hello
println!("Hello, world!");
```

```output
Output:
Old output

Stderr:


Exit code: 0
```

Some other content here.
"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "New output!\n".to_string(),
            stderr: "Some warning".to_string(),
            status: Some(1),
        };

        let edit = doc.format_output("hello", &output).unwrap();

        assert_eq!(edit.line, 7); // Start of existing output block (1-based)
        assert_eq!(edit.offset, 9); // Replace 8 lines (from ```output to ```)
        assert!(edit.content.contains("New output!"));
        assert!(edit.content.contains("Some warning"));
        assert!(edit.content.contains("Exit code: 1"));
    }

    #[test]
    fn test_format_output_skip_non_output_blocks() {
        let markdown = r#"# Test Document

```rust hello
println!("Hello, world!");
```

```python
# This is not an output block
print("Different block")
```

Some other content here.
"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "Hello, world!\n".to_string(),
            stderr: "".to_string(),
            status: Some(0),
        };

        let edit = doc.format_output("hello", &output).unwrap();

        assert_eq!(edit.line, 6); // Line after the code block
        assert_eq!(edit.offset, 0); // Insert, don't replace (didn't find output block)
    }

    #[test]
    fn test_format_output_with_empty_lines_before_output() {
        let markdown = r#"# Test Document

```rust hello
println!("Hello, world!");
```


```output
Output:
Old output

Stderr:


Exit code: 0
```
"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "New output!\n".to_string(),
            stderr: "".to_string(),
            status: Some(0),
        };

        let edit = doc.format_output("hello", &output).unwrap();

        assert_eq!(edit.line, 8); // Start of existing output block (1-based)
        assert_eq!(edit.offset, 9); // Replace the output block
        assert!(edit.content.contains("New output!"));
    }

    #[test]
    fn test_format_output_with_multiline_output() {
        let markdown = r#"```rust multiline
for i in 1..=3 {
    println!("Line {}", i);
}
```"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "Line 1\nLine 2\nLine 3\n".to_string(),
            stderr: "".to_string(),
            status: Some(0),
        };

        let edit = doc.format_output("multiline", &output).unwrap();

        assert!(edit.content.contains("Line 1\nLine 2\nLine 3"));
    }

    #[test]
    fn test_format_output_replace_with_different_content() {
        let markdown = r#"```rust counter
let mut count = 0;
count += 1;
println!("{}", count);
```

```output
Output:
1

Stderr:


Exit code: 0
```"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "42".to_string(),
            stderr: "some warning".to_string(),
            status: Some(0),
        };

        let edit = doc.format_output("counter", &output).unwrap();

        assert_eq!(edit.offset, 9); // Should replace the existing block
        assert!(edit.content.contains("42"));
        assert!(edit.content.contains("some warning"));
        assert!(!edit.content.contains("1\n")); // Old output shouldn't be there
    }

    #[test]
    #[should_panic] // This test expects the current behavior where unwrap() panics
    fn test_format_output_nonexistent_block() {
        let markdown = r#"```rust hello
println!("Hello, world!");
```"#;

        let doc = TanglitDoc::new_from_string(markdown).unwrap();
        let output = ExecutionOutput {
            stdout: "test".to_string(),
            stderr: "".to_string(),
            status: Some(0),
        };

        // This should panic because "nonexistent" block doesn't exist
        doc.format_output("nonexistent", &output).unwrap();
    }
}
