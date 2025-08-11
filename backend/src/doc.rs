mod error;
mod generate_pdf;
mod parser;
mod tangle;

pub use crate::doc::error::DocError;
use crate::doc::generate_pdf::generate_pdf;
use crate::doc::parser::{
    ast_to_markdown, markdown_to_html, parse_code_blocks_from_ast, parse_from_string,
};
pub use crate::doc::tangle::TangleError;
use markdown::mdast::Node;
pub use parser::ParserError;
pub use parser::code_block::{CodeBlock, Language};
use parser::exclude::exclude_from_ast;
pub use parser::slides::Slide;
use parser::slides::parse_slides_from_ast;
use std::collections::HashMap;
use tangle::tangle_block;

pub struct TanglitDoc {
    raw_markdown: String,
    ast: Node,
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

    pub fn parse_blocks(&self) -> Result<HashMap<String, CodeBlock>, DocError> {
        Ok(parse_code_blocks_from_ast(&self.ast)?)
    }

    pub fn parse_slides(&self) -> Vec<Slide> {
        parse_slides_from_ast(&self.ast, &self.raw_markdown)
    }

    pub fn exclude(&self) -> Result<String, DocError> {
        let ast_with_exclusions = exclude_from_ast(&self.ast);
        Ok(ast_to_markdown(&ast_with_exclusions)?)
    }

    pub fn tangle_block(
        &self,
        target_block: &str,
        add_wrapper: bool,
    ) -> Result<(String, Language), DocError> {
        let blocks = self.parse_blocks()?;
        Ok(tangle_block(target_block, blocks, add_wrapper)?)
    }

    pub fn generate_pdf(&self, output_file_path: &str) -> Result<(), DocError> {
        let markdown_with_exclusions = self.exclude()?;
        let html_with_exclusions = markdown_to_html(&markdown_with_exclusions);
        generate_pdf(&html_with_exclusions, output_file_path);
        Ok(())
    }
}
