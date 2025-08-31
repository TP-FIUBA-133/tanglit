mod error;
mod generate_pdf;
mod macro_dependency;
mod parser;
mod tangle;

use crate::doc::generate_pdf::generate_pdf;
use crate::doc::parser::{
    ast_to_markdown, markdown_to_html, parse_code_blocks_from_ast, parse_from_string,
};
pub use error::DocError;
use markdown::mdast::Node;
pub use parser::ParserError;
pub use parser::code_block::CodeBlock;
use parser::exclude::exclude_from_ast;
pub use parser::slides::Slide;
use parser::slides::parse_slides_from_ast;
use std::collections::HashMap;
pub use tangle::CodeBlocks;
pub use tangle::TangleError;

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

    pub fn parse_slides(&self) -> Vec<Slide> {
        parse_slides_from_ast(&self.ast, &self.raw_markdown)
    }

    pub fn exclude(&self) -> Result<String, DocError> {
        let ast_with_exclusions = exclude_from_ast(&self.ast);
        Ok(ast_to_markdown(&ast_with_exclusions)?)
    }

    pub fn get_code_blocks(&self) -> Result<CodeBlocks, DocError> {
        let blocks = self.parse_blocks()?;
        Ok(CodeBlocks::from_codeblocks(blocks))
    }

    pub fn generate_pdf(&self, output_file_path: &str) -> Result<(), DocError> {
        let markdown_with_exclusions = self.exclude()?;
        let html_with_exclusions = markdown_to_html(&markdown_with_exclusions);
        generate_pdf(&html_with_exclusions, output_file_path)?;
        Ok(())
    }
}
