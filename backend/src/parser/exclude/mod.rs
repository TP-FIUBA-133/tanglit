use log::debug;
use markdown::mdast::{List, Node, Paragraph, Text};
use markdown::unist::{Point, Position};

#[cfg(test)]
mod test;

fn get_ast(input: &str) -> Node {
    markdown::to_mdast(input, &markdown::ParseOptions::mdx()).unwrap()
}

const EXCLUDE_PARAGRAPH_MARKER: &str = "%p";
const EXCLUDE_LINE_MARKER: &str = "%";
const EXCLUDE_LIST_MARKER: &str = "%l";
const EXCLUDE_LIST_ITEM_MARKER: &str = "%i";

fn process_list(list_node: &List) -> Option<List> {
    let mut new_list = List {
        children: vec![],
        position: None,
        ordered: false,
        start: None,
        spread: false,
    };
    'list_items_loop: for item in &list_node.children {
        if let Node::ListItem(list_item) = item {
            let mut new_item = list_item.clone();
            for child in list_item.children.iter() {
                if let Node::Paragraph(p) = child {
                    let first_child = p.children.iter().next();
                    if let Some(Node::Text(t)) = first_child {
                        let first_line = t.value.lines().next();
                        if let Some(line) = first_line {
                            if line.ends_with(EXCLUDE_LIST_MARKER) {
                                return None;
                            }
                            if line.ends_with(EXCLUDE_LIST_ITEM_MARKER) {
                                continue 'list_items_loop; // Exclude this list item completely, no need to process the rest of its children
                            }
                        }
                    }
                }
            }
            new_item.children = process_children(&list_item.children);
            new_list.children.push(Node::ListItem(new_item));
        }
    }
    return Some(new_list);
}

fn process_paragraph(paragraph: &Paragraph) -> Option<Paragraph> {
    let mut new_paragraph = Paragraph {
        children: vec![],
        position: None,
    };
    for child in &paragraph.children {
        if let Node::Text(text) = child {
            let mut lines = text.value.lines();
            let first_line = lines.next();
            if let Some(line) = first_line {
                // debug!("First line {}", line);
                if line.ends_with(EXCLUDE_PARAGRAPH_MARKER) {
                    // debug!("Excluding this paragraph: {}", line);
                    return None;
                }
            }
            let mut content = String::new();
            for line in text.value.lines() {
                if line.ends_with(EXCLUDE_LINE_MARKER) {
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

fn process_text(text: &Text) -> Option<Text> {
    let mut new_text = Text {
        value: String::new(),
        position: None,
    };
    let mut line_start_offset = vec![0];
    for (i, char) in text.value.char_indices() {
        if (char == '\n') || (char == '\r') {
            line_start_offset.push(i + 1);
        }
    }
    line_start_offset.push(text.position.clone().unwrap().end.offset);
    for (i, line) in text.value.lines().enumerate() {
        if (line.ends_with(" %")) {
            continue;
        }
        new_text.value.push_str(line);
    }
    return Some(new_text);
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
    return new_children;
}

fn exclude_from_markdown(input_str: &str) -> Node {
    let mut new_children: Vec<Node> = vec![];
    let mut mdast = get_ast(input_str);
    new_children = process_children(mdast.children().unwrap());
    if let Node::Root(r) = &mut mdast {
        r.children = new_children;
    }
    return mdast;
}
