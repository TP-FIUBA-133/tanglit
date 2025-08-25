use super::ParserError;
use markdown::mdast::Code;
use regex::Regex;
use serde::Serialize;

const USE_REGEX: &str = r"use=\[([^\]]*)\]";

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
    pub tag: String,
    pub imports: Vec<String>,
    pub start_line: usize,
}

impl CodeBlock {
    pub fn new(
        language: Option<String>,
        code: String,
        tag: String,
        imports: Vec<String>,
        start_line: usize,
    ) -> Self {
        Self {
            language,
            code,
            tag,
            imports,
            start_line,
        }
    }

    pub fn new_with_code(code: String) -> Self {
        Self::new(None, code, "".to_string(), Vec::new(), 0)
    }

    /// Creates a CodeBlock from a Code node, extracting the language, code, tag, and imports.
    /// If the tag is not specified in the code block, it defaults to the line number of the code block.
    pub fn from_code_node(code_block: Code) -> Result<Self, ParserError> {
        let language = code_block.lang;
        let (tag, imports) = Self::parse_metadata(code_block.meta.unwrap_or_default().as_str());
        let start_line = code_block
            .position
            .ok_or_else(|| ParserError::CodeBlockError("Block position not found".to_string()))?
            .start
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
            start_line,
        ))
    }

    fn parse_metadata(metadata: &str) -> (Option<String>, Vec<String>) {
        // Regex to capture `use=[...]`
        let use_re = Regex::new(USE_REGEX).expect("Failed to compile USE_REGEX");

        // Extract imports
        let imports: Vec<String> = use_re
            .captures(metadata)
            .map(|caps| {
                caps[1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        // Remove the `use=[...]` part to get the block tag
        let metadata_without_use = use_re.replace(metadata, "");

        // Take the first word that is not part of `use=` as the tag
        let tag = metadata_without_use
            .split_whitespace()
            .next()
            .map(|s| s.to_string());

        (tag, imports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata_with_use() {
        let metadata = "use=[block1,block2] tag1";
        let (tag, imports) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag1".to_string()));
        assert_eq!(imports, vec!["block1".to_string(), "block2".to_string()]);
    }

    #[test]
    fn test_parse_metadata_without_use() {
        let metadata = "tag2";
        let (tag, imports) = CodeBlock::parse_metadata(metadata);
        assert_eq!(tag, Some("tag2".to_string()));
        assert!(imports.is_empty());
    }

    #[test]
    fn test_parse_metadata_empty() {
        let metadata = "";
        let (tag, imports) = CodeBlock::parse_metadata(metadata);
        assert!(tag.is_none());
        assert!(imports.is_empty());
    }
}
