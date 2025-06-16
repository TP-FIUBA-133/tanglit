pub fn get_slides_json(mdast: Node) -> Result<String, ParserError> {
    let slides = get_slides(&mdast, "")?;
    let x = serde_json::to_string(&slides)
        .map_err(|_| ParserError::InvalidInput("invalid input!".to_string()))?;
    return Ok(x);
}

const EXCLUDE_PARAGRAPH_MARKER: &str = "%p";
const EXCLUDE_LINE_MARKER: &str = "%";
const EXCLUDE_LIST_MARKER: &str = "%l";
const EXCLUDE_LIST_ITEM_MARKER: &str = "%i";

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
                            if line.ends_with(EXCLUDE_LIST_MARKER) {
                                return None;
                            }
                            if line.ends_with(EXCLUDE_LIST_ITEM_MARKER) {
                                continue 'list_items_loop;
                            }
                        }
                    }
                    let new_paragraph = process_paragraph(p);
                    if let Some(np) = new_paragraph {
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

pub fn _process_slides(root_ast: &Node, input_str: &str) -> Vec<usize> {
    let start_millis = std::time::Instant::now();
    let end_millis = std::time::Instant::now();
    debug!(
        "Processing took {} ms",
        (end_millis - start_millis).as_millis()
    );
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
    let mut total_millis = 0;
    for s in &slides {
        let mut final_slide_markdown = "".to_string();
        if let Some(title) = s.title {
            let title_position = elements[title]
                .position()
                .expect("Title should have a position");
            let title_offset_start = title_position.start.offset;
            let title_offset_end = title_position.end.offset;
            let title = &input_str[title_offset_start..title_offset_end];
            //debug!("Slide title: {}", title);
            final_slide_markdown = title.to_string();
        }
        if !s.content.is_empty() {
            let offset_start = elements[s.content[0]]
                .position()
                .expect("Content should have a position")
                .start
                .offset;
            let offset_end = elements[s.content[s.content.len() - 1]]
                .position()
                .expect("Content should have a position")
                .end
                .offset;
            let content = &input_str[offset_start..offset_end];
            //debug!("Slide content: {}", content);
            final_slide_markdown = final_slide_markdown.to_string() + "\n" + content;
        }
        debug!("Final slide markdown: \n{}", final_slide_markdown);
        let slide_ast = exclude_from_markdown(final_slide_markdown.as_str());
        let start_millis = std::time::Instant::now();
        // let slide_final_md = mdast_util_to_markdown::to_markdown(&slide_ast);
        let end_millis = std::time::Instant::now();
        total_millis += end_millis.duration_since(start_millis).as_millis();
        // debug!(
        //     "Slide markdown with exclusions: \n{}",
        //     slide_final_md.unwrap()
        // );
        slides_start_lines.push(s.start_line);
    }
    debug!(
        "Slide line starts: {}",
        serde_json::to_string(slides_start_lines.as_slice()).unwrap()
    );
    debug!("total to markdown time {} ", total_millis);
    return slides_start_lines;
}
