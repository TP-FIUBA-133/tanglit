use backend::{parser::parse_blocks_from_file, tangle::tangle_block};
use std::fs::write;

const INPUT_FILE: &str = "./test_data/test_file_2.md";
const OUTPUT_FILE: &str = "./test_data/output_file_2.c";

fn main() {
    // Parse blocks from the input file
    let blocks = match parse_blocks_from_file(INPUT_FILE) {
        Ok(blocks) => blocks,
        Err(e) => {
            println!("Error parsing blocks: {}", e);
            return;
        }
    };

    // Tangle blocks
    let Ok(output) = tangle_block(String::from("print"), &blocks)
        .inspect_err(|e| println!("Error tangling blocks: {e}"))
    else {
        return;
    };

    // Write the output to a file
    match write(OUTPUT_FILE, output) {
        Ok(_) => println!("Blocks written to {}", OUTPUT_FILE),
        Err(e) => println!("Error writing to file: {}", e),
    };
}
