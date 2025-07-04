use crate::{
    errors::TangleError,
    parser::code_block::{CodeBlock, Language},
};
use std::collections::HashMap;

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
    add_wrapper: bool,
) -> Result<(String, Language), TangleError> {
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

    if add_wrapper {
        if target_code_block.language == crate::parser::code_block::Language::C {
            add_main_code_block(target_code_block, &mut tangle);
        } else {
            tangle.push_str(&target_code_block.code);
        }
    } else {
        tangle.push_str(&target_code_block.code);
    }

    Ok((tangle, target_code_block.language.clone()))
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

    /// Tests that both imports and the wrapper is added to C code when wrapper is requested
    #[test]
    fn test_tangle_block_wrapper() {
        let blocks = HashMap::<String, CodeBlock>::from([
            (
                "imports".to_string(),
                CodeBlock::new(
                    Language::C,
                    "#include <stdio.h>\n".to_string(),
                    "imports".to_string(),
                    vec![],
                ),
            ),
            (
                "main_block".to_string(),
                CodeBlock::new(
                    Language::C,
                    "printf(\"Hello, world!\");".to_string(),
                    "main_block".to_string(),
                    vec!["imports".to_string()],
                ),
            ),
        ]);
        let tangle = tangle_block("main_block", blocks, true);
        assert!(tangle.is_ok());
        let (tangled_code, _) = tangle.unwrap();
        assert_eq!(
            tangled_code,
            r#"#include <stdio.h>


int main() {
printf("Hello, world!");
    return 0;
}
"#
        );
    }

    /// Tests that imports are added to C code but no wrapper is added when wrapper is not requested
    #[test]
    fn test_tangle_block_no_wrapper() {
        let blocks = HashMap::<String, CodeBlock>::from([
            (
                "imports".to_string(),
                CodeBlock::new(
                    Language::C,
                    "#include <stdio.h>\n".to_string(),
                    "imports".to_string(),
                    vec![],
                ),
            ),
            (
                "main_block".to_string(),
                CodeBlock::new(
                    Language::C,
                    r#"int main() {
printf("Hello, world!");
    return 0;
}
"#
                    .to_string(),
                    "main_block".to_string(),
                    vec!["imports".to_string()],
                ),
            ),
        ]);
        let tangle = tangle_block("main_block", blocks, false);
        assert!(tangle.is_ok());
        let (tangled_code, _) = tangle.unwrap();
        assert_eq!(
            tangled_code,
            r#"#include <stdio.h>


int main() {
printf("Hello, world!");
    return 0;
}
"#
        );
    }

    /// Tests that Python code is returned without a wrapper
    /// since Python does not require a main function.
    /// The wrapper is only added for C code.
    #[test]
    fn test_tangle_block_wrapper_python() {
        let blocks = HashMap::<String, CodeBlock>::from([(
            "main".to_string(),
            CodeBlock::new(
                Language::Python,
                "print('monty python')\n".to_string(),
                "main".to_string(),
                vec![],
            ),
        )]);
        let tangle = tangle_block("main", blocks, true);
        assert!(tangle.is_ok());
        let (tangled_code, _) = tangle.unwrap();
        assert_eq!(tangled_code, "print('monty python')\n");
    }
}
