use crate::doc::parser::exclude::to_node::ToNode;
use markdown::mdast::{Code, List, ListItem, Node, Paragraph, Text};
use once_cell::sync::Lazy;
use regex::Regex;

#[cfg(test)]
mod test;
mod to_node;

static MARKER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.* (%[ipl]?)").unwrap());

pub enum FilterTarget {
    Pdf,
    Slides,
}

impl FilterTarget {
    fn code_marker(&self) -> &str {
        match self {
            FilterTarget::Pdf => "%",
            FilterTarget::Slides => "&",
        }
    }
    fn line_marker(&self) -> &str {
        self.code_marker() // same as code_marker
    }
    fn list_marker(&self) -> &str {
        match self {
            FilterTarget::Pdf => "%l",
            FilterTarget::Slides => "&l",
        }
    }
    fn list_item_marker(&self) -> &str {
        match self {
            FilterTarget::Pdf => "%i",
            FilterTarget::Slides => "&i",
        }
    }
    fn paragraph_marker(&self) -> &str {
        match self {
            FilterTarget::Pdf => "%p",
            FilterTarget::Slides => "&p",
        }
    }
}

pub fn exclude_from_ast(mdast: &Node, target: FilterTarget) -> Node {
    let mut new_mdast = mdast.clone();
    let new_children = process_children(mdast.children().unwrap(), &target);
    if let Node::Root(r) = &mut new_mdast {
        r.children = new_children;
    }
    new_mdast
}

fn process_children(children: &Vec<Node>, target: &FilterTarget) -> Vec<Node> {
    let mut new_children: Vec<Node> = vec![];
    for child in children {
        match child {
            Node::Paragraph(p) => {
                let Some(new_paragraph) = process_paragraph(p, target) else {
                    continue;
                };
                new_children.push(new_paragraph.to_node());
            }
            Node::Code(code) => {
                let Some(new_code) = process_code(code, target) else {
                    continue;
                };
                new_children.push(new_code.to_node());
            }
            Node::List(list) => {
                let Some(new_list) = process_list(list, target) else {
                    continue;
                };
                new_children.push(new_list.to_node());
            }
            _ => {
                // For all other nodes, push them as they are
                new_children.push(child.clone());
            }
        }
    }
    new_children
}

fn process_paragraph(paragraph: &Paragraph, target: &FilterTarget) -> Option<Paragraph> {
    let mut new_paragraph = Paragraph {
        children: vec![],
        position: None,
    };
    if get_paragraph_first_line_marker(paragraph) == Some(target.paragraph_marker().into()) {
        return None; // Exclude the entire paragraph if the first line has the marker
    }
    for child in &paragraph.children {
        let Node::Text(text) = child else {
            new_paragraph.children.push(child.clone());
            continue;
        };
        let Some(new_text) = process_text(text, target) else {
            continue; // Skip this text if it should be excluded
        };
        new_paragraph.children.push(new_text.to_node());
    }

    Some(new_paragraph)
}

fn process_code(code: &Code, target: &FilterTarget) -> Option<Code> {
    let Some(meta_str) = &code.meta else {
        return Some(code.clone());
    };

    if meta_str.ends_with(target.code_marker()) {
        return None; // Exclude this code block
    }

    Some(code.clone())
}

fn process_list(list_node: &List, target: &FilterTarget) -> Option<List> {
    let mut new_list = List {
        children: vec![],
        position: None,
        ordered: false,
        start: None,
        spread: false,
    };

    if list_node.children.is_empty() {
        return Some(new_list);
    }

    if should_exclude_list(list_node, target) {
        return None;
    }

    for item in &list_node.children {
        let Node::ListItem(list_item) = item else {
            panic!("Expected a ListItem");
        };
        let Some(new_item) = process_list_item(list_item, target) else {
            continue; // Skip this item if it should be excluded
        };
        new_list.children.push(new_item.to_node());
    }
    Some(new_list)
}

fn process_list_item(list_item: &ListItem, target: &FilterTarget) -> Option<ListItem> {
    if should_exclude_list_item(list_item, target) {
        return None;
    }
    let mut new_item = list_item.clone();
    new_item.children = process_children(&list_item.children, target);
    Some(new_item)
}

fn should_exclude_list(list_node: &List, target: &FilterTarget) -> bool {
    let Node::ListItem(first_item) = &list_node.children[0] else {
        panic!("Expected a ListItem")
    };

    if first_item.children.is_empty() {
        return false;
    }
    let Node::Paragraph(p) = &first_item.children[0] else {
        return false;
    };

    // Exclude the entire list if the first line of the first item has the marker
    let first_line_marker = get_paragraph_first_line_marker(p);
    first_line_marker == Some(target.list_marker().into())
}

fn should_exclude_list_item(list_item: &ListItem, target: &FilterTarget) -> bool {
    if list_item.children.is_empty() {
        return false; // No children to process
    }
    let Node::Paragraph(p) = &list_item.children[0] else {
        return false;
    };
    let item_marker = get_paragraph_first_line_marker(p);
    item_marker == Some(target.list_item_marker().into())
}

fn process_text(text: &Text, target: &FilterTarget) -> Option<Text> {
    let mut content = String::new();
    for line in text.value.lines() {
        if line.ends_with(target.line_marker()) {
            continue;
        } else {
            content.push_str((line.to_string() + "\n").as_str());
        }
    }
    if content.is_empty() {
        return None;
    }
    Some(Text {
        value: content.trim().to_string(),
        position: None,
    })
}

fn get_paragraph_first_line_marker(paragraph: &Paragraph) -> Option<String> {
    if let Some(Node::Text(text)) = paragraph.children.first() {
        let first_line = text.value.lines().next()?;
        return Some(MARKER_REGEX.captures(first_line)?[1].to_string());
    }
    None
}
