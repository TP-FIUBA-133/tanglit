use backend::cli::{Commands, GeneratePDFArgs, GenerateSlidesMdArgs, TangleArgs};
use backend::configuration::init_configuration;
use backend::configuration::language_config::LanguageConfig;
use backend::doc::{TangleError, TanglitDoc};
use backend::errors::ExecutionError;
use backend::errors::ExecutionError::WriteError;
use backend::{cli::Cli, execution};
use clap::Parser;
use std::path::PathBuf;

use backend::execution::write_file;
use env_logger::init;

fn handle_tangle_command(tangle_args: TangleArgs) -> Result<String, ExecutionError> {
    let input_file_path = tangle_args.general.input_file_path;
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
        &PathBuf::from(tangle_args.output_dir),
        &tangle_args.target_block,
        extension.as_deref(),
    ) {
        Ok(r) => Ok(format!("Blocks written to {}", r.display())),
        Err(e) => Err(WriteError(format!("Error writing to file: {}", e))),
    }
}

fn handle_execute_command(
    execute_args: backend::cli::ExecuteArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&execute_args.general.input_file_path)?;
    let output = execution::execute(&doc, &execute_args.target_block)?;
    Ok(format!(
        "Output of block {}:\n{}\nstderr: {}\nexit code: {}",
        execute_args.target_block,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        &output.status
    ))
}

fn handle_generate_pdf_command(
    generate_pdf_args: GeneratePDFArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&generate_pdf_args.general.input_file_path)?;
    doc.generate_pdf(&generate_pdf_args.output_file_path)?;

    Ok(format!(
        "✅ PDF saved to {}",
        &generate_pdf_args.output_file_path
    ))
}

fn handle_generate_md_slides(args: GenerateSlidesMdArgs) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&args.general.input_file_path)?;
    doc.generate_md_slides(args.output_dir)?;

    Ok("✅ Slides Generated".to_string())
}

fn main() {
    init(); // Initialize the logger

    let cli = Cli::parse();

    if let Err(e) = init_configuration() {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    }

    let result = match cli.command {
        Commands::Tangle(args) => handle_tangle_command(args),
        Commands::Execute(args) => handle_execute_command(args),
        Commands::GeneratePDF(args) => handle_generate_pdf_command(args),
        Commands::GenerateSlidesMd(args) => handle_generate_md_slides(args),
    };
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}
