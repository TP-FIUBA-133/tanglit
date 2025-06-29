use markdown::mdast::Code;
use regex::Regex;
use serde::Serialize;

use crate::errors::ParserError;

const USE_REGEX: &str = r"use=\[([^\]]*)\]";

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Language {
    Unknown,
    Python,
    Rust,
    C,
}

impl Language {
    pub fn parse_language(lang: &str) -> Self {
        match lang {
            "python" => Language::Python,
            "rust" => Language::Rust,
            "c" => Language::C,
            _ => Language::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlock {
    pub language: Language,
    pub code: String,
    pub tag: String,
    pub imports: Vec<String>,
    pub start_line: usize,
}

impl CodeBlock {
    pub fn new(
        language: Language,
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
        Self::new(Language::Unknown, code, "".to_string(), Vec::new(), 0)
    }

    /// Creates a CodeBlock from a Code node, extracting the language, code, tag, and imports.
    /// If the tag is not specified in the code block, it defaults to the line number of the code block.
    pub fn from_code_node(code_block: &Code) -> Result<Self, ParserError> {
        let language = Language::parse_language(code_block.lang.as_deref().unwrap_or_default());
        let (tag, imports) = Self::parse_metadata(code_block.meta.as_deref().unwrap_or_default());
        let start_line = code_block
            .position
            .as_ref()
            .ok_or_else(|| ParserError::CodeBlockError("Block position not found".to_string()))?
            .start
            .line;
        let tag = match tag {
            Some(t) => t,
            None => start_line.to_string(),
        };

        Ok(Self::new(
            language,
            code_block.value.clone(),
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
