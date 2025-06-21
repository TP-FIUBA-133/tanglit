use crate::parser::parse_input;
use log::debug;
use markdown::mdast::{List, Node, Paragraph, Text};

#[cfg(test)]
mod test;

const EXCLUDE_CODE_MARKER: &str = "%";
const EXCLUDE_LINE_MARKER: &str = "%";
const EXCLUDE_LIST_MARKER: &str = "%l";
const EXCLUDE_LIST_ITEM_MARKER: &str = "%i";
const EXCLUDE_PARAGRAPH_MARKER: &str = "%p";

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
    // check the first line of the first item in the list for the EXCLUDE_LIST_MARKER
    let Node::ListItem(first_item) = &list_node.children[0] else {
        panic!("Expected a ListItem")
    };
    if !first_item.children.is_empty() {
        if let Node::Paragraph(p) = &first_item.children[0] {
            let first_line_marker = get_paragraph_first_line_marker(&p);
            if first_line_marker == Some(EXCLUDE_LIST_MARKER.into()) {
                // Exclude the entire list if the first line of the first item has the marker
                return None;
            }
        }
    }

    'list_items_loop: for item in &list_node.children {
        let Node::ListItem(list_item) = item else {
            panic!("Expected a ListItem");
        };
        if let Node::Paragraph(p) = &list_item.children[0] {
            let item_marker = get_paragraph_first_line_marker(p);
            if item_marker == Some(EXCLUDE_LIST_ITEM_MARKER.into()) {
                // Exclude this list item completely, no need to process the rest of its children
                continue 'list_items_loop;
            }
        }
        let mut new_item = list_item.clone();
        new_item.children = process_children(&list_item.children);
        new_list.children.push(Node::ListItem(new_item));
    }
    Some(new_list)
}

fn exclude_lines_from_text(text: &Text) -> Text {
    let mut content = String::new();
    for line in text.value.lines() {
        if line.ends_with(EXCLUDE_LINE_MARKER) {
            continue;
        } else {
            content.push_str((line.to_string() + "\n").as_str());
        }
    }
    Text {
        value: content.trim().to_string(),
        position: None,
    }
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
        let Node::Text(text) = child else { continue };
        let new_text = exclude_lines_from_text(text);
        if !new_text.value.is_empty() {
            new_paragraph.children.push(Node::Text(new_text));
        }
    }

    Some(new_paragraph)
}

fn process_children(children: &Vec<Node>) -> Vec<Node> {
    let mut new_children: Vec<Node> = vec![];
    for child in children {
        match child {
            Node::Paragraph(p) => {
                debug!("Paragraph: {}", serde_json::to_string_pretty(&p).unwrap());
                let new_paragraph = process_paragraph(p);
                if let Some(np) = new_paragraph {
                    debug!(
                        "Processed Paragraph: {}",
                        serde_json::to_string_pretty(&np).unwrap()
                    );
                    new_children.push(Node::Paragraph(np));
                }
            }
            Node::Code(code) => {
                if let Some(meta_str) = &code.meta {
                    if meta_str.ends_with(EXCLUDE_CODE_MARKER) {
                        continue;
                    }
                }
                new_children.push(child.clone());
            }
            Node::List(list) => {
                let new_list = process_list(list);
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
