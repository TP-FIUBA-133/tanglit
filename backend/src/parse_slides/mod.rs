#[cfg(test)]
mod test;

use markdown::mdast::Node;
use serde::Serialize;

const REPEAT_TITLE: &str = "---";

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct Slide {
    title: Option<usize>,
    content: Vec<usize>,
    start_line: usize,
}

pub fn get_slides(mdast: &Node, input: &str) -> Vec<Slide> {
    let mut slides = vec![];
    let Some(children) = mdast.children() else {
        return slides;
    };
    for (i, child) in children.iter().enumerate() {
        let mut new_slide = None;
        if let Node::Heading(heading) = child {
            if heading.depth == 1 {
                if heading.children.is_empty() {
                    // it's empty, we still have a slide, but with no title
                    new_slide = Some(Slide {
                        title: None,
                        content: vec![],
                        start_line: heading.position.as_ref().unwrap().start.line,
                    });
                } else if let Node::Text(_) = &heading.children[0] {
                    new_slide = Some(Slide {
                        title: Some(i),
                        content: vec![],
                        start_line: heading.position.as_ref().unwrap().start.line,
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
            if input[pos.start.offset..pos.end.offset].trim() == REPEAT_TITLE {
                new_slide_title = slides[slides.len() - 1].title;
            }
            new_slide = Some(Slide {
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
            slides.push(Slide {
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
