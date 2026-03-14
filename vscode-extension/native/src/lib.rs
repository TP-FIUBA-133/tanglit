use napi::bindgen_prelude::*;
use napi_derive::napi;
use tanglit::configuration::init_configuration as tanglit_init_configuration;
use tanglit::doc::TanglitDoc;
use tanglit::execution::ExecutionOutput as TanglitExecutionOutput;

#[napi(object)]
pub struct CodeBlock {
    pub tag: String,
    pub language: Option<String>,
    pub code: String,
    pub imports: Vec<String>,
    pub export: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
}

#[napi(object)]
pub struct SlideByIndex {
    pub start_line: u32,
}

#[napi(object)]
#[derive(Clone)]
pub struct ExecutionOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
}

#[napi(object)]
pub struct Edit {
    pub content: String,
    pub start_line: u32,
    pub end_line: u32,
}

#[napi]
pub fn init_configuration() -> Result<()> {
    tanglit_init_configuration().map_err(|e| Error::from_reason(format!("Configuration error: {}", e)))
}

#[napi]
pub fn parse_blocks(raw_markdown: String) -> Result<Vec<CodeBlock>> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    let code_blocks = doc
        .get_code_blocks()
        .map_err(|e| Error::from_reason(format!("Error parsing blocks: {}", e)))?;
    let blocks = code_blocks
        .blocks
        .values()
        .map(|b| CodeBlock {
            tag: b.tag.clone(),
            language: b.language.clone(),
            code: b.code.clone(),
            imports: b.imports.clone(),
            export: b.export.clone(),
            start_line: b.start_line as u32,
            end_line: b.end_line as u32,
        })
        .collect();
    Ok(blocks)
}

#[napi]
pub fn parse_slides(raw_markdown: String) -> Result<Vec<SlideByIndex>> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    let slides = doc.parse_slides_index();
    // SlideByIndex fields are private, so we use serde to extract start_line
    let slides: Vec<SlideByIndex> = slides
        .iter()
        .map(|s| {
            let json = serde_json::to_value(s).unwrap();
            SlideByIndex {
                start_line: json["start_line"].as_u64().unwrap_or(0) as u32,
            }
        })
        .collect();
    Ok(slides)
}

#[napi]
pub fn execute_block(raw_markdown: String, block_name: String) -> Result<ExecutionOutput> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    let output = tanglit::execution::execute(&doc, &block_name)
        .map_err(|e| Error::from_reason(format!("Execution error: {}", e)))?;
    Ok(ExecutionOutput {
        stdout: output.stdout,
        stderr: output.stderr,
        status: output.status,
    })
}

#[napi]
pub fn format_output(
    raw_markdown: String,
    block_name: String,
    output: ExecutionOutput,
) -> Result<Edit> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    let tanglit_output = TanglitExecutionOutput {
        stdout: output.stdout,
        stderr: output.stderr,
        status: output.status,
    };
    let edit = doc
        .format_output(&block_name, &tanglit_output)
        .map_err(|e| Error::from_reason(format!("Format error: {}", e)))?;
    Ok(Edit {
        content: edit.content,
        start_line: edit.start_line as u32,
        end_line: edit.end_line as u32,
    })
}

#[napi]
pub fn preview_html(raw_markdown: String, theme: String) -> Result<String> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    doc.generate_html(&theme)
        .map_err(|e| Error::from_reason(format!("HTML generation error: {}", e)))
}

#[napi]
pub fn preview_slides(
    raw_markdown: String,
    theme: String,
    code_theme: String,
) -> Result<String> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    doc.generate_slides_html(&theme, &code_theme)
        .map_err(|e| Error::from_reason(format!("Slides generation error: {}", e)))
}

#[napi]
pub fn tangle(raw_markdown: String, output_path: String) -> Result<u32> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    let count = doc
        .generate_code_files(&output_path)
        .map_err(|e| Error::from_reason(format!("Tangle error: {}", e)))?;
    Ok(count as u32)
}

#[napi]
pub fn exclude(raw_markdown: String) -> Result<String> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    doc.filter_content_for_doc()
        .map_err(|e| Error::from_reason(format!("Exclude error: {}", e)))
}

#[napi]
pub fn save_pdf(raw_markdown: String, theme: String, output_path: String) -> Result<()> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    doc.generate_doc_pdf(&output_path, &theme)
        .map_err(|e| Error::from_reason(format!("PDF generation error: {}", e)))
}

#[napi]
pub fn save_slides_pdf(
    raw_markdown: String,
    theme: String,
    code_theme: String,
    output_path: String,
) -> Result<()> {
    let doc = TanglitDoc::new_from_string(&raw_markdown)
        .map_err(|e| Error::from_reason(format!("Parse error: {}", e)))?;
    doc.generate_slides_pdf(&output_path, &theme, &code_theme)
        .map_err(|e| Error::from_reason(format!("Slides PDF generation error: {}", e)))
}
