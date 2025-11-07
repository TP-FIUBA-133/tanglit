mod error;
mod format_blocks;
mod gen_html;
mod generate_pdf;
mod parser;
mod tangle;

use crate::doc::format_blocks::format_code_blocks;
pub use crate::doc::gen_html::DEFAULT_THEME;
use crate::doc::gen_html::{
    AVAILABLE_THEMES, CUSTOM_CSS, GITHUB_MARKDOWN_LIGHT_CSS, PAGE_BREAK_AND_CENTER_CSS,
    markdown_to_html, markdown_to_html_fragment, wrap_in_html_doc,
};
use crate::doc::generate_pdf::generate_pdf;
use crate::doc::parser::exclude::FilterTarget;
use crate::doc::parser::slides::parse_slides_from_ast;
use crate::doc::parser::{ast_to_markdown, parse_code_blocks_from_ast, parse_from_string};
use crate::execution::ExecutionOutput;
use crate::execution::write_code_to_file;
use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{Arena, ComrakOptions, Plugins, parse_document};
pub use error::DocError;
use log::warn;
use markdown::mdast::Node;
pub use parser::ParserError;
pub use parser::code_block::CodeBlock;
use parser::exclude::exclude_from_ast;
pub use parser::slides::SlideByIndex;
use parser::slides::parse_slides_index_from_ast;
use serde::Serialize;
use std::collections::HashMap;
use syntect::highlighting::ThemeSet;
pub use tangle::CodeBlocks;
pub use tangle::TangleError;

pub struct TanglitDoc {
    raw_markdown: String,
    ast: Node,
}

#[derive(Debug, Clone, Serialize)]
pub struct Edit {
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
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

    pub fn generate_md_slides_vec(&self) -> Result<Vec<String>, DocError> {
        let ast_with_exclusions = exclude_from_ast(&self.ast, FilterTarget::Slides);
        let slides = parse_slides_from_ast(&ast_with_exclusions, &self.raw_markdown);
        let mut v: Vec<String> = vec![];
        for slide in slides.iter() {
            let slide_md = slide.to_markdown()?;
            v.push(slide_md);
        }

        Ok(v)
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
                    start_line: start + 1, // Monaco uses 1-based line numbers
                    end_line: lines_to_replace + start + 1,
                })
            }
            _ => {
                // Insert new output block after the code block
                Ok(Edit {
                    content: format!("\n{}", output_content),
                    start_line: code_end_line + 1,
                    end_line: code_end_line + 1, // 0 means insert, don't replace
                })
            }
        }
    }

    pub fn filter_content_for_doc(&self) -> Result<String, DocError> {
        let ast_with_exclusions = exclude_from_ast(&self.ast, FilterTarget::Doc);
        Ok(ast_to_markdown(&ast_with_exclusions)?)
    }

    pub fn get_code_blocks(&self) -> Result<CodeBlocks, DocError> {
        let blocks = self.parse_blocks()?;
        Ok(CodeBlocks::from_codeblocks(blocks))
    }

    pub fn generate_html(&self, theme: &str) -> Result<String, DocError> {
        let markdown_with_exclusions = self.filter_content_for_doc()?;

        let arena = Arena::new();
        let root = parse_document(&arena, &markdown_with_exclusions, &ComrakOptions::default());

        format_code_blocks(root, &arena); // add block names, etc.

        let mut html = vec![];
        let mut options = comrak::Options::default();
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tagfilter = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;
        options.extension.footnotes = true;
        options.extension.header_ids = Some("user-content-".to_string()); // mimics GitHub's prefix
        options.render.github_pre_lang = true;
        options.render.unsafe_ = true; // Allow raw HTML, this is needed for format_code_blocks to work

        let builder = SyntectAdapterBuilder::new().theme("Solarized (dark)");
        let adapter = builder.build();
        let mut plugins = Plugins::default();

        let ts = ThemeSet::load_defaults();

        println!("Available built-in themes:");
        for name in ts.themes.keys() {
            println!("- {}", name);
        }

        plugins.render.codefence_syntax_highlighter = Some(&adapter);

        comrak::format_html_with_plugins(root, &options, &mut html, &plugins)?;

        let inner_html = String::from_utf8(html).unwrap();

        let mut final_theme = theme.to_string();
        if !AVAILABLE_THEMES.contains(&theme) {
            warn!(
                "Theme '{}' is not available. Available themes: {:?}",
                theme, AVAILABLE_THEMES
            );
            warn!("Falling back to default theme {}", DEFAULT_THEME);
            final_theme = DEFAULT_THEME.to_string();
        }

        Ok(wrap_in_html_doc(
            &inner_html,
            "Document", // TODO get title from arg or extract from markdown
            &[
                crate::doc::gen_html::get_theme_css(final_theme.as_str())
                    .unwrap()
                    .to_string(),
                CUSTOM_CSS.to_string(),
            ],
        ))
    }

    pub fn generate_doc_pdf(&self, output_file_path: &str, theme: &str) -> Result<(), DocError> {
        let markdown_with_exclusions = self.filter_content_for_doc()?;
        let html_with_exclusions = markdown_to_html(&markdown_with_exclusions, theme);
        generate_pdf(&html_with_exclusions, output_file_path)?;
        Ok(())
    }

    pub fn generate_slides_pdf(&self, output_file_path: &str) -> Result<(), DocError> {
        let slides_md = self.generate_md_slides_vec()?;

        // Build the HTML for all slides
        let mut slides_sections = String::new();
        for slide_md in slides_md.iter() {
            let frag = markdown_to_html_fragment(slide_md);
            slides_sections.push_str(&format!(r#"<section class="slide">{}</section>"#, frag));
        }

        // Wrap once into a complete HTML document.
        let all_slides_html = wrap_in_html_doc(
            &slides_sections,
            "Slides",
            &[
                GITHUB_MARKDOWN_LIGHT_CSS.to_string(),
                PAGE_BREAK_AND_CENTER_CSS.to_string(),
            ],
        );

        generate_pdf(&all_slides_html, output_file_path)?;
        Ok(())
    }

    pub fn generate_code_files(&self, output_dir: &str) -> Result<usize, DocError> {
        let blocks = self.get_code_blocks()?;
        let blocks_to_tangle = blocks.get_all_blocks_to_tangle();
        let count = blocks_to_tangle.len();
        for block in blocks_to_tangle.iter() {
            let tangle_result = blocks.tangle_codeblock(block)?;
            write_code_to_file(block, tangle_result, output_dir)?;
        }
        Ok(count)
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

        assert_eq!(edit.start_line, 6); // Line after the code block
        assert_eq!(edit.end_line, 6); // Insert, don't replace
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

        assert_eq!(edit.start_line, 7); // Start of existing output block (1-based)
        assert_eq!(edit.end_line, 16); // Replace 9 lines (from ```output to ```)
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

        assert_eq!(edit.start_line, 6); // Line after the code block
        assert_eq!(edit.end_line, 6); // Insert, don't replace (didn't find output block)
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

        assert_eq!(edit.start_line, 8); // Start of existing output block (1-based)
        assert_eq!(edit.end_line, 17); // Replace the output block
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

        assert_eq!(edit.start_line, 7);
        assert_eq!(edit.end_line, 16); // Should replace the existing block
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
