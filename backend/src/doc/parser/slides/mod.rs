#[cfg(test)]
mod tests;

use markdown::mdast::Node;
use serde::Serialize;

const REPEAT_TITLE: &str = "---";

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct SlideByIndex {
    title: Option<usize>, // index of the title node in the AST
    content: Vec<usize>,  // indices of the content node in the AST
    start_line: usize,    // start line in the raw markdown
}

pub fn parse_slides_index_from_ast(mdast: &Node, input: &str) -> Vec<SlideByIndex> {
    let mut slides = vec![];
    let Some(children) = mdast.children() else {
        return slides;
    };
    for (i, child) in children.iter().enumerate() {
        let mut new_slide = None;
        if let Node::Heading(heading) = child {
            if heading.depth == 1 {
                let start_line = heading.position.as_ref().unwrap().start.line;
                if heading.children.is_empty() {
                    // it's empty, we still have a slide, but with no title
                    new_slide = Some(SlideByIndex {
                        title: None,
                        content: vec![],
                        start_line,
                    });
                } else if let Node::Text(_) = &heading.children[0] {
                    new_slide = Some(SlideByIndex {
                        title: Some(i),
                        content: vec![],
                        start_line,
                    });
                }
            }
        } else if let Node::ThematicBreak(n) = child {
            let mut new_slide_title: Option<usize> = None;
            let pos = n.position.as_ref().unwrap();
            let new_slide_start_line = n.position.as_ref().unwrap().start.line;
            /*  TODO Review this
                TODO The AST doesn't differentiate between "---" and "--- ---"
                TODO or "-------" or even "****" / "_ _ _" are all ThematicBreaks
            */
            let thematic_break_text = input[pos.start.offset..pos.end.offset].trim();
            if thematic_break_text == REPEAT_TITLE && !slides.is_empty() {
                new_slide_title = slides[slides.len() - 1].title;
            }
            new_slide = Some(SlideByIndex {
                title: new_slide_title,
                content: vec![],
                start_line: new_slide_start_line,
            });
        }
        if let Some(slide) = new_slide {
            slides.push(slide);
            continue;
        }
        let slides_len = slides.len();
        if slides_len == 0 {
            // If the markdown starts without a title, we still have a slide
            slides.push(SlideByIndex {
                title: None,
                content: vec![i],
                start_line: child.position().unwrap().start.line,
            })
        } else {
            slides[slides_len - 1].content.push(i);
        }
    }
    slides
}

#[derive(Debug)]
pub struct Slide {
    pub title: Option<Node>,
    pub content: Vec<Node>,
}

pub fn parse_slides_from_ast(mdast: &Node, input: &str) -> Vec<Slide> {
    let mut slides = vec![];
    let Some(children) = mdast.children() else {
        return slides;
    };

    for child in children {
        let mut new_slide = None;

        if let Node::Heading(heading) = child {
            if heading.depth == 1 {
                if heading.children.is_empty() {
                    new_slide = Some(Slide {
                        title: None,
                        content: vec![],
                    });
                } else if let Node::Text(_) = &heading.children[0] {
                    new_slide = Some(Slide {
                        title: Some(child.clone()),
                        content: vec![],
                    });
                }
            }
        } else if let Node::ThematicBreak(n) = child {
            let mut new_slide_title: Option<Node> = None;
            let pos = n.position.as_ref().unwrap();
            let thematic_break_text = input[pos.start.offset..pos.end.offset].trim();

            if thematic_break_text == REPEAT_TITLE && !slides.is_empty() {
                new_slide_title = slides.last().unwrap().title.clone();
            }

            new_slide = Some(Slide {
                title: new_slide_title,
                content: vec![],
            });
        }

        if let Some(slide) = new_slide {
            slides.push(slide);
            continue;
        }

        if slides.is_empty() {
            slides.push(Slide {
                title: None,
                content: vec![child.clone()],
            });
        } else {
            slides.last_mut().unwrap().content.push(child.clone());
        }
    }
    slides
}

impl Slide {
    pub fn get_html(&self) -> Result<String, crate::doc::DocError> {
        let mut slide_md = String::new();

        if let Some(title_node) = &self.title {
            // slide_md.push_str("# ");
            slide_md.push_str(&crate::doc::parser::ast_to_markdown(title_node)?);
            // slide_md.push_str("\n\n");
        }

        for content_node in &self.content {
            slide_md.push_str(&crate::doc::parser::ast_to_markdown(content_node)?);
            // slide_md.push_str("\n\n");
        }

        Ok(crate::doc::parser::markdown_to_html(&slide_md))
    }
}
