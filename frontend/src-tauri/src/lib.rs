use backend::execution::ExecutionResult;
use backend::frontend_api;
use backend::frontend_api::TanglitInfo;

#[tauri::command(rename_all = "snake_case")]
fn tanglit_exclude(raw_markdown: &str) -> Result<String, String> {
    frontend_api::exclude(raw_markdown)
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_blocks_and_slides(raw_markdown: &str) -> Result<TanglitInfo, String> {
    frontend_api::parse_blocks_and_slides(raw_markdown)
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_execute_block(raw_markdown: &str, block_tag: &str) -> Result<ExecutionResult, String> {
    frontend_api::execute_block(raw_markdown, block_tag)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tanglit_exclude,
            tanglit_parse_blocks_and_slides,
            tanglit_execute_block
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
