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

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
    pub tag: String,
    pub imports: Vec<String>,
    pub export: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
}

impl CodeBlock {
    pub fn new(
        language: Option<String>,
        code: String,
        tag: String,
        imports: Vec<String>,
        export: Option<String>,
        start_line: usize,
        end_line: usize,
    ) -> Self {
        Self {
            language,
            code,
            tag,
            imports,
            export,
            start_line,
            end_line,
        }
    }

    pub fn new_with_code(code: String) -> Self {
        Self::new(None, code, "".to_string(), Vec::new(), None, 0, 0)
    }

    /// Creates a CodeBlock from a Code node, extracting the language, code, tag, and imports.
    /// If the tag is not specified in the code block, it defaults to the line number of the code block.
    pub fn from_code_node(code_block: Code) -> Result<Self, ParserError> {
        let language = code_block.lang;
        let (tag, imports, export) =
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
            start_line,
            end_line,
        ))
    }

    fn parse_metadata(metadata: &str) -> (Option<String>, Vec<String>, Option<String>) {
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

        // Remove the `use=[...]` and `export=` part to get the block tag
        let metadata_without_use = USE_RE.replace(metadata, "");
        let metadata_clean = EXPORT_RE.replace(&metadata_without_use, "");

        // Take the first word that is not part of `use=` and `export=` as the tag
        let tag = metadata_clean
            .split_whitespace()
            .next()
            .map(|s| s.to_string());

        (tag, imports, export)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_with_use() {
        let metadata = "use=[block1,block2] tag1";
        let (tag, imports, export) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag1".to_string()));
        assert_eq!(imports, vec!["block1".to_string(), "block2".to_string()]);
        assert!(export.is_none());
    }

    #[test]
    fn test_parse_metadata_with_only_tag() {
        let metadata = "tag2";
        let (tag, imports, export) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag2".to_string()));
        assert!(imports.is_empty());
        assert!(export.is_none());
    }

    #[test]
    fn test_parse_metadata_empty() {
        let metadata = "";
        let (tag, imports, export) = CodeBlock::parse_metadata(metadata);
        assert!(tag.is_none());
        assert!(imports.is_empty());
        assert!(export.is_none());
    }

    #[test]
    fn test_parse_metadata_with_export_and_tag() {
        let metadata = "export=main.c tag3";
        let (tag, imports, export) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag3".to_string()));
        assert!(imports.is_empty());
        assert_eq!(export, Some("main.c".to_string()));
    }

    #[test]
    fn test_parse_metadata_with_use_and_export() {
        let metadata = "use=[block1, block2] export=main.c";
        let (tag, imports, export) = CodeBlock::parse_metadata(metadata);
        assert!(tag.is_none());
        assert_eq!(imports, vec!["block1".to_string(), "block2".to_string()]);
        assert_eq!(export, Some("main.c".to_string()));
    }

    #[test]
    fn test_parse_metadata_with_use_export_and_tag() {
        let metadata = "use=[block1] export=main.c tag4";
        let (tag, imports, export) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag4".to_string()));
        assert_eq!(imports, vec!["block1".to_string()]);
        assert_eq!(export, Some("main.c".to_string()));
    }
}
