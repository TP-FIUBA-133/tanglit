use std::collections::HashMap;
use crate::errors::ParserError;
use crate::parser::code_block::CodeBlock;
use crate::parser::{parse_code_blocks_from_ast, parse_from_file};

pub fn parse_code_blocks_from_file(
    file_path: &str,
) -> Result<HashMap<String, CodeBlock>, ParserError> {
    let mdast = parse_from_file(file_path)?;
    parse_code_blocks_from_ast(&mdast)
}
