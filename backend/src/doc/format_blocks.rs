use comrak::Arena;
use comrak::nodes::{Ast, AstNode, LineColumn, NodeHtmlBlock, NodeValue};
use regex::Regex;
use std::cell::RefCell;

fn parse_metadata(metadata: &str) -> (Option<String>, Option<String>) {
    // Regex to capture `use=[...]`
    let use_re =
        Regex::new(crate::doc::parser::code_block::USE_REGEX).expect("Failed to compile USE_REGEX");

    // Remove the `use=[...]` part to get the block tag
    let metadata_without_use = use_re.replace(metadata, "");

    // Take the first word that is not part of `use=` as the tag
    let lang = metadata_without_use
        .split_whitespace()
        .next()
        .map(|s| s.to_string());

    let tag = metadata_without_use
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string());

    (lang, tag)
}

pub fn make_html_node<'a>(arena: &'a Arena<AstNode<'a>>, raw_html: &str) -> &'a AstNode<'a> {
    let html_ast = Ast::new(
        NodeValue::HtmlBlock(NodeHtmlBlock {
            block_type: 0,
            literal: raw_html.to_string() + "\n",
        }),
        LineColumn::from((0, 0)),
    );
    arena.alloc(AstNode::new(RefCell::new(html_ast)))
}

pub fn insert_html_before<'a>(
    arena: &'a Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
    raw_html: &str,
) -> &'a AstNode<'a> {
    let html_ast_node = make_html_node(arena, raw_html);
    node.insert_before(html_ast_node);
    html_ast_node
}

pub fn insert_html_after<'a>(
    arena: &'a Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
    raw_html: &str,
) -> &'a AstNode<'a> {
    let html_ast_node = make_html_node(arena, raw_html);
    node.insert_after(html_ast_node);
    html_ast_node
}

pub fn wrap<'a>(
    arena: &'a Arena<AstNode<'a>>,
    first_node: &'a AstNode<'a>,
    last_node: &'a AstNode<'a>,
    opening_html: &str,
    closing_html: &str,
) -> &'a AstNode<'a> {
    insert_html_before(arena, first_node, opening_html);
    insert_html_after(arena, last_node, closing_html)
}

pub fn ast_format_output(_lang: &Option<String>, tag: &Option<String>) -> (String, String) {
    let main_opening_html = r#"<div class="code-execution-pair">"#.to_string();
    let main_closing_html = r#"</div>"#.to_string();
    let block_tag_html = tag
        .as_ref()
        .map(|_tag| format!(r#"<div class="block-tag">{_tag}</div>"#));
    let mut opening_html = main_opening_html;
    if let Some(block_tag) = block_tag_html {
        opening_html.push_str(&block_tag);
    }
    (opening_html, main_closing_html)
}

pub fn ast_format_single_code_block(
    _lang: &Option<String>,
    tag: &Option<String>,
) -> (String, String) {
    let main_opening_html = r#"<div class="code-block">"#.to_string();
    let main_closing_html = r#"</div>"#.to_string();
    let block_tag_html = tag.as_ref().map(|_tag| {
        format!(r#"<div class="block-header"><span class="block-tag">{_tag}</span></div>"#)
    });
    let mut opening_html = main_opening_html;
    if let Some(block_tag) = block_tag_html {
        opening_html.push_str(&block_tag);
    }
    (opening_html, main_closing_html)
}

pub fn format_output_ast<'a>(
    arena: &'a Arena<AstNode<'a>>,
    lang: Option<String>,
    tag: Option<String>,
    code_block: &'a AstNode<'a>,
    output_block: &'a AstNode<'a>,
) -> Option<&'a comrak::arena_tree::Node<'a, RefCell<Ast>>> {
    // Takes a code block AstNode, and its corresponding output AstNode, and formats them together
    // by wrapping them in a div with appropriate classes and adding an "OUTPUT" header.
    // Returns the next sibling of the closing div for further traversal.
    let (opening_html, closing_html) = ast_format_output(&lang, &tag);
    insert_html_before(
        arena,
        output_block,
        r#"<div class="output-header">OUTPUT</div>"#,
    );
    let closing_div = wrap(
        arena,
        code_block,
        output_block,
        &opening_html,
        &closing_html,
    );
    closing_div.next_sibling()
}

pub fn format_code_blocks<'a>(root: &'a AstNode<'a>, arena: &'a Arena<AstNode<'a>>) {
    use comrak::nodes::NodeValue;

    // format code blocks | output block pairs
    let mut node = root.first_child();
    while let Some(current_node) = node {
        if let Some(next_node) = current_node.next_sibling() {
            // Check if current_node is a code block and next_node is an output block
            if let NodeValue::CodeBlock(block) = &current_node.data.borrow().value {
                let (lang, tag) = parse_metadata(block.info.as_ref());
                if block.info != "output" {
                    if let NodeValue::CodeBlock(next_block) = &next_node.data.borrow().value {
                        if next_block.info == "output" {
                            // We have a code block followed by the corresponding output block
                            // Add HTML formatting to them, so they are grouped together
                            // we add the code block language and tag, etc.
                            node = format_output_ast(arena, lang, tag, current_node, next_node);
                            continue;
                        }
                    }
                    // A standalone code block with no output block
                    let (opening_html, closing_html) = ast_format_single_code_block(&lang, &tag);
                    let closing_div = wrap(
                        arena,
                        current_node,
                        current_node,
                        &opening_html,
                        &closing_html,
                    );
                    node = closing_div.next_sibling();
                    continue;
                }
            }
        }

        node = current_node.next_sibling();
    }
}
