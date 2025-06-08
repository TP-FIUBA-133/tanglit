#[cfg(test)]
mod test;

use crate::errors::ParserError;
use log::debug;
use markdown::mdast::{List, Node, Paragraph, Text};
use serde::Serialize;
use serde_json;

const REPEAT_TITLE: &str = "---";

// Currently, blocks are strings, but they should later be a struct

fn process_list(list_node: &List) -> Option<List> {
    let mut new_list = List {
        children: vec![],
        position: list_node.position.clone(),
        ordered: false,
        start: None,
        spread: false,
    };
    'list_items_loop: for item in &list_node.children {
        if let Node::ListItem(list_item) = item {
            let mut new_item = list_item.clone();
            new_item.children = vec![];
            for child in list_item.children.iter() {
                if let Node::Paragraph(p) = child {
                    let first_child = p.children.iter().next();
                    if let Some(Node::Text(t)) = first_child {
                        let first_line = t.value.lines().next();
                        if let Some(line) = first_line {
                            if line.ends_with("%l") {
                                // debug!("Excluding this entire list");
                                return None;
                            }
                            if line.ends_with("%i") {
                                // debug!("Excluding this list item: {}", line);
                                continue 'list_items_loop;
                            }
                        }
                    }
                    let new_paragraph = process_paragraph(p);
                    if let Some(np) = new_paragraph {
                        // debug!(
                        //     "Processed Paragraph: {}",
                        //     serde_json::to_string(&np).unwrap()
                        // );
                        new_item.children.push(Node::Paragraph(np));
                    }
                }
            }
            new_list.children.push(Node::ListItem(new_item));
        }
    }
    return Some(new_list);
}

fn process_paragraph(paragraph: &Paragraph) -> Option<Paragraph> {
    let mut new_paragraph = Paragraph {
        children: vec![],
        position: paragraph.position.clone(),
    };
    for child in &paragraph.children {
        if let Node::Text(text) = child {
            let mut lines = text.value.lines();
            let first_line = lines.next();
            if let Some(line) = first_line {
                // debug!("First line {}", line);
                if line.ends_with("%p") {
                    // debug!("Excluding this paragraph: {}", line);
                    return None;
                }
            }
            let mut content = String::new();
            for line in text.value.lines() {
                if line.ends_with("%") {
                    // debug!("Excluding this line: {}", line);
                    continue;
                } else {
                    content.push_str((line.to_string() + "\n").as_str());
                }
            }
            let new_text = Text {
                value: content.trim().to_string(),
                position: text.position.clone(),
            };
            if !new_text.value.is_empty() {
                new_paragraph.children.push(Node::Text(new_text));
            }
        }
    }

    return Some(new_paragraph);
}

fn exclude_from_markdown(input_str: &str) -> Node {
    let mut new_children: Vec<Node> = vec![];
    let mut mdast = get_ast(input_str);
    for child in mdast.children().unwrap() {
        match child {
            Node::Paragraph(p) => {
                // debug!("Paragraph: {}", serde_json::to_string(&p).unwrap());
                let new_paragraph = process_paragraph(p);
                if let Some(np) = new_paragraph {
                    // debug!(
                    //     "Processed Paragraph: {}",
                    //     serde_json::to_string(&np).unwrap()
                    // );
                    new_children.push(Node::Paragraph(np));
                }
            }
            Node::Code(code) => {
                if let Some(meta_str) = &code.meta {
                    if meta_str.ends_with("%") {
                        continue;
                    }
                }
                new_children.push(child.clone());
            }
            Node::List(list) => {
                let new_list = process_list(&list);
                if let Some(nl) = new_list {
                    // debug!("Processed List: {:?}", nl);
                    new_children.push(Node::List(nl));
                } else {
                    // debug!("List was excluded");
                    continue;
                }
            }
            _ => {
                // For all other nodes, push them as they are
                new_children.push(child.clone());
            }
        }
    }
    if let Node::Root(r) = &mut mdast {
        r.children = new_children;
    }
    return mdast;
}

fn get_ast(input: &str) -> Node {
    markdown::to_mdast(input, &markdown::ParseOptions::mdx()).unwrap()
}

#[derive(Debug, Serialize)]
struct Slide {
    title: Option<usize>,
    content: Vec<usize>,
    start_line: usize,
}

fn get_slides(mdast: &Node, input: &str) -> Result<Vec<Slide>, ParserError> {
    // TODO: remove unwrap
    let x = get_slides_(mdast, input).unwrap_or_else(|e| {
        eprintln!("Error parsing slides: {:?}", e);
        let v: Vec<Slide> = vec![];
        return v;
    });
    return Ok(x);
}

fn get_slides_(mdast: &Node, input: &str) -> Result<Vec<Slide>, ParserError> {
    let mut slides: Vec<Slide> = vec![];
    let Some(children) = mdast.children() else {
        return Ok(slides);
    };
    for (i, child) in children.iter().enumerate() {
        let mut new_slide: Option<Slide> = None;
        if let Node::Heading(heading) = child {
            if heading.depth == 1 {
                if let Node::Text(_) = &heading.children[0] {
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
        slides[slides_len - 1].content.push(i);
    }
    Ok(slides)
}

pub fn get_slides_json(mdast: Node) -> Result<String, ParserError> {
    let slides = get_slides(&mdast, "")?;
    let x = serde_json::to_string(&slides)
        .map_err(|_| ParserError::InvalidInput("invalid input!".to_string()))?;
    return Ok(x);
}

pub fn process_slides(input: &str) -> Vec<usize> {
    let input_str = input.trim();
    let root_ast = get_ast(input_str);
    for c in root_ast.children().unwrap() {
        debug!("Child: {:?}", c);
    }
    let slides = get_slides(&root_ast, input_str).unwrap(); // TODO: remove unwrap
    let elements = root_ast.children().unwrap(); // TODO: remove unwrap
    let mut slides_start_lines = vec![];
    debug!(
        "Slides: {}",
        serde_json::to_string(&slides).unwrap_or_else(|_| "[]".to_string())
    );
    for s in &slides {
        let mut final_slide_markdown = "".to_string();
        if let Some(title) = s.title {
            let title_offset_start = elements[title].position().unwrap().start.offset;
            let title_offset_end = elements[title].position().unwrap().end.offset;
            let title = &input_str[title_offset_start..title_offset_end];
            //debug!("Slide title: {}", title);
            final_slide_markdown = title.to_string();
        }
        let offset_start = elements[s.content[0]].position().unwrap().start.offset;
        let offset_end = elements[s.content[s.content.len() - 1]]
            .position()
            .unwrap()
            .end
            .offset;
        let content = &input_str[offset_start..offset_end];
        //debug!("Slide content: {}", content);
        final_slide_markdown = final_slide_markdown.to_string() + "\n" + content;
        debug!("Final slide markdown: \n{}", final_slide_markdown);
        let slide_ast = exclude_from_markdown(final_slide_markdown.as_str());
        let slide_final_md = mdast_util_to_markdown::to_markdown(&slide_ast);
        debug!(
            "Slide markdown with exclusions: \n{}",
            slide_final_md.unwrap()
        );
        slides_start_lines.push(s.start_line);
    }
    debug!(
        "Slide line starts: {}",
        serde_json::to_string(slides_start_lines.as_slice()).unwrap()
    );
    return slides_start_lines;
}
