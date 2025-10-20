mod error;
mod gen_html;
mod generate_pdf;
mod parser;
mod tangle;

use crate::doc::gen_html::{
    CUSTOM_CSS, GITHUB_MARKDOWN_LIGHT_CSS, PAGE_BREAK_AND_CENTER_CSS, markdown_to_html,
    markdown_to_html_fragment, wrap_in_html_doc,
};
use crate::doc::generate_pdf::generate_pdf;
use crate::doc::parser::exclude::FilterTarget;
use crate::doc::parser::slides::parse_slides_from_ast;
use crate::doc::parser::{ast_to_markdown, parse_code_blocks_from_ast, parse_from_string};
use crate::execution::ExecutionOutput;
use crate::execution::write_code_to_file;
use comrak::nodes::{Ast, AstNode, LineColumn, NodeHtmlBlock, NodeValue};
use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{Arena, ComrakOptions, Plugins, parse_document};
pub use error::DocError;
use markdown::mdast::Node;
pub use parser::ParserError;
pub use parser::code_block::CodeBlock;
use parser::exclude::exclude_from_ast;
pub use parser::slides::SlideByIndex;
use parser::slides::parse_slides_index_from_ast;
use regex::Regex;
use serde::Serialize;
use std::cell::RefCell;
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

fn parse_metadata(metadata: &str) -> (Option<String>, Option<String>) {
    // Regex to capture `use=[...]`
    let use_re =
        Regex::new(crate::doc::parser::code_block::USE_REGEX).expect("Failed to compile USE_REGEX");

    // // Extract imports
    // let imports: Vec<String> = use_re
    //     .captures(metadata)
    //     .map(|caps| {
    //         caps[1]
    //             .split(',')
    //             .map(|s| s.trim().to_string())
    //             .filter(|s| !s.is_empty())
    //             .collect()
    //     })
    //     .unwrap_or_default();

    // Remove the `use=[...]` part to get the block tag
    let metadata_without_use = use_re.replace(metadata, "");

    // Take the first word that is not part of `use=` as the tag
    let lang = metadata_without_use
        .split_whitespace()
        .next()
        .map(|s| s.to_string());

    let tag = metadata_without_use
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string());

    (lang, tag)
}

pub fn make_html_node<'a>(arena: &'a Arena<AstNode<'a>>, raw_html: &str) -> &'a AstNode<'a> {
    let html_ast = Ast::new(
        NodeValue::HtmlBlock(NodeHtmlBlock {
            block_type: 0,
            literal: raw_html.to_string() + "\n",
        }),
        LineColumn::from((0, 0)),
    );
    arena.alloc(AstNode::new(RefCell::new(html_ast)))
}

pub fn insert_html_before<'a>(
    arena: &'a Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
    raw_html: &str,
) -> &'a AstNode<'a> {
    let html_ast_node = make_html_node(arena, raw_html);
    node.insert_before(html_ast_node);
    html_ast_node
}

pub fn insert_html_after<'a>(
    arena: &'a Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
    raw_html: &str,
) -> &'a AstNode<'a> {
    let html_ast_node = make_html_node(arena, raw_html);
    node.insert_after(html_ast_node);
    html_ast_node
}

pub fn wrap<'a>(
    arena: &'a Arena<AstNode<'a>>,
    first_node: &'a AstNode<'a>,
    last_node: &'a AstNode<'a>,
    opening_html: &str,
    closing_html: &str,
) -> &'a AstNode<'a> {
    insert_html_before(arena, first_node, opening_html);
    insert_html_after(arena, last_node, closing_html)
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
    pub fn ast_format_output(
        &self,
        _lang: &Option<String>,
        tag: &Option<String>,
    ) -> (String, String) {
        let main_opening_html = r#"<div class="code-execution-pair">"#.to_string();
        let main_closing_html = r#"</div>"#.to_string();
        let block_tag_html = tag
            .as_ref()
            .map(|_tag| format!(r#"<div class="block-tag">{_tag}</div>"#));
        let mut opening_html = main_opening_html;
        if block_tag_html.is_some() {
            opening_html.push_str(block_tag_html.as_deref().unwrap());
        }
        (opening_html, main_closing_html)
    }

    pub fn ast_format_single_code_block(
        &self,
        _lang: &Option<String>,
        tag: &Option<String>,
    ) -> (String, String) {
        let main_opening_html = r#"<div class="code-block">"#.to_string();
        let main_closing_html = r#"</div>"#.to_string();
        let block_tag_html = tag.as_ref().map(|_tag| {
            format!(r#"<div class="block-header"><span class="block-tag">{_tag}</span></div>"#)
        });
        let mut opening_html = main_opening_html;
        if block_tag_html.is_some() {
            opening_html.push_str(block_tag_html.as_deref().unwrap());
        }
        (opening_html, main_closing_html)
    }

    pub fn format_output_ast<'a>(
        &self,
        arena: &'a Arena<AstNode<'a>>,
        lang: Option<String>,
        tag: Option<String>,
        code_block: &'a AstNode<'a>,
        output_block: &'a AstNode<'a>,
    ) -> Option<&'a comrak::arena_tree::Node<'a, RefCell<Ast>>> {
        // Takes a code block AstNode, and its corresponding output AstNode, and formats them together
        // by wrapping them in a div with appropriate classes and adding an "OUTPUT" header.
        // Returns the next sibling of the closing div for further traversal.
        let (opening_html, closing_html) = self.ast_format_output(&lang, &tag);
        insert_html_before(
            arena,
            output_block,
            r#"<div class="output-header">OUTPUT</div>"#,
        );
        let closing_div = wrap(
            arena,
            code_block,
            output_block,
            &opening_html,
            &closing_html,
        );
        closing_div.next_sibling()
    }

    pub fn format_code_blocks<'a>(&self, root: &'a AstNode<'a>, arena: &'a Arena<AstNode<'a>>) {
        use comrak::nodes::NodeValue;

        // format code blocks | output block pairs
        let mut node = root.first_child();
        while let Some(current_node) = node {
            if let Some(next_node) = current_node.next_sibling() {
                // Check if current_node is a code block and next_node is an output block
                if let NodeValue::CodeBlock(block) = &current_node.data.borrow().value {
                    let (lang, tag) = parse_metadata(block.info.as_ref());
                    if block.info != "output" {
                        if let NodeValue::CodeBlock(next_block) = &next_node.data.borrow().value {
                            if next_block.info == "output" {
                                // We have a code block followed by the corresponding output block
                                // Add HTML formatting to them, so they are grouped together
                                // we add the code block language and tag, etc.
                                node = self.format_output_ast(
                                    arena,
                                    lang,
                                    tag,
                                    current_node,
                                    next_node,
                                );
                                continue;
                            }
                        }
                        // A standalone code block with no output block
                        let (opening_html, closing_html) =
                            self.ast_format_single_code_block(&lang, &tag);
                        let closing_div = wrap(
                            arena,
                            current_node,
                            current_node,
                            &opening_html,
                            &closing_html,
                        );
                        node = closing_div.next_sibling();
                        continue;
                    }
                }
            }

            node = current_node.next_sibling();
        }
    }

    pub fn generate_html(&self) -> Result<String, DocError> {
        let markdown_with_exclusions = self.filter_content_for_doc()?;

        let arena = Arena::new();
        let root = parse_document(&arena, &markdown_with_exclusions, &ComrakOptions::default());

        self.format_code_blocks(root, &arena); // add block names, etc.

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

        Ok(wrap_in_html_doc(
            &inner_html,
            "Some title",
            &[CUSTOM_CSS.to_string()],
        ))

        // Ok(format!(
        //     r#"
        //     <!DOCTYPE html>
        //     <style>
        //     {}
        //     {}
        //     </style><div class="markdown-body">{}</div></body></html>"#,
        //     custom_css, GITHUB_MARKDOWN_LIGHT_CSS, inner_html
        // ))
        // Ok(markdown_to_html(&markdown_with_exclusions))
    }

    pub fn generate_doc_pdf(&self, output_file_path: &str) -> Result<(), DocError> {
        let markdown_with_exclusions = self.filter_content_for_doc()?;
        let html_with_exclusions = markdown_to_html(&markdown_with_exclusions);
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
