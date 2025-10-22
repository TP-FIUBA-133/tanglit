use backend::configuration::init_configuration;
use backend::doc::{CodeBlock, Edit, SlideByIndex, TanglitDoc};
use backend::execution::ExecutionOutput;

#[tauri::command(rename_all = "snake_case")]
fn tanglit_exclude(raw_markdown: &str) -> Result<String, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.filter_content_for_doc()
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

#[tauri::command(rename_all = "snake_case")]
fn tanglit_execute_block(raw_markdown: &str, block_name: &str) -> Result<ExecutionOutput, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;

    backend::execution::execute(&doc, block_name)
        .map_err(|e| format!("Error executing block: {}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_format_output(
    raw_markdown: &str,
    block_name: &str,
    output: ExecutionOutput,
) -> Result<Edit, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.format_output(block_name, &output)
        .map_err(|e| format!("Error formatting output: {}", e))
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

#[tauri::command(rename_all = "snake_case")]
fn tanglit_preview_html(raw_markdown: &str, theme: &str) -> Result<String, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.generate_html(theme)
        .map_err(|e| format!("Error generating HTML: {}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_save_pdf(raw_markdown: &str, theme: &str, output_path: &str) -> Result<(), String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.generate_doc_pdf(output_path, theme)
        .map_err(|e| format!("Error generating PDF: {}", e))
}

#[tauri::command(rename_all = "snake_case")]
fn tanglit_tangle(raw_markdown: &str, output_path: &str) -> Result<usize, String> {
    let doc = TanglitDoc::new_from_string(raw_markdown)
        .map_err(|e| format!("Error creating TanglitDoc: {}", e))?;
    doc.generate_code_files(output_path)
        .map_err(|e| format!("Error tangling code: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_configuration().expect("Error initializing configuration");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tanglit_exclude,
            tanglit_parse_slides,
            tanglit_parse_blocks,
            tanglit_execute_block,
            tanglit_format_output,
            tanglit_gen_slides,
            tanglit_preview_html,
            tanglit_save_pdf,
            tanglit_tangle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
