use backend::parser::code_block::CodeBlock;
use backend::parser::input_to_mdast;
use backend::parser::slides::Slide;
use std::string::ParseError;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
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

// struct CodeInfo {
//     pub language: String,
//     pub code: String,
//     pub tag: Option<String>,
// pub imports: Vec<String>
// }
#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_blocks(raw_markdown: &str) -> Vec<CodeBlock> {
    let mdast = input_to_mdast(raw_markdown).expect("Failed to parse input to mdast");
    let rv = backend::parser::parse_input(raw_markdown.to_string()).unwrap_or_else(|_| vec![]);

    rv
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tanglit_exclude,
            tanglit_parse_slides,
            tanglit_parse_blocks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
