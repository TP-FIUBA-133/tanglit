use backend::cli::{Commands, ExcludeArgs, TangleArgs};
use backend::doc::{Language, TangleError, TanglitDoc};
use backend::errors::ExecutionError;
use backend::errors::ExecutionError::WriteError;
use backend::{cli::Cli, execution};
use clap::Parser;
use std::{
    fs::write,
    path::{Path, PathBuf},
};

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
