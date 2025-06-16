use super::*;

#[cfg(test)]
mod tests {
    use crate::parser::input_to_mdast;
    use super::*;

    #[test]
    fn test_empty_markdown_returns_empty_slides() {
        let input = r#""#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        assert_eq!(slides, Vec::<Slide>::new());
    }
    #[test]
    fn test_single_slide_no_title() {
        let input = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.
- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl. 
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slide = Slide {
            title: None,
            content: vec![0, 1], // 0: paragraph node, 1: list node
            start_line: 1,
        };
        assert_eq!(slides, vec![expected_slide]);
    }

    #[test]
    fn test_single_slide_title() {
        let input = r#"
# Title of the first slide
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.
- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl. 
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slide = Slide {
            title: Some(0),      // 0: title node
            content: vec![1, 2], // 1: paragraph node, 2: list node
            start_line: 1,
        };
        assert_eq!(slides, vec![expected_slide]);
    }

    #[test]
    fn test_single_slide_empty_title() {
        // We can have empty titles in markdown
        let input = r#"
#
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.
- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl. 
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slide = Slide {
            title: None,         // no title (just like starting slide with --- ---)
            content: vec![1, 2], // 1: paragraph node, 2: list node
            start_line: 1,
        };
        assert_eq!(slides, vec![expected_slide]);
    }

    #[test]
    fn test_multiple_slides_with_title() {
        let input = r#"
# Title of the first slide
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.
# Title of the second slide
- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl. 
# Title of the third slide
Interdum et malesuada fames ac ante ipsum primis in faucibus. 
Fusce ultricies magna eget ultrices fringilla. Nullam egestas, metus 
sed accumsan varius, odio metus porta ante, id feugiat erat tortor eget lacus.
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slides = vec![
            Slide {
                title: Some(0),   // 0: title node
                content: vec![1], // 1: paragraph node
                start_line: 1,
            },
            Slide {
                title: Some(2),   // 2: title node
                content: vec![3], // 3: list node
                start_line: 5,
            },
            Slide {
                title: Some(4),   // 4: title node
                content: vec![5], // 5: paragraph node
                start_line: 10,
            },
        ];
        assert_eq!(slides, expected_slides);
    }

    #[test]
    fn test_repeat_slide_title() {
        let input = r#"
# Title of the first slide
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.

---

- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl.
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slides = vec![
            Slide {
                title: Some(0),   // 0: title node
                content: vec![1], // 1: paragraph node
                start_line: 1,
            },
            Slide {
                title: Some(0),   // 0: title node (same as previous slide)
                content: vec![3], // 3: list node
                start_line: 6,
            },
        ];
        assert_eq!(slides, expected_slides);
    }

    #[test]
    fn test_repeat_slide_title_twice() {
        let input = r#"
# Title of the first slide
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.

---

- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl.

---

Interdum et malesuada fames ac ante ipsum primis in faucibus. 
Fusce ultricies magna eget ultrices fringilla. Nullam egestas, metus 
sed accumsan varius, odio metus porta ante, id feugiat erat tortor eget lacus.
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slides = vec![
            Slide {
                title: Some(0),   // 0: title node
                content: vec![1], // 1: paragraph node
                start_line: 1,
            },
            Slide {
                title: Some(0),   // 0: title node (same as previous slide)
                content: vec![3], // 3: list node
                start_line: 6,
            },
            Slide {
                title: Some(0),   // 0: title node (same as previous slide)
                content: vec![5], // 3: list node
                start_line: 13,
            },
        ];
        assert_eq!(slides, expected_slides);
    }

    #[test]
    fn test_slide_no_title() {
        let input = r#"
# Title of the first slide
Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Praesent facilisis elit non odio viverra, ac faucibus libero egestas. 
Cras lacinia non justo at ornare.

--- ---

- Lorem ipsum dolor sit amet, consectetur adipiscing elit.
- Fusce et tortor egestas, dignissim sapien eget, vulputate tortor.
- Proin imperdiet nulla vel hendrerit aliquet.
- Fusce id tellus vitae lectus ornare venenatis consectetur quis nisl.
        "#;
        let input_str = input.trim();
        let root_ast = input_to_mdast(input_str).expect("Input expected to be ok");
        let slides = get_slides(&root_ast, input_str);
        let expected_slides = vec![
            Slide {
                title: Some(0),   // 0: title node
                content: vec![1], // 1: paragraph node
                start_line: 1,
            },
            Slide {
                title: None,      // no title
                content: vec![3], // 3: list node
                start_line: 6,
            },
        ];
        assert_eq!(slides, expected_slides);
    }
}
