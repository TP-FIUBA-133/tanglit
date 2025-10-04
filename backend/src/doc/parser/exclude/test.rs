use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::parser::parse_from_string;
    use std::path::Path;

    fn exclude_from_markdown(input: &str, target: FilterTarget) -> Node {
        let mdast = parse_from_string(input).expect("Failed to parse markdown");
        exclude_from_ast(&mdast, target)
    }

    fn test_exclusions_file(file: &str, target: FilterTarget) {
        let file_path = Path::new(file!()); // "tests/my_test.rs"
        let dir = file_path.parent().unwrap().join("test_files");
        let in_file = dir.join(format!("{file}-in.md"));
        let out_file = dir.join(format!("{file}-out.md"));
        let input = std::fs::read_to_string(&in_file)
            .expect(format!("Failed to read file {}", in_file.display()).as_str());
        let expected_output = std::fs::read_to_string(&out_file)
            .expect(format!("Failed to read {}", out_file.display()).as_str());

        let ast_with_exclusions = exclude_from_markdown(input.as_str(), target);
        let actual_output = mdast_util_to_markdown::to_markdown(&ast_with_exclusions)
            .expect("Failed to convert to markdown");
        let actual_output = actual_output.trim();
        assert_eq!(
            expected_output, actual_output,
            "Output does not match expected for {file}"
        );
    }

    #[test_log::test]
    fn test_exclusions_1() {
        test_exclusions_file("doc/test_1", FilterTarget::Doc);
        test_exclusions_file("slides/test_1", FilterTarget::Slides);
    }
    #[test_log::test]
    fn test_exclusions_2() {
        test_exclusions_file("doc/test_2", FilterTarget::Doc);
        test_exclusions_file("slides/test_2", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_3() {
        test_exclusions_file("doc/test_3", FilterTarget::Doc);
        test_exclusions_file("slides/test_3", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_4() {
        test_exclusions_file("doc/test_4", FilterTarget::Doc);
        test_exclusions_file("slides/test_4", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_5() {
        test_exclusions_file("doc/test_5", FilterTarget::Doc);
        test_exclusions_file("slides/test_5", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_6() {
        test_exclusions_file("doc/test_6", FilterTarget::Doc);
        test_exclusions_file("slides/test_6", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_7() {
        test_exclusions_file("doc/test_7", FilterTarget::Doc);
        test_exclusions_file("slides/test_7", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_8() {
        test_exclusions_file("doc/test_8", FilterTarget::Doc);
        test_exclusions_file("slides/test_8", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_9() {
        test_exclusions_file("doc/test_9", FilterTarget::Doc);
        test_exclusions_file("slides/test_9", FilterTarget::Slides);
    }

    #[test_log::test]
    fn test_exclusions_10() {
        // test_exclusions_file("doc/test_10", FilterTarget::Doc);
        test_exclusions_file("slides/test_10", FilterTarget::Slides);
    }
}
