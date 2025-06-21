use crate::parser::parse_input;
use log::debug;
use markdown::mdast::{Code, List, ListItem, Node, Paragraph, Text};

#[cfg(test)]
mod test;

const EXCLUDE_CODE_MARKER: &str = "%";
const EXCLUDE_LINE_MARKER: &str = "%";
const EXCLUDE_LIST_MARKER: &str = "%l";
const EXCLUDE_LIST_ITEM_MARKER: &str = "%i";
const EXCLUDE_PARAGRAPH_MARKER: &str = "%p";

trait ToNode {
    fn to_node(self) -> Node;
}

impl ToNode for Paragraph {
    fn to_node(self) -> Node {
        Node::Paragraph(self)
    }
}
impl ToNode for Text {
    fn to_node(self) -> Node {
        Node::Text(self)
    }
}

impl ToNode for ListItem {
    fn to_node(self) -> Node {
        Node::ListItem(self)
    }
}

impl ToNode for List {
    fn to_node(self) -> Node {
        Node::List(self)
    }
}
impl ToNode for Code {
    fn to_node(self) -> Node {
        Node::Code(self)
    }
}

fn get_ast(input: &str) -> Node {
    markdown::to_mdast(input, &markdown::ParseOptions::mdx()).unwrap()
}

fn get_paragraph_first_line_marker(paragraph: &Paragraph) -> Option<String> {
    if let Some(Node::Text(text)) = paragraph.children.first() {
        let Some(first_line) = text.value.lines().next() else {
            return None;
        };
        let marker_regex =
            regex::Regex::new(r"^.* (%[ipl]?)").expect("Failed to compile marker regex");
        return Some(marker_regex.captures(&first_line)?[1].to_string());
    }
    None
}

fn should_exclude_list(list_node: &List) -> bool {
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
    let first_line_marker = get_paragraph_first_line_marker(&p);
    first_line_marker == Some(EXCLUDE_LIST_MARKER.into())
}

fn should_exclude_list_item(list_item: &ListItem) -> bool {
    if list_item.children.is_empty() {
        return false; // No children to process
    }
    let Node::Paragraph(p) = &list_item.children[0] else {
        return false;
    };
    let item_marker = get_paragraph_first_line_marker(p);
    item_marker == Some(EXCLUDE_LIST_ITEM_MARKER.into())
}

fn process_list_item(list_item: &ListItem) -> Option<ListItem> {
    if should_exclude_list_item(list_item) {
        return None;
    }
    let mut new_item = list_item.clone();
    new_item.children = process_children(&list_item.children);
    Some(new_item)
}

fn process_list(list_node: &List) -> Option<List> {
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

    if should_exclude_list(list_node) {
        return None;
    }

    for item in &list_node.children {
        let Node::ListItem(list_item) = item else {
            panic!("Expected a ListItem");
        };
        let Some(new_item) = process_list_item(list_item) else {
            continue; // Skip this item if it should be excluded
        };
        new_list.children.push(new_item.to_node());
    }
    Some(new_list)
}

fn process_text(text: &Text) -> Option<Text> {
    let mut content = String::new();
    for line in text.value.lines() {
        if line.ends_with(EXCLUDE_LINE_MARKER) {
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

fn process_paragraph(paragraph: &Paragraph) -> Option<Paragraph> {
    let mut new_paragraph = Paragraph {
        children: vec![],
        position: None,
    };
    if get_paragraph_first_line_marker(paragraph) == Some(EXCLUDE_PARAGRAPH_MARKER.into()) {
        return None; // Exclude the entire paragraph if the first line has the marker
    }
    for child in &paragraph.children {
        let Node::Text(text) = child else {
            new_paragraph.children.push(child.clone());
            continue;
        };
        let Some(new_text) = process_text(text) else {
            continue; // Skip this text if it should be excluded
        };
        new_paragraph.children.push(new_text.to_node());
    }

    Some(new_paragraph)
}

fn process_code(code: &Code) -> Option<Code> {
    let Some(meta_str) = &code.meta else {
        return Some(code.clone());
    };

    if meta_str.ends_with(EXCLUDE_CODE_MARKER) {
        return None; // Exclude this code block
    }

    Some(code.clone())
}

fn process_children(children: &Vec<Node>) -> Vec<Node> {
    let mut new_children: Vec<Node> = vec![];
    for child in children {
        match child {
            Node::Paragraph(p) => {
                let Some(new_paragraph) = process_paragraph(p) else {
                    continue;
                };
                new_children.push(new_paragraph.to_node());
            }
            Node::Code(code) => {
                let Some(new_code) = process_code(code) else {
                    continue;
                };
                new_children.push(new_code.to_node());
            }
            Node::List(list) => {
                let Some(new_list) = process_list(list) else {
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

pub fn exclude_from_markdown(input_str: &str) -> Node {
    let mut mdast = get_ast(input_str);
    let new_children = process_children(mdast.children().unwrap());
    if let Node::Root(r) = &mut mdast {
        r.children = new_children;
    }
    mdast
}
