use backend::cli::{Commands, ExcludeArgs, TangleArgs};
use backend::errors::ExecutionError;
use backend::errors::ExecutionError::WriteError;
use backend::parser::code_block::Language;
use backend::parser::exclude::exclude_from_ast;
use backend::parser::{ast_to_markdown, parse_from_file};
use backend::util::read_file_and_parse_blocks;
use backend::{cli::Cli, execution, tangle::tangle_block};
use clap::Parser;
use std::{
    fs::write,
    path::{Path, PathBuf},
};

fn handle_tangle_command(tangle_args: TangleArgs) -> Result<String, ExecutionError> {
    // Parse blocks from the input file
    let blocks = read_file_and_parse_blocks(&tangle_args.general.input_file_path)?;
    // Tangle blocks
    let (output, lang) = tangle_block(&tangle_args.target_block, blocks, true)?;
    // Write the output to a file
    let output_file_path =
        get_output_file_path(&tangle_args.output_dir, &tangle_args.target_block, lang);
    match write(&output_file_path, output) {
        Ok(_) => Ok(format!("Blocks written to {}", output_file_path.display())),
        Err(e) => Err(WriteError(format!("Error writing to file: {}", e))),
    }
}

fn handle_exclude_command(exclude_args: ExcludeArgs) -> Result<String, ExecutionError> {
    let input_file_path = exclude_args.general.input_file_path;
    let ast = parse_from_file(input_file_path.trim()).expect("Failed to parse");
    let ast_with_exclusions = exclude_from_ast(&ast);
    let output = ast_to_markdown(&ast_with_exclusions)?;

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
    let output = execution::execute(
        &execute_args.general.input_file_path,
        &execute_args.target_block,
    )?;
    Ok(format!(
        "Output of block {}:\n{}\nstderr: {}\nexit code: {}",
        execute_args.target_block,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        &output.status
    ))
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Tangle(args) => handle_tangle_command(args),
        Commands::Exclude(args) => handle_exclude_command(args),
        Commands::Execute(args) => handle_execute_command(args),
    };
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// TODO: We should get the output file path based on the language
fn get_output_file_path(output_file_path: &str, main_block: &str, lang: Language) -> PathBuf {
    let output_file_path = Path::new(output_file_path);
    match lang {
        Language::C => output_file_path.join(format!("{main_block}.c")),
        Language::Python => output_file_path.join(format!("{main_block}.py")),
        _ => output_file_path.join(format!("{main_block}.txt")),
    }
}
