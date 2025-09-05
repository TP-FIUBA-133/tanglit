mod error;
mod generate_pdf;
mod parser;
mod tangle;

use crate::doc::generate_pdf::generate_pdf;
use crate::doc::parser::slides::parse_slides_from_ast;
use crate::doc::parser::{
    ast_to_markdown, markdown_to_html, parse_code_blocks_from_ast, parse_from_string,
};
pub use error::DocError;
use markdown::mdast::Node;
pub use parser::ParserError;
pub use parser::code_block::CodeBlock;
use parser::exclude::exclude_from_ast;
pub use parser::slides::SlideByIndex;
use parser::slides::parse_slides_index_from_ast;
use std::collections::HashMap;
use std::fs;
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

    pub fn parse_slides_index(&self) -> Vec<SlideByIndex> {
        parse_slides_index_from_ast(&self.ast, &self.raw_markdown)
    }

    pub fn generate_slides(&self) -> Result<(), DocError> {
        let slides = parse_slides_from_ast(&self.ast, &self.raw_markdown);

        for (i, slide) in slides.iter().enumerate() {
            let slide_html = slide.get_html()?;
            fs::write(format!("slide_{}.html", i), slide_html)?;
        }

        Ok(())
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
