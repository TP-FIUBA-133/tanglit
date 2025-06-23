use crate::{errors::TangleError, parser::code_block::CodeBlock};

pub fn tangle_blocks(blocks: Vec<CodeBlock>) -> String {
    let mut tangle = String::new();
    for block in blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
    }
    tangle
}

fn add_imports(block: &CodeBlock, blocks: &[CodeBlock], source_code: String) -> String {
    // Search imported blocks
    let imported_blocks: Vec<CodeBlock> = blocks
        .iter()
        .filter(|b| block.imports.contains(&b.tag.clone().unwrap_or_default()))
        .cloned()
        .collect();

    // Tangle imports
    let mut accum_block = String::new();
    for block in imported_blocks {
        accum_block.push_str(&block.code);
        accum_block.push('\n');
        // TODO: lines between blocks should be configurable
        accum_block.push('\n');
    }
    accum_block.push_str(&source_code);
    accum_block
}

fn find_block_by_tag<'a, 'b>(
    blocks: &'a [CodeBlock],
    tag: &'b str,
) -> Result<&'a CodeBlock, TangleError> {
    blocks
        .iter()
        .find(|b| b.tag.clone().unwrap_or_default() == tag)
        .ok_or(TangleError::BlockNotFound(tag.into()))
}

pub fn tangle_block(block: &str, blocks: &[CodeBlock]) -> Result<String, TangleError> {
    // Search block
    let code_block = find_block_by_tag(blocks, block)?;
    let mut tangle = String::new();

    add_main_code_block(code_block, &mut tangle);
    let tangle = add_imports(code_block, blocks, tangle);

    Ok(tangle)
}

// TODO: this should use a template depending on the language
// TODO: handle indentation
pub fn add_main_code_block(code_block: &CodeBlock, tangle: &mut String) {
    tangle.push_str("int main() {\n");
    tangle.push_str(&code_block.code);
    tangle.push('\n');
    tangle.push_str("    return 0;");
    tangle.push_str("\n}\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tangle_blocks() {
        let blocks = vec![
            CodeBlock::new_with_code("print('Hello, world!')".to_string()),
            CodeBlock::new_with_code("console.log('Hello, world!')".to_string()),
        ];
        let tangle = tangle_blocks(blocks);
        assert_eq!(
            tangle,
            "print('Hello, world!')\nconsole.log('Hello, world!')\n"
        );
    }

    #[test]
    fn test_tangle_blocks_single() {
        let blocks = vec![CodeBlock::new_with_code(
            "print('Hello, world!')".to_string(),
        )];
        let tangle = tangle_blocks(blocks);
        assert_eq!(tangle, "print('Hello, world!')\n");
    }
}
