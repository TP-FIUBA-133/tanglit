use crate::errors::ExecutionError;
use crate::utils::get_indentation_at_offset;
use crate::utils::set_indentation;
use log::debug;
use regex::Regex;
use std::collections::HashMap;

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
pub fn render(
    template: String,
    regex: &Regex,
    imports: &str,
    body: &str,
) -> Result<String, ExecutionError> {
    let replacements = HashMap::from([
        ("IMPORTS".to_string(), imports.to_string()),
        ("BODY".to_string(), body.to_string()),
    ]);

    let result = process_replacements(&replacements, regex, template)?;

    Ok(result)
}

// TODO: Remove unwraps.
fn process_replacements(
    replacements: &HashMap<String, String>,
    regex: &Regex,
    template: String,
) -> Result<String, ExecutionError> {
    // Replace all occurrences, adjusting indentation for each match
    let result = regex
        .replace_all(&template, |caps: &regex::Captures| {
            // Extract the key from the first capture group
            if let Some(captured_key) = caps.get(1) {
                debug!("Found placeholder key: {}", captured_key.as_str());
                let replacement_value = replacements
                    .get(captured_key.as_str())
                    .cloned()
                    .unwrap_or("".to_string());
                format_replacement(&template, replacement_value, caps)
            } else {
                // If no capture group, keep the original match
                caps.get(0).unwrap().as_str().to_string()
            }
        })
        .into_owned();
    Ok(result)
}

fn format_replacement(
    wrapper_template: &str,
    replacement_value: String,
    captures: &regex::Captures,
) -> String {
    // Find the start of the match in the result string to calculate indentation
    let mat = captures.get(0).unwrap();
    let start_offset = mat.start();

    let indent_size = get_indentation_at_offset(wrapper_template, start_offset);

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
    use crate::configuration::language_config::CONFIG_PLACEHOLDER_DEFAULT_PATTERN;

    use super::*;

    fn get_sample_template() -> String {
        r##"#<IMPORTS>#

void main(){
    #<BODY>#
    return;
}"##
        .to_string()
    }

    #[test]
    fn test_render_with_replacements() {
        let template = get_sample_template();
        let regex = Regex::new(CONFIG_PLACEHOLDER_DEFAULT_PATTERN).unwrap();

        // The template expects to replace #<IMPORTS># and #<BODY>#
        let rendered = render(template, &regex, "use std::io;", "println!(\"Hello\");").unwrap();

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
        let template = get_sample_template();
        let placeholder = Regex::new(CONFIG_PLACEHOLDER_DEFAULT_PATTERN).unwrap();

        let mut replacements = HashMap::new();
        replacements.insert("IMPORTS".to_string(), "use std::io;".to_string());
        // Missing BODY replacement

        let result = process_replacements(&replacements, &placeholder, template).unwrap();

        assert_eq!(
            result,
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
}
