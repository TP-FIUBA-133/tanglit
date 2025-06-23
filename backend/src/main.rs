use backend::cli::{Commands, ExcludeArgs, TangleArgs};
use backend::parser::exclude::exclude_from_markdown;
use backend::tangle as tg;
use backend::{cli::Cli, parser::parse_blocks_from_file, tangle::tangle_block};
use clap::Parser;
use std::fs;
use std::process::{Command, Stdio};
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
    let Ok(output) = tangle_block(&tangle_args.target_block, &blocks)
        .inspect_err(|e| println!("Error tangling blocks: {e}"))
    else {
        return;
    };

    // Write the output to a file
    let output_file_path = get_output_file_path(&tangle_args.output_dir, &tangle_args.target_block);
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
    let Ok(output) = tg::tangle_execute_block(&execute_args.target_block, &blocks)
        .inspect_err(|e| println!("Error tangling blocks: {e}"))
    else {
        return;
    };

    // Step 1: Compile the C program
    // Create a temporary directory in the current working directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let tmp_dir = current_dir.join("tmp");
    if !tmp_dir.exists() {
        fs::create_dir_all(&tmp_dir).expect("Failed to create temp directory");
    }

    // Create the C file path using the target block name
    let c_file = tmp_dir.join(format!("{}.c", execute_args.target_block));
    let output_binary = tmp_dir.join(format!("{}", execute_args.target_block));

    // Write the tangled output to the C file
    fs::write(&c_file, output).expect("Failed to write C file");

    // Compile the C program
    let compile_status = Command::new("gcc")
        .arg(&c_file)
        .arg("-o")
        .arg(&output_binary)
        .status()
        .expect("Failed to start gcc");

    if !compile_status.success() {
        eprintln!("Failed to compile C program.");
        std::process::exit(1);
    }

    // Step 2: Run the compiled C binary and capture output
    let output = Command::new(output_binary)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute C program");

    println!("C stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("C stderr:\n{}", String::from_utf8_lossy(&output.stderr));
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
fn get_output_file_path(output_file_path: &str, main_block: &str) -> PathBuf {
    let output_file_path = Path::new(output_file_path);
    output_file_path.join(format!("{main_block}.c"))
}
