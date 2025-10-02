use backend::cli::{
    Commands, ExcludeArgs, GenerateDocArgs, GenerateSlidesMdArgs, TangleAllArgs, TangleArgs,
};
use backend::configuration::language_config::LanguageConfig;
use backend::configuration::{get_config_for_lang, init_configuration};
use backend::doc::{TangleError, TanglitDoc};
use backend::errors::ExecutionError;
use backend::errors::ExecutionError::WriteError;
use backend::{cli::Cli, execution};
use clap::Parser;
use std::{
    fs::write,
    path::{Path, PathBuf},
};

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

fn handle_exclude_command(exclude_args: ExcludeArgs) -> Result<String, ExecutionError> {
    let input_file_path = exclude_args.general.input_file_path;
    let doc = TanglitDoc::new_from_file(&input_file_path)?;
    let output = doc.exclude()?;

    // Write the output to a file
    match write(Path::new(&exclude_args.output_file_path), output) {
        Ok(_) => Ok(format!(
            "Blocks written to {}",
            exclude_args.output_file_path
        )),
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

fn handle_generate_html_command(
    generate_html_args: GenerateDocArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&generate_html_args.general.input_file_path)?;
    let html = doc.generate_html()?;

    match write(Path::new(&generate_html_args.output_file_path), html) {
        Ok(_) => Ok(format!(
            "✅ HTML saved to {}",
            generate_html_args.output_file_path
        )),
        Err(e) => Err(WriteError(format!("Error writing to file: {}", e))),
    }
}

fn handle_generate_pdf_command(
    generate_pdf_args: GenerateDocArgs,
) -> Result<String, ExecutionError> {
    let doc = TanglitDoc::new_from_file(&generate_pdf_args.general.input_file_path)?;
    doc.generate_pdf(&generate_pdf_args.output_file_path)?;

    Ok(format!(
        "✅ PDF saved to {}",
        &generate_pdf_args.output_file_path
    ))
}

fn handle_tangle_all_command(tangle_all_command: TangleAllArgs) -> Result<String, ExecutionError> {
    let input_file_path = &tangle_all_command.general.input_file_path;
    let doc = TanglitDoc::new_from_file(input_file_path)?;

    let blocks = doc.get_code_blocks()?;
    let blocks_to_tangle = blocks.get_all_blocks_to_tangle();

    for block in &blocks_to_tangle {
        let output = blocks.tangle_codeblock(block)?;

        let lang = block.language.as_deref();
        let extension = lang
            .and_then(|l| get_config_for_lang(l).ok())
            .and_then(|cfg| cfg.extension);

        let file_name = block.export.clone().unwrap_or(block.tag.clone());

        write_file(
            output,
            &PathBuf::from(&tangle_all_command.output_dir),
            &file_name,
            extension.as_deref(),
        )
        .map_err(|e| WriteError(format!("Error writing to file: {}", e)))?;
    }

    Ok(format!(
        "All {} blocks tangled to {}",
        blocks_to_tangle.len(),
        tangle_all_command.output_dir
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
        Commands::Exclude(args) => handle_exclude_command(args),
        Commands::Execute(args) => handle_execute_command(args),
        Commands::GeneratePDF(args) => handle_generate_pdf_command(args),
        Commands::GenerateHTML(args) => handle_generate_html_command(args),
        Commands::TangleAll(args) => handle_tangle_all_command(args),
        Commands::GenerateSlidesMd(args) => handle_generate_md_slides(args),
    };
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}
