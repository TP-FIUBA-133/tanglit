use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

const DEFAULT_INDENT_SIZE: usize = 4;
const DEFAULT_INDENT_CHARACTER: char = ' ';

#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub placeholders: HashMap<String, String>,
    pub template_content: String,
}

/// Sets indentation for each line in the code.
///
/// # Arguments
/// * `code` - A mutable reference to the string containing the code to indent
/// * `indent_size` - Optional number of indentation characters to use per level (defaults to 4)
/// * `indent_character` - Optional character to use for indentation (defaults to space)
pub fn set_indentation(
    code: &mut String,
    indent_size: Option<usize>,
    indent_character: Option<char>,
) {
    let indent_str = indent_character
        .unwrap_or(DEFAULT_INDENT_CHARACTER)
        .to_string()
        .repeat(indent_size.unwrap_or(DEFAULT_INDENT_SIZE));

    let mut lines = code.lines();
    let mut result = String::new();

    if let Some(first_line) = lines.next() {
        result.push_str(first_line);
    }

    for line in lines {
        result.push('\n');
        result.push_str(&format!("{}{}", indent_str, line));
    }

    *code = result;
}

impl TemplateConfig {
    /// Parses a template configuration from a file path.
    pub fn parse_from_file(file_path: PathBuf) -> io::Result<Self> {
        let content = fs::read_to_string(file_path)?;
        Self::parse(&content).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "No separator line found (5 or more dashes)",
            )
        })
    }

    /// Parses a template configuration from the contents of a template file.
    /// Fails if the content does not contain a valid separator line (5 or more dashes).
    pub fn parse(content: &str) -> Option<Self> {
        let lines: Vec<&str> = content.lines().collect();

        // Find the separator line (5 or more dashes)
        let separator_index = lines
            .iter()
            .position(|line| line.trim().len() >= 5 && line.trim().chars().all(|c| c == '-'))?;

        // Parse key-value pairs from the first section
        let mut placeholders = HashMap::new();
        for line in &lines[..separator_index] {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().trim_matches('"').to_string();
                placeholders.insert(key, value);
            }
        }

        // Extract template content (everything after the separator)
        let template_content = lines[separator_index + 1..].join("\n");

        Some(TemplateConfig {
            placeholders,
            template_content,
        })
    }

    /// Renders the template with the provided replacements
    /// by interpolating the placeholders with their corresponding replacement values.  
    /// The interpolation takes into account indentation
    /// by indenting all lines after the first with the same amount of columns as the placeholder's position.
    /// # Arguments
    /// * `replacements` - A map of placeholder names to their replacement values
    /// # Returns
    /// * A result with a string with the rendered template content
    ///   or an error if rendering fails
    pub fn render(&self, replacements: &HashMap<String, String>) -> Result<String, String> {
        let mut result = self.template_content.clone();

        // Process each placeholder pattern defined in the config
        for (placeholder_name, pattern) in &self.placeholders {
            // Remove quotes if present
            let pattern = pattern.trim_matches('"');

            match Regex::new(pattern) {
                Ok(regex) => {
                    // Replace all occurrences, adjusting indentation for each match
                    result = regex
                        .replace_all(&result, |caps: &regex::Captures| {
                            // Extract the key from the first capture group
                            if let Some(captured_key) = caps.get(1) {
                                let key = captured_key.as_str();
                                println!(
                                    "Replacing '{}' with '{}'",
                                    key,
                                    replacements.get(key).map_or("", |v| v)
                                );
                                // Look up the replacement value using the captured key
                                if let Some(replacement_value) = replacements.get(key) {
                                    format_replacement(&result, replacement_value.to_string(), caps)
                                } else {
                                    // If no replacement found, keep the original match
                                    caps.get(0).unwrap().as_str().to_string()
                                }
                            } else {
                                // If no capture group, keep the original match
                                caps.get(0).unwrap().as_str().to_string()
                            }
                        })
                        .into_owned();
                    println!(
                        "Rendered template with placeholder '{}' using pattern '{}'",
                        placeholder_name, pattern
                    );
                }
                Err(e) => {
                    return Err(format!(
                        "Invalid regex pattern for '{}': {}",
                        placeholder_name, e
                    ));
                }
            }
        }

        Ok(result)
    }
}

fn format_replacement(
    wrapper_template: &str,
    replacement_value: String,
    captures: &regex::Captures,
) -> String {
    // Find the start of the match in the result string to calculate indentation
    let mat = captures.get(0).unwrap();
    let start = wrapper_template[..mat.start()]
        .rfind('\n')
        .map_or(0, |i| i + 1);
    let line = &wrapper_template[start..mat.start()];

    // Calculate indentation: count all characters from start of line to match
    // Doesn't handle tabs correctly, but assumes spaces for simplicity
    let indent_size = line.len();

    let mut replacement = replacement_value;
    if indent_size > 0 {
        set_indentation(
            &mut replacement,
            Some(indent_size),
            Some(' '), // Use space as default indent character
        );
    }
    replacement
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_template() -> String {
        r##"PLACEHOLDER = "#<([^#<>]+)>#"
-----
#<IMPORTS>#

void main(){
    #<BODY>#
    return;
}"##
        .to_string()
    }

    #[test]
    fn test_parse_template_config() {
        let content = get_sample_template();
        let config = TemplateConfig::parse(&content).unwrap();
        assert_eq!(config.placeholders.len(), 1);
        assert_eq!(
            config.placeholders.get("PLACEHOLDER").unwrap(),
            "#<([^#<>]+)>#"
        );
        assert!(config.template_content.contains("void main(){"));
    }

    #[test]
    fn test_render_with_replacements() {
        let content = get_sample_template();
        let config = TemplateConfig::parse(&content).unwrap();

        // This template expects to replace #<IMPORTS># and #<BODY>#, so let's add those as well
        let mut replacements = HashMap::new();
        replacements.insert("IMPORTS".to_string(), "use std::io;".to_string());
        replacements.insert("BODY".to_string(), "println!(\"Hello\");".to_string());

        // First, render PLACEHOLDER
        let rendered = config.render(&replacements).unwrap();

        assert_eq!(
            rendered,
            r##"use std::io;

void main(){
    println!("Hello");
    return;
}"##
        );
    }

    #[test]
    fn test_set_indentation_default() {
        let mut code = "fn main() {\nprintln!(\"hi\");\n}".to_string();
        set_indentation(&mut code, None, None);
        let expected = "fn main() {\n    println!(\"hi\");\n    }";
        assert_eq!(code, expected);
    }

    #[test]
    fn test_set_indentation_custom() {
        let mut code = "let x = 1;\nlet y = 2;".to_string();
        set_indentation(&mut code, Some(2), Some('\t'));
        let expected = "let x = 1;\n\t\tlet y = 2;";
        assert_eq!(code, expected);
    }

    #[test]
    fn test_parse_from_file_error() {
        let result = TemplateConfig::parse_from_file(PathBuf::from("nonexistent_file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_separator() {
        let content = "KEY=VALUE\nno separator here";
        let result = TemplateConfig::parse(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_render_invalid_regex() {
        let config = TemplateConfig {
            placeholders: [("BAD".to_string(), "[".to_string())]
                .iter()
                .cloned()
                .collect(),
            template_content: "test".to_string(),
        };
        let mut replacements = HashMap::new();
        replacements.insert("something".to_string(), "oops".to_string());
        let result = config.render(&replacements);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_no_placeholder_match() {
        let config = TemplateConfig {
            placeholders: [("FOO".to_string(), "\\-(foo)\\-".to_string())]
                .iter()
                .cloned()
                .collect(),
            template_content: "bar".to_string(),
        };
        let mut replacements = HashMap::new();
        replacements.insert("foo".to_string(), "baz".to_string());
        let rendered = config.render(&replacements).unwrap();
        assert_eq!(rendered, "bar");
    }

    #[test]
    fn test_render_multiple_placeholders() {
        let content = r#"A = "\,(\w)\,"
B = "\*(\d+)\*"
-----
a: ,a,
b: *42*
"#;
        let config = TemplateConfig::parse(content).unwrap();
        let mut replacements = HashMap::new();
        replacements.insert("a".to_string(), "Alpha".to_string());
        replacements.insert("42".to_string(), "Beta".to_string());
        let rendered = config.render(&replacements).unwrap();
        assert_eq!(rendered, "a: Alpha\nb: Beta");
    }
}
