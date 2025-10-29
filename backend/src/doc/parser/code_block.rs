use std::collections::HashMap;

use super::ParserError;
use markdown::mdast::Code;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

// Regex to capture `use=[...]`
const USE_REGEX: &str = r"use=\[([^\]]*)\]";
static USE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(USE_REGEX).expect("Failed to compile USE_REGEX"));

// Regex to capture `export=`
const EXPORT_REGEX: &str = r"export\s*=\s*([^\s]+)";
static EXPORT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(EXPORT_REGEX).expect("Failed to compile EXPORT_REGEX"));

// Regex to capture `args=[...]`
const ARGS_REGEX: &str = r"args=\[(.*)\]";
static ARGS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(ARGS_REGEX).expect("Failed to compile ARGS_REGEX"));

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
    pub tag: String,
    pub imports: Vec<String>,
    pub export: Option<String>,
    pub args: HashMap<String, String>,
    pub start_line: usize,
    pub end_line: usize,
}

impl CodeBlock {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        language: Option<String>,
        code: String,
        tag: String,
        imports: Vec<String>,
        export: Option<String>,
        args: HashMap<String, String>,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        Self {
            language,
            code,
            tag,
            imports,
            export,
            args,
            start_line,
            end_line,
        }
    }

    pub fn new_with_code(code: String) -> Self {
        Self::new(
            None,
            code,
            "".to_string(),
            Vec::new(),
            None,
            HashMap::new(),
            0,
            0,
        )
    }

    /// Creates a CodeBlock from a Code node, extracting the language, code, tag, and imports.
    /// If the tag is not specified in the code block, it defaults to the line number of the code block.
    pub fn from_code_node(code_block: Code) -> Result<Self, ParserError> {
        let language = code_block.lang;
        let (tag, imports, export, args) =
            Self::parse_metadata(code_block.meta.unwrap_or_default().as_str());
        let position = code_block.position.as_ref();
        let start_line = position
            .ok_or_else(|| ParserError::CodeBlockError("Block position not found".to_string()))?
            .start
            .line;
        let end_line = position
            .ok_or_else(|| ParserError::CodeBlockError("Block position not found".to_string()))?
            .end
            .line;
        let tag = match tag {
            Some(t) => t,
            None => start_line.to_string(),
        };

        Ok(Self::new(
            language,
            code_block.value,
            tag,
            imports,
            export,
            args,
            start_line,
            end_line,
        ))
    }

    fn parse_metadata(
        metadata: &str,
    ) -> (
        Option<String>,
        Vec<String>,
        Option<String>,
        HashMap<String, String>,
    ) {
        // Extract imports
        let imports: Vec<String> = USE_RE
            .captures(metadata)
            .map(|caps| {
                caps[1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        // Extract export
        let export = EXPORT_RE.captures(metadata).map(|caps| caps[1].to_string());

        // Extract args
        let args: HashMap<String, String> = ARGS_RE
            .captures(metadata)
            .map(|caps| {
                let args_content = &caps[1];
                args_content
                    .split(';')
                    .filter_map(|s| {
                        let mut parts = s.splitn(2, '=').map(|s| s.trim());
                        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                            Some((k.to_string(), v.to_string()))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Remove the `use=[...]`, `export=` and `args=[...]` part to get the block tag
        let metadata_without_use = USE_RE.replace(metadata, "");
        let metadata_without_export = EXPORT_RE.replace(&metadata_without_use, "");
        let metadata_clean = ARGS_RE.replace(&metadata_without_export, "");

        // Take the first word that is not part of `use=` and `export=` as the tag
        let tag = metadata_clean
            .split_whitespace()
            .next()
            .map(|s| s.to_string());

        (tag, imports, export, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_with_use() {
        let metadata = "use=[block1,block2] tag1";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag1".to_string()));
        assert_eq!(imports, vec!["block1".to_string(), "block2".to_string()]);
        assert!(export.is_none());
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_metadata_with_only_tag() {
        let metadata = "tag2";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag2".to_string()));
        assert!(imports.is_empty());
        assert!(export.is_none());
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_metadata_empty() {
        let metadata = "";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert!(tag.is_none());
        assert!(imports.is_empty());
        assert!(export.is_none());
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_metadata_with_export_and_tag() {
        let metadata = "export=main.c tag3";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag3".to_string()));
        assert!(imports.is_empty());
        assert_eq!(export, Some("main.c".to_string()));
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_metadata_with_use_and_export() {
        let metadata = "use=[block1, block2] export=main.c";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert!(tag.is_none());
        assert_eq!(imports, vec!["block1".to_string(), "block2".to_string()]);
        assert_eq!(export, Some("main.c".to_string()));
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_metadata_with_use_export_and_tag() {
        let metadata = "use=[block1] export=main.c tag4";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag4".to_string()));
        assert_eq!(imports, vec!["block1".to_string()]);
        assert_eq!(export, Some("main.c".to_string()));
        assert!(args.is_empty());
    }

    #[test]
    fn test_parse_metadata_with_args() {
        let metadata = "use=[block1] export=main.c args=[arg1=1; arg2=2] tag5";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag5".to_string()));
        assert_eq!(imports, vec!["block1".to_string()]);
        assert_eq!(export, Some("main.c".to_string()));
        let mut expected_args = HashMap::new();
        expected_args.insert("arg1".to_string(), "1".to_string());
        expected_args.insert("arg2".to_string(), "2".to_string());
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_metadata_with_only_args() {
        let metadata = "args=[arg1=1; arg2=2]";
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert!(tag.is_none());
        assert!(imports.is_empty());
        assert!(export.is_none());
        let mut expected_args = HashMap::new();
        expected_args.insert("arg1".to_string(), "1".to_string());
        expected_args.insert("arg2".to_string(), "2".to_string());
        assert_eq!(args, expected_args);
    }

    #[test]
    fn test_parse_metadata_with_quoted_args() {
        let metadata = r#"args=[arg1="1"; arg2="2"] tag5"#;
        let (tag, imports, export, args) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag5".to_string()));
        assert!(imports.is_empty());
        assert_eq!(export, None);
        let mut expected_args = HashMap::new();
        expected_args.insert("arg1".to_string(), "\"1\"".to_string());
        expected_args.insert("arg2".to_string(), "\"2\"".to_string());
        assert_eq!(args, expected_args);
    }
}
