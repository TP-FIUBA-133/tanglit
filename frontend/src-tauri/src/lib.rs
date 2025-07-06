use backend::parser::code_block::CodeBlock;
use backend::parser::input_to_mdast;
use backend::parser::slides::Slide;
use std::collections::HashMap;
use std::fs;

#[tauri::command(rename_all = "snake_case")]
fn tanglit_exclude(raw_markdown: &str) -> Result<String, String> {
    let rv = backend::parser::exclude::exclude_from_markdown(raw_markdown);
    let default_options = mdast_util_to_markdown::Options::default();
    let options = mdast_util_to_markdown::Options {
        bullet: '-',
        rule: '-',
        ..default_options
    };
    mdast_util_to_markdown::to_markdown_with_options(&rv, &options)
        .map_err(|e| "Error converting AST to markdown".to_string())
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_slides(raw_markdown: &str) -> Vec<Slide> {
    let mdast = input_to_mdast(raw_markdown).expect("Failed to parse input to mdast");
    let rv = backend::parser::slides::get_slides(&mdast, raw_markdown);
    rv
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_blocks(raw_markdown: &str) -> Vec<CodeBlock> {
    let mdast = input_to_mdast(raw_markdown).expect("Failed to parse input to mdast");
    let rv = backend::parser::parse_code_blocks(raw_markdown.to_string())
        .unwrap_or_else(|_| HashMap::new())
        .iter()
        .map(|a| (a.1.clone()))
        .collect();
    rv
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_execute_block(raw_markdown: &str, block_name: &str) -> String {
    // write raw_markdown to a file
    let data = raw_markdown;
    let file_name = "output.md"; // TODO: use a temporary file instead
    fs::write(file_name, data).map_err(|e| format!("Error writing file: {}", e)); // TODO: handle error properly

    match backend::execution::execute(file_name, block_name) {
        Ok(output) => output.to_string(),
        Err(e) => format!("Error executing block: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tanglit_exclude,
            tanglit_parse_slides,
            tanglit_parse_blocks,
            tanglit_execute_block
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
