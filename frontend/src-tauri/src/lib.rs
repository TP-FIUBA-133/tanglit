use backend::doc::{CodeBlock, Slide, TanglitDoc};

#[tauri::command(rename_all = "snake_case")]
fn tanglit_exclude(raw_markdown: &str) -> Result<String, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.exclude()
        .map_err(|e| format!("Error excluding content: {}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_slides(raw_markdown: &str) -> Result<Vec<Slide>, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    Ok(doc.parse_slides())
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_blocks(raw_markdown: &str) -> Result<Vec<CodeBlock>, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    let code_blocks = doc
        .get_code_blocks()
        .map_err(|e| format!("Error parsing blocks: {}", e))?;
    let blocks = code_blocks.blocks.values().cloned().collect();
    Ok(blocks)
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_execute_block(raw_markdown: &str, block_name: &str) -> Result<String, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;

    match backend::execution::execute(&doc, block_name) {
        Ok(output) => Ok(format!("{output:?}")),
        Err(e) => Ok(format!("Error executing block: {}", e)),
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
