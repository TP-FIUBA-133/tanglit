use crate::{errors::TangleError, parser::code_block::CodeBlock};

pub fn tangle_blocks(blocks: Vec<CodeBlock>) -> String {
    let mut tangle = String::new();
    for block in blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
    }
    tangle
}

pub fn tangle_block(block: String, blocks: &[CodeBlock]) -> Result<String, TangleError> {
    // Search block
    let code_block = blocks
        .iter()
        .find(|b| b.tag.clone().unwrap_or_default() == block)
        .ok_or_else(|| TangleError::BlockNotFound(block))?;

    // Search imported blocks
    let imported_blocks: Vec<CodeBlock> = blocks
        .iter()
        .filter(|b| {
            code_block
                .imports
                .contains(&b.tag.clone().unwrap_or_default())
        })
        .cloned()
        .collect();

    // Tangle imports
    let mut tangle = String::new();
    for block in imported_blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
    }

    tangle.push('\n');

    // Add the main block code
    tangle.push_str("int main() {\n"); // TODO: this should be configurable
    tangle.push_str(&code_block.code); // TODO: handle indentation
    tangle.push('\n');
    tangle.push_str("\treturn 0;");
    tangle.push_str("\n}\n");

    Ok(tangle)
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
