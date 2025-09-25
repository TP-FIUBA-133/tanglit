use backend::configuration::init_configuration;
use backend::doc::{CodeBlock, SlideByIndex, TanglitDoc};
use serde::Serialize;

#[tauri::command(rename_all = "snake_case")]
fn tanglit_exclude(raw_markdown: &str) -> Result<String, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.exclude()
        .map_err(|e| format!("Error excluding content: {}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_parse_slides(raw_markdown: &str) -> Result<Vec<SlideByIndex>, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    Ok(doc.parse_slides_index())
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

#[derive(Debug, Clone, Serialize)]
struct ExecutionOutput {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_execute_block(raw_markdown: &str, block_name: &str) -> Result<ExecutionOutput, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;

    match backend::execution::execute(&doc, block_name) {
        Ok(output) => Ok(ExecutionOutput {
            status: output.status.code(),
            stdout: String::from_utf8(output.stdout).expect("Error reading stdout"),
            stderr: String::from_utf8(output.stderr).expect("Error reading stderr"),
        }),
        Err(e) => Err(format!("Error executing block: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_gen_slides(raw_markdown: &str) -> Result<Vec<String>, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    let slides = doc
        .generate_md_slides_vec()
        .map_err(|e| format!("Error generating Md slides: {}", e))?;
    Ok(slides)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_configuration().expect("Error initializing configuration");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tanglit_exclude,
            tanglit_parse_slides,
            tanglit_parse_blocks,
            tanglit_execute_block,
            tanglit_gen_slides
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
