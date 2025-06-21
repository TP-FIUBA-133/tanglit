use std::collections::HashMap;

use crate::{errors::TangleError, parser::code_block::CodeBlock};

pub fn tangle_blocks(blocks: Vec<CodeBlock>) -> String {
    let mut tangle = String::new();
    for block in blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
    }
    tangle
}

pub fn tangle_block(
    target_block: &str,
    blocks: HashMap<String, CodeBlock>,
) -> Result<String, TangleError> {
    // Get target_code_block
    let target_code_block = blocks
        .get(target_block)
        .ok_or(TangleError::BlockNotFound(target_block.into()))?;

    // Get imported blocks
    let imported_blocks: Vec<CodeBlock> = target_code_block
        .imports
        .iter()
        .map(|import| {
            blocks
                .get(import)
                .cloned()
                .ok_or(TangleError::BlockNotFound(import.clone()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Tangle imports
    let mut tangle = String::new();
    for block in imported_blocks {
        tangle.push_str(&block.code);
        tangle.push('\n');
        // TODO: lines between blocks should be configurable
        tangle.push('\n');
    }

    add_main_code_block(target_code_block, &mut tangle);

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
