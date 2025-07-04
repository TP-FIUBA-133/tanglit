use backend::cli::{Commands, ExcludeArgs, TangleArgs};
use backend::execution::write_file;
use backend::parser::code_block::Language;
use backend::parser::exclude::exclude_from_markdown;
use backend::{cli::Cli, parser::parse_blocks_from_file, tangle::tangle_block};
use clap::Parser;
use std::{
    fs::write,
    path::{Path, PathBuf},
};

fn handle_tangle_command(tangle_args: TangleArgs) {
    // Parse blocks from the input file
    let blocks = match parse_blocks_from_file(&tangle_args.general.input_file_path) {
        Ok(blocks) => blocks,
        Err(e) => {
            println!("Error parsing blocks: {}", e);
            return;
        }
    };

    // Tangle blocks
    let Ok((output, lang)) = tangle_block(&tangle_args.target_block, blocks, true)
        .inspect_err(|e| println!("Error tangling blocks: {e}"))
    else {
        return;
    };

    // Write the output to a file
    let output_file_path =
        get_output_file_path(&tangle_args.output_dir, &tangle_args.target_block, lang);
    match write(&output_file_path, output) {
        Ok(_) => println!("Blocks written to {}", output_file_path.display()),
        Err(e) => println!("Error writing to file: {}", e),
    };
}

fn handle_exclude_command(exclude_args: ExcludeArgs) {
    let input = std::fs::read_to_string(&exclude_args.general.input_file_path)
        .expect("Failed to read input file");
    let ast_with_exclusions = exclude_from_markdown(input.as_str());
    let output = mdast_util_to_markdown::to_markdown(&ast_with_exclusions)
        .expect("Failed to convert to markdown");
    // Write the output to a file
    match write(Path::new(&exclude_args.output_file_path), output) {
        Ok(_) => println!("Output written to {}", exclude_args.output_file_path),
        Err(e) => eprintln!("Error writing to file: {}", e),
    };
}

fn handle_execute_command(execute_args: backend::cli::ExecuteArgs) {
    // Parse blocks from the input file
    let blocks = match parse_blocks_from_file(&execute_args.general.input_file_path) {
        Ok(blocks) => blocks,
        Err(e) => {
            println!("Error parsing blocks: {}", e);
            return;
        }
    };

    // Tangle blocks
    let Ok((output, lang)) = tangle_block(&execute_args.target_block, blocks, true)
        .inspect_err(|e| println!("Error tangling blocks: {e}"))
    else {
        return;
    };

    // Write the output to a file
    let Ok(block_file_path) =
        write_file(output, &execute_args.target_block, &lang).inspect_err(|e| {
            println!("Error writing to file: {e}");
        })
    else {
        return;
    };

    let handles = match lang {
        backend::parser::code_block::Language::C => {
            backend::execution::execute_c_file(block_file_path)
        }
        backend::parser::code_block::Language::Python => {
            backend::execution::execute_python_file(block_file_path)
        }
        _ => {
            println!("Unsupported language");
            return;
        }
    };

    println!("stdout:\n{}", String::from_utf8_lossy(&handles.stdout));
    eprintln!("stderr:\n{}", String::from_utf8_lossy(&handles.stderr));
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
