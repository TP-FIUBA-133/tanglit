use backend::cli::{Commands, ExcludeArgs, TangleArgs};
use backend::parser::code_block::Language;
use backend::parser::exclude::exclude_from_ast;
use backend::parser::parse_from_file;
use backend::util::{ast_to_markdown, read_file_and_parse_blocks};
use backend::{cli::Cli, execution, tangle::tangle_block};
use clap::Parser;
use std::{
    fs::write,
    path::{Path, PathBuf},
};

fn handle_tangle_command(tangle_args: TangleArgs) {
    // Parse blocks from the input file
    let blocks = match read_file_and_parse_blocks(&tangle_args.general.input_file_path) {
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("Error parsing blocks: {}", e);
            return;
        }
    };

    // Tangle blocks
    let (output, lang) = match tangle_block(&tangle_args.target_block, blocks, true) {
        Ok((output, lang)) => (output, lang),
        Err(e) => {
            eprintln!("Error tangling blocks: {}", e);
            return;
        }
    };
    // Write the output to a file
    let output_file_path =
        get_output_file_path(&tangle_args.output_dir, &tangle_args.target_block, lang);
    match write(&output_file_path, output) {
        Ok(_) => println!("Blocks written to {}", output_file_path.display()),
        Err(e) => eprintln!("Error writing to file: {}", e),
    };
}

fn handle_exclude_command(exclude_args: ExcludeArgs) {
    let input_file_path = exclude_args.general.input_file_path;
    let ast = parse_from_file(input_file_path.trim()).expect("Failed to parse");
    let ast_with_exclusions = exclude_from_ast(&ast);
    let output = ast_to_markdown(&ast_with_exclusions);
    // Write the output to a file
    match write(Path::new(&exclude_args.output_file_path), output) {
        Ok(_) => println!("Output written to {}", exclude_args.output_file_path),
        Err(e) => eprintln!("Error writing to file: {}", e),
    };
}

fn handle_execute_command(execute_args: backend::cli::ExecuteArgs) {
    let input_file_path = execute_args.general.input_file_path;

    let blocks = match read_file_and_parse_blocks(&input_file_path) {
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("Error parsing blocks: {}", e);
            return;
        }
    };

    let result = execution::execute(blocks, &execute_args.target_block);

    let ex_result = match result {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error executing block: {}", e);
            return;
        }
    };

    println!("Stdout: {}", ex_result.stdout);
    eprintln!("Stderr: {}", ex_result.stderr);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tangle(args) => {
            handle_tangle_command(args);
        }
        Commands::Exclude(args) => {
            handle_exclude_command(args);
        }
        Commands::Execute(args) => {
            handle_execute_command(args);
        }
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
