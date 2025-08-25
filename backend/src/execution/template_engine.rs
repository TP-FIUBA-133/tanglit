use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const DEFAULT_INDENT_SIZE: usize = 4;
const DEFAULT_INDENT_CHARACTER: char = ' ';
const CONFIG_PLACEHOLDER_DEFAULT_PATTERN: &str = "#<([^#<>]+)>#";

#[derive(Debug, Clone)]
pub struct Template {
    // TODO: add configuration field to configure template parameters
    pub placeholder_pattern: Regex,
    pub template_content: String,
}

impl Template {
    /// Loads a template from a file path.
    pub fn load_from_file(
        file_path: &Path,
        placeholder_regex: &Option<String>,
    ) -> io::Result<Self> {
        let content = fs::read_to_string(file_path)?;
        Self::load(&content, placeholder_regex).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "No separator line found (5 or more dashes)",
            )
        })
    }

    /// Loads a template from the contents of a template file.
    pub fn load(content: &str, placeholder_regex: &Option<String>) -> Option<Self> {
        let placeholder = Regex::new(
            placeholder_regex
                .as_ref()
                .unwrap_or(&CONFIG_PLACEHOLDER_DEFAULT_PATTERN.to_string()),
        )
        .unwrap();

        let template_content = content.to_string();

        Some(Template {
            placeholder_pattern: placeholder,
            template_content,
        })
    }

    /// Renders the template with the provided replacements
    /// by interpolating the locations of the placeholders with their corresponding replacement values.  
    /// The interpolation takes into account indentation
    /// by indenting all lines after the first with the same amount of columns as the placeholder's position.
    /// Missing replacement values are removed from the template.
    /// This function assumes that the template contains 2 placeholder markers: IMPORTS and BODY.
    /// If any of these are missing, they will be replaced by empty strings.
    /// # Arguments
    /// * `imports` - contents of the imports section to be spliced
    ///   into the IMPORTS placeholder marker of the template
    /// * `body` - contents of the codeblock to be spliced into
    ///   the BODY placeholder marker of the template
    /// # Returns
    /// * A result with a string with the rendered template content
    ///   or an error if rendering fails
    pub fn render(&self, imports: &str, body: &str) -> Result<String, String> {
        let mut result = self.template_content.clone();
        let pattern = &self.placeholder_pattern;

        let replacements = HashMap::from([
            ("IMPORTS".to_string(), imports.to_string()),
            ("BODY".to_string(), body.to_string()),
        ]);

        process_replacements(&replacements, pattern, &mut result)?;

        Ok(result)
    }
}

fn process_replacements(
    replacements: &HashMap<String, String>,
    regex: &Regex,
    template: &mut String,
) -> Result<(), String> {
    // Replace all occurrences, adjusting indentation for each match
    let result = regex
        .replace_all(template, |caps: &regex::Captures| {
            // Extract the key from the first capture group
            if let Some(captured_key) = caps.get(1) {
                debug!("Found placeholder key: {}", captured_key.as_str());
                let replacement_value = replacements
                    .get(captured_key.as_str())
                    .cloned()
                    .unwrap_or("".to_string());
                format_replacement(template, replacement_value, caps)
            } else {
                // If no capture group, keep the original match
                caps.get(0).unwrap().as_str().to_string()
            }
        })
        .into_owned();
    *template = result;
    Ok(())
}

/// Sets indentation for each line in the code.
/// This function modifies the input code by intentionally indenting all lines *after* the first.
/// The first line is **not** indented to respect the indentation set by the template, otherwise
/// the first line would get indented twice.
///
/// # Arguments
/// * `code` - A mutable reference to the string containing the code to indent
/// * `indent_size` - Optional number of indentation characters to use per level (defaults to 4)
/// * `indent_character` - Optional character to use for indentation (defaults to space)
///
/// # examples
/// if we have a snippet of code like:
///
/// ~~~text
/// int x = 42;
/// std::cout << "the meaning of life is " << x << std::endl;
/// std::cout << "or something like that" << std::endl;
/// ~~~
/// and the desired indentation is 4 spaces, then the output string is actually:
/// ~~~text
/// int x = 42;
///     std::cout << "the meaning of life is " << x << std::endl;
///     std::cout << "or something like that" << std::endl;
/// ~~~
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
    use std::path::PathBuf;

    fn get_sample_template() -> String {
        r##"#<IMPORTS>#

void main(){
    #<BODY>#
    return;
}"##
        .to_string()
    }

    #[test]
    fn test_parse_template_config() {
        let content = get_sample_template();
        let config = Template::load(&content, &None).unwrap();
        assert!(config.template_content.contains("void main(){"));
    }

    #[test]
    fn test_render_with_replacements() {
        let content = get_sample_template();
        let config = Template::load(&content, &Option::from("#<([A-Z]+)>#".to_string())).unwrap();

        // The template expects to replace #<IMPORTS># and #<BODY>#
        let rendered = config
            .render("use std::io;", "println!(\"Hello\");")
            .unwrap();

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
    // missing replacements should remove placeholder patterns from the rendered output
    fn test_render_with_missing_replacements() {
        let mut content = get_sample_template();
        let placeholder = Regex::new(CONFIG_PLACEHOLDER_DEFAULT_PATTERN).unwrap();

        let mut replacements = HashMap::new();
        replacements.insert("IMPORTS".to_string(), "use std::io;".to_string());
        // Missing BODY replacement

        let result = process_replacements(&replacements, &placeholder, &mut content);

        assert!(result.is_ok());
        assert_eq!(
            content,
            r##"use std::io;

void main(){
    
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
        let result = Template::load_from_file(
            &PathBuf::from("nonexistent_file.txt"),
            &Option::from("<RWA>".to_string()),
        );
        assert!(result.is_err());
    }
}
