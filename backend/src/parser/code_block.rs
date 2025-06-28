use markdown::mdast::Code;
use regex::Regex;
use serde::Serialize;

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
    pub tag: Option<String>,
    pub imports: Vec<String>,
    pub start_line: usize,
}

impl CodeBlock {
    pub fn new(
        language: Language,
        code: String,
        tag: Option<String>,
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
        Self::new(Language::Unknown, code, None, Vec::new(), 0)
    }

    pub fn from_code_node(code_block: Code) -> Self {
        let language = Language::parse_language(&code_block.lang.unwrap_or_default());
        let (tag, imports) = Self::parse_metadata(&code_block.meta.unwrap_or_default());
        Self::new(
            language,
            code_block.value,
            tag,
            imports,
            code_block
                .position
                .expect("CodeBlock expected to have a position")
                .start
                .line,
        )
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
