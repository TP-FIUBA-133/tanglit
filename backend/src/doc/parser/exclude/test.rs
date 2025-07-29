use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::parser::parse_from_string;
    use std::path::Path;

    fn exclude_from_markdown(input: &str) -> Node {
        let mdast = parse_from_string(input).expect("Failed to parse markdown");
        exclude_from_ast(&mdast)
    }

    fn test_exclusions_file(file: &str) {
        let file_path = Path::new(file!()); // "tests/my_test.rs"
        let dir = file_path.parent().unwrap().join("test_files");
        let in_file = dir.join(format!("{file}-in.md"));
        let out_file = dir.join(format!("{file}-out.md"));
        let input = std::fs::read_to_string(&in_file)
            .expect(format!("Failed to read file {}", in_file.display()).as_str());
        let expected_output = std::fs::read_to_string(&out_file)
            .expect(format!("Failed to read {}", out_file.display()).as_str());

        let ast_with_exclusions = exclude_from_markdown(input.as_str());
        let actual_output = mdast_util_to_markdown::to_markdown(&ast_with_exclusions)
            .expect("Failed to convert to markdown");
        let x = actual_output.trim();
        assert_eq!(
            expected_output, x,
            "Output does not match expected for {file}"
        );
    }

    #[test_log::test]
    fn test_exclusions_1() {
        test_exclusions_file("test_1")
    }
    #[test_log::test]
    fn test_exclusions_2() {
        test_exclusions_file("test_2")
    }

    #[test_log::test]
    fn test_exclusions_3() {
        test_exclusions_file("test_3")
    }

    #[test_log::test]
    fn test_exclusions_4() {
        test_exclusions_file("test_4")
    }

    #[test_log::test]
    fn test_exclusions_5() {
        test_exclusions_file("test_5")
    }

    #[test_log::test]
    fn test_exclusions_6() {
        test_exclusions_file("test_6")
    }

    #[test_log::test]
    fn test_exclusions_7() {
        test_exclusions_file("test_7")
    }

    #[test_log::test]
    fn test_exclusions_8() {
        test_exclusions_file("test_8")
    }

    #[test_log::test]
    fn test_exclusions_9() {
        test_exclusions_file("test_9")
    }
}
