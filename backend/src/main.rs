use clap::Parser;
use std::fs::{self, write};
use std::path::{Path, PathBuf};
use tanglit::cli::{
    Commands, GenerateDocArgs, GenerateSlidesMdArgs, GenerateSlidesPdfArgs, TangleAllArgs,
    TangleArgs,
};
use tanglit::configuration::init_configuration;
use tanglit::configuration::language_config::LanguageConfig;
use tanglit::doc::{TangleError, TanglitDoc};
use tanglit::errors::ExecutionError;
use tanglit::errors::ExecutionError::WriteError;
use tanglit::{cli::Cli, execution};

use env_logger::init;
use tanglit::execution::write_file;

fn handle_tangle_command(tangle_args: TangleArgs) -> Result<String, ExecutionError> {
    let input_file_path = tangle_args.input.in_file;
    let doc = TanglitDoc::new_from_file(&input_file_path)?;

    // Tangle the document
    let blocks = doc.get_code_blocks()?;
    let block = blocks
        .get_block(&tangle_args.target_block)
        .ok_or(TangleError::BlockNotFound(tangle_args.target_block.clone()))?;
    let output = blocks.tangle_codeblock(block)?;

    let lang = block.language.clone();

    // we can tangle even if we don't have a config for the language
    let lang_config = lang
        .as_deref()
        .and_then(|l| LanguageConfig::load_for_lang(l).ok());
    // we can tangle even if we don't have an extension
    let extension = lang_config.and_then(|cfg| cfg.extension);

    // Write the output to a file
    match write_file(
        output,
        &PathBuf::from(tangle_args.output.out_dir),
        &tangle_args.target_block,
        extension.as_deref(),
    ) {
        Ok(r) => Ok(format!("Blocks written to {}", r.display())),
        Err(e) => Err(WriteError(format!("Error writing to file: {}", e))),
    }
}

fn handle_execute_command(
    execute_args: tanglit::cli::ExecuteArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&execute_args.input.in_file)?;
    let output = execution::execute(&doc, &execute_args.target_block)?;
    Ok(format!(
        "Output of block {}:\n{}\nstderr: {}\nexit code: {}",
        execute_args.target_block,
        output.stdout,
        output.stderr,
        output.status.unwrap_or(-1)
    ))
}

fn handle_generate_html_command(
    generate_html_args: GenerateDocArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&generate_html_args.input.in_file)?;
    let html = doc.generate_html()?;

    match write(Path::new(&generate_html_args.output.out_file), html) {
        Ok(_) => Ok(format!(
            "✅ HTML saved to {}",
            generate_html_args.output.out_file
        )),
        Err(e) => Err(WriteError(format!("Error writing to file: {}", e))),
    }
}

fn handle_generate_pdf_command(
    generate_pdf_args: GenerateDocArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&generate_pdf_args.input.in_file)?;
    doc.generate_doc_pdf(&generate_pdf_args.output.out_file)?;

    Ok(format!(
        "✅ PDF saved to {}",
        &generate_pdf_args.output.out_file
    ))
}

fn handle_tangle_all_command(tangle_all_command: TangleAllArgs) -> Result<String, ExecutionError> {
    let input_file_path = &tangle_all_command.input.in_file;
    let doc = TanglitDoc::new_from_file(input_file_path)?;
    let blocks_processed = doc.generate_code_files(&tangle_all_command.output.out_dir)?;
    Ok(format!(
        "✅ {} blocks tangled to {}",
        blocks_processed, tangle_all_command.output.out_dir
    ))
}

fn handle_generate_md_slides(args: GenerateSlidesMdArgs) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&args.input.in_file)?;
    let slides_md = doc.generate_md_slides_vec()?;

    for (i, slide_md) in slides_md.iter().enumerate() {
        fs::write(format!("{}/slide_{}.md", args.output.out_dir, i), slide_md)?;
    }

    Ok("✅ Slides Generated".to_string())
}

fn handle_generate_slides_pdf(
    generate_slides_args: GenerateSlidesPdfArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&generate_slides_args.input.in_file)?;
    doc.generate_slides_pdf(&generate_slides_args.output.out_file)?;

    Ok(format!(
        "✅ Slides PDF saved to {}",
        &generate_slides_args.output.out_file
    ))
}

fn main() {
    init(); // Initialize the logger

    let cli = Cli::parse();

    if let Err(e) = init_configuration() {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    }

    // TODO: use a general error instead of ExecutionError
    let result = match cli.command {
        Commands::Tangle(args) => handle_tangle_command(args),
        Commands::Execute(args) => handle_execute_command(args),
        Commands::GeneratePDF(args) => handle_generate_pdf_command(args),
        Commands::GenerateHTML(args) => handle_generate_html_command(args),
        Commands::TangleAll(args) => handle_tangle_all_command(args),
        Commands::GenerateSlidesMd(args) => handle_generate_md_slides(args),
        Commands::GenerateSlidesPdf(args) => handle_generate_slides_pdf(args),
    };
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}
