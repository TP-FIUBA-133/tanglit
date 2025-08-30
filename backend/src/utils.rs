const DEFAULT_INDENT_SIZE: usize = 4;
const DEFAULT_INDENT_CHARACTER: char = ' ';

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
