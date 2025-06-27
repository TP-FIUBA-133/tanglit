use crate::{errors::TangleError, parser::code_block::CodeBlock};
use std::process;

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

fn find_block_by_tag<'a>(blocks: &'a [CodeBlock], tag: &str) -> Result<&'a CodeBlock, TangleError> {
    blocks
        .iter()
        .find(|b| b.tag.clone().unwrap_or_default() == tag)
        .ok_or(TangleError::BlockNotFound(tag.into()))
}

pub fn tangle_execute_block(
    block: &str,
    blocks: &[CodeBlock],
) -> Result<process::Output, TangleError> {
    // Search block
    let code_block = find_block_by_tag(blocks, block)?;

    match code_block.language {
        crate::parser::code_block::Language::Python => {
            let code = add_imports(code_block, blocks, code_block.code.clone());
            let file = crate::file_generation::write_file(code, block, "py");
            Ok(crate::file_generation::execute_python_file(file))
        }
        crate::parser::code_block::Language::C => {
            let mut tangle = String::new();
            add_c_wrapper(code_block, &mut tangle);
            let code = add_imports(code_block, blocks, tangle);
            let file = crate::file_generation::write_file(code, block, "c");
            Ok(crate::file_generation::execute_c_file(file))
        }
        crate::parser::code_block::Language::Rust => todo!(),
        _ => Err(TangleError::LanguageNotSupported("some lang".into())),
    }
}

pub fn tangle_block(block: &str, blocks: &[CodeBlock]) -> Result<String, TangleError> {
    // Search block
    let code_block = find_block_by_tag(blocks, block)?;
    let tangle = String::new();

    let tangle = add_imports(code_block, blocks, tangle);

    Ok(tangle)
}

fn add_c_wrapper(code_block: &CodeBlock, tangle: &mut String) {
    tangle.push_str("int main() {\n");
    tangle.push_str(&code_block.code);
    tangle.push('\n');
    tangle.push_str("    return 0;");
    tangle.push_str("\n}\n");
}

#[cfg(test)]
mod tests {
    use crate::parser::code_block::Language;

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

    #[test]
    fn test_tangle_execute_block_python() {
        let blocks = vec![CodeBlock::new(
            Language::Python,
            "print('Hello, world!')".to_string(),
            Some("print_block".to_string()),
            vec![],
        )];
        let tangle = tangle_execute_block("print_block", &blocks);
        assert!(tangle.is_ok());
        let tangle = tangle.unwrap();
        assert_eq!(String::from_utf8_lossy(&tangle.stdout), "Hello, world!\n");
    }

    #[test]
    fn test_tangle_execute_block_c() {
        let blocks = vec![
            CodeBlock::new(
                Language::C,
                "#include <stdio.h>\n".to_string(),
                Some("imports".to_string()),
                vec![],
            ),
            CodeBlock::new(
                Language::C,
                "printf(\"Hello, world!\");".to_string(),
                Some("print_block".to_string()),
                vec!["imports".to_string()],
            ),
        ];
        let tangle = tangle_execute_block("print_block", &blocks);
        assert!(tangle.is_ok());
        let tangle = tangle.unwrap();
        assert_eq!(String::from_utf8_lossy(&tangle.stdout), "Hello, world!");
    }
}
