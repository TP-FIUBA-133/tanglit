use crate::{errors::TangleError, parser::code_block::CodeBlock};
use regex::Regex;
use std::collections::{HashMap, HashSet};

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
    mut blocks: HashMap<String, CodeBlock>,
) -> Result<String, TangleError> {
    // Get target_code_block
    let mut target_code_block = blocks
        .remove(target_block)
        .ok_or(TangleError::BlockNotFound(target_block.into()))?;

    resolve_macros(&mut target_code_block, &blocks)?;

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

    add_main_code_block(&target_code_block, &mut tangle);

    Ok(tangle)
}

fn resolve_macros(
    code_block: &mut CodeBlock,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<(), TangleError> {
    let re = Regex::new(r"@\[([a-zA-Z0-9_]+)\]").unwrap();

    // Collect all referenced block names
    let referenced_blocks: HashSet<_> = re
        .captures_iter(&code_block.code)
        .map(|caps| caps[1].to_string())
        .collect();

    // Check for missing blocks
    let missing: Vec<_> = referenced_blocks
        .iter()
        .filter(|tag| !blocks.contains_key(*tag))
        .collect();

    if let Some(missing) = missing.first() {
        return Err(TangleError::BlockNotFound(missing.to_string()));
    }

    // Replace imports with block content
    let code_block_with_macros = re.replace_all(&code_block.code, |caps: &regex::Captures| {
        blocks
            .get(&caps[1])
            .unwrap() // It is safe to unwrap here because we checked for missing blocks above
            .code
            .clone()
    });

    code_block.code = code_block_with_macros.into_owned();

    Ok(())
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
