use backend::{parser::parse_blocks_from_file, tangle::tangle_blocks};
use std::{env, fs::write};

const INPUT_FILE: &str = "./test_data/test_file.md";
const OUTPUT_FILE: &str = "./test_data/output_file.rs";

fn main() {
    match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current dir: {}", e),
    }

    // Parse blocks from the input file
    let blocks = match parse_blocks_from_file(INPUT_FILE) {
        Ok(blocks) => blocks,
        Err(e) => {
            println!("Error parsing blocks: {}", e);
            return;
        }
    };

    // Tangle blocks
    let output = tangle_blocks(blocks);

    // Write the output to a file
    match write(OUTPUT_FILE, output) {
        Ok(_) => println!("Blocks written to {}", OUTPUT_FILE),
        Err(e) => println!("Error writing to file: {}", e),
    };
}
