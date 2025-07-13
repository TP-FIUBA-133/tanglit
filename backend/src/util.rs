use crate::errors::ParserError;
use crate::parser::code_block::CodeBlock;
use crate::parser::{parse_code_blocks_from_ast, parse_from_file};
use std::collections::HashMap;

pub fn read_file_and_parse_blocks(
    file_path: &str,
) -> Result<HashMap<String, CodeBlock>, ParserError> {
    let mdast = parse_from_file(file_path)?;
    parse_code_blocks_from_ast(&mdast)
}
