//! # Frontend API Module
//!
//! This module provides the main interface for interacting with Tanglit's core features from the frontend.
//! It exposes functions for parsing code blocks and slides from markdown, executing code blocks, and
//! excluding content from markdown. All other internal logic is encapsulated.
//!
//! ## Public Functions
//!
//! - `parse_blocks_and_slides(raw_markdown: &str) -> Result<TanglitInfo, String>`  
//!   Parses the provided markdown string, extracting code blocks and slide information.
//!
//! - `execute_block(raw_markdown: &str, block_tag: &str) -> Result<ExecutionResult, String>`  
//!   Executes the code block identified by `block_tag` within the provided markdown, returning the result.
//!
//! - `exclude(raw_markdown: &str) -> Result<String, String>`  
//!   Processes the markdown to exclude the marked content, returning the final markdown.
//!
//! ## Data Structures
//!
//! - `TanglitInfo`  
//!   Contains the parsed code blocks and slides from the markdown.
//!
//! ## Usage
//!
//! These functions are intended to be called by the frontend to interact with Tanglit's parsing and execution logic.

use crate::execution::{execute, ExecutionResult};
use crate::parser;
use crate::parser::ast_to_markdown;
use crate::parser::code_block::CodeBlock;
use serde::Serialize;

#[derive(Serialize)]
pub struct TanglitInfo {
    pub blocks: Vec<CodeBlock>,
    pub slides: Vec<parser::slides::Slide>,
}

pub fn parse_blocks_and_slides(raw_markdown: &str) -> Result<TanglitInfo, String> {
    let ast = parser::parse_from_string(raw_markdown).map_err(|e| e.to_string())?;
    let blocks = parser::parse_code_blocks_from_ast(&ast)
        .map_err(|e| e.to_string())?
        .iter()
        .map(|(_, block)| block.clone())
        .collect();
    let slides = parser::slides::parse_slides_from_ast(&ast, raw_markdown);
    Ok(TanglitInfo { blocks, slides })
}

pub fn execute_block(raw_markdown: &str, block_tag: &str) -> Result<ExecutionResult, String> {
    let ast = parser::parse_from_string(raw_markdown).map_err(|e| e.to_string())?;
    let blocks = parser::parse_code_blocks_from_ast(&ast).map_err(|e| e.to_string())?;
    execute(blocks, block_tag)
}

pub fn exclude(raw_markdown: &str) -> Result<String, String> {
    let ast = parser::parse_from_string(raw_markdown).map_err(|e| e.to_string())?;
    let ast_with_exclusions = parser::exclude::exclude_from_ast(&ast);
    ast_to_markdown(&ast_with_exclusions)
}
