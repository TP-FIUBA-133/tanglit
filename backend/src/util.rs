use crate::errors::ParserError;
use crate::parser::code_block::CodeBlock;
use crate::parser::{parse_code_blocks_from_ast, parse_from_file};
use markdown::mdast::Node;
use std::collections::HashMap;

pub fn read_file_and_parse_blocks(
    file_path: &str,
) -> Result<HashMap<String, CodeBlock>, ParserError> {
    let mdast = parse_from_file(file_path)?;
    parse_code_blocks_from_ast(&mdast)
}

pub fn ast_to_markdown(ast: &Node) -> String {
    mdast_util_to_markdown::to_markdown(&ast).expect("Failed to convert to markdown")
}
