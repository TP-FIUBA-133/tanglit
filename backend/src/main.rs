use backend::{cli::Cli, parser::parse_blocks_from_file, tangle::tangle_block};
use clap::Parser;
use std::{
    fs::write,
    path::{Path, PathBuf},
};

fn main() {
    let Cli { tangle_args } = Cli::parse();

    // Parse blocks from the input file
    let blocks = match parse_blocks_from_file(&tangle_args.input_file_path) {
        Ok(blocks) => blocks,
        Err(e) => {
            println!("Error parsing blocks: {}", e);
            return;
        }
    };

    // Tangle blocks
    let Ok(output) = tangle_block(&tangle_args.target_block, blocks)
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

// TODO: We should get the output file path based on the language
fn get_output_file_path(output_file_path: &str, main_block: &str) -> PathBuf {
    let output_file_path = Path::new(output_file_path);
    output_file_path.join(format!("{main_block}.c"))
}
