use crate::doc::parser::exclude::to_node::ToNode;
use markdown::mdast::{Code, List, ListItem, Node, Paragraph, Text};
use once_cell::sync::Lazy;
use regex::Regex;

#[cfg(test)]
mod test;
mod to_node;

// Matches one-or-more trailing markers (any mix of % / & with  i|optionalp|l), with optional spaces.
static TRAILING_MARKERS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:\s*(?:%[ipl]?|&[ipl]?))+\s*$").unwrap());

const PARAGRAPH_MARKER: char = 'p';
const LIST_MARKER: char = 'l';
const LIST_ITEM_MARKER: char = 'i';

pub enum FilterTarget {
    Doc,
    Slides,
}

impl FilterTarget {
    pub fn target_symbol(&self) -> char {
        match self {
            FilterTarget::Doc => '%',
            FilterTarget::Slides => '&',
        }
    }
}

// Return true if the trailing marker contains the target marker with the given suffix:
// suffix: None => bare line marker ("%"), Some('p') => paragraph, Some('l') => list, Some('i') => list item
fn has_target_marker(s: &str, target: &FilterTarget, suffix: Option<char>) -> bool {
    // Find the trailing markers part
    let Some(m) = TRAILING_MARKERS.find(s) else {
        return false;
    };
    let tail = &s[m.start()..];

    // Iterate pairs of (symbol, optional suffix) inside the tail
    // Example matches: "%", "%p", "&", "&l", "%i", "&p"
    static ONE_MARKER: Lazy<Regex> = Lazy::new(|| Regex::new(r"(%|&)([ipl]?)").unwrap());

    let target_sym = target.target_symbol();
    for cap in ONE_MARKER.captures_iter(tail) {
        let sym = cap.get(1).unwrap().as_str().as_bytes()[0] as char;
        let suf = cap.get(2).unwrap().as_str().chars().next(); // None if empty

        if sym == target_sym {
            match suffix {
                None => {
                    // Want bare line marker, so if suf is empty, it's a match
                    if suf.is_none() {
                        return true;
                    }
                }
                Some(c) => {
                    // Want specific suffix, so if suf matches, it's a match
                    if suf == Some(c) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// Strip *all* trailing markers for both targets from a line
fn strip_all_trailing_markers(s: &str) -> String {
    TRAILING_MARKERS.replace(s, "").to_string()
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

    // Exclude the entire paragraph if the first line's trailing markers include the paragraph marker
    if paragraph_has_marker(paragraph, target, Some(PARAGRAPH_MARKER)) {
        return None;
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

    // Exclude code block if the trailing markers include the target's bare line marker
    if has_target_marker(meta_str, target, None) {
        return None;
    }

    // Clean ALL markers so nothing leaks
    let cleaned_meta = strip_all_trailing_markers(meta_str);

    let mut new_code = code.clone();
    new_code.meta = Some(cleaned_meta);

    Some(new_code)
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

    // Exclude the entire list if the first line of the first item has the list marker
    paragraph_has_marker(p, target, Some(LIST_MARKER))
}

fn should_exclude_list_item(list_item: &ListItem, target: &FilterTarget) -> bool {
    if list_item.children.is_empty() {
        return false; // No children to process
    }
    let Node::Paragraph(p) = &list_item.children[0] else {
        return false;
    };
    paragraph_has_marker(p, target, Some(LIST_ITEM_MARKER))
}

fn process_text(text: &Text, target: &FilterTarget) -> Option<Text> {
    let mut content = String::new();

    for line in text.value.lines() {
        // Exclude the entire line if trailing markers include *target* bare marker (%, &)
        if has_target_marker(line, target, None) {
            continue;
        }

        // Strip ALL trailing markers (both %... and &...) on kept lines
        let cleaned_line = strip_all_trailing_markers(line);
        content.push_str(cleaned_line.trim_end());
        content.push('\n');
    }

    if content.trim().is_empty() {
        return None;
    }
    Some(Text {
        value: content.trim().to_string(),
        position: None,
    })
}

fn paragraph_has_marker(
    paragraph: &Paragraph,
    target: &FilterTarget,
    suffix: Option<char>,
) -> bool {
    if let Some(Node::Text(text)) = paragraph.children.first() {
        if let Some(first_line) = text.value.lines().next() {
            return has_target_marker(first_line, target, suffix);
        }
    }
    false
}
