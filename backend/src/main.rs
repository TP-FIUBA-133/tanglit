use backend::parser::input_to_mdast;
use backend::{parser::parse_blocks_from_file, parser::slides::get_slides, tangle::tangle_blocks};
use std::fs::write;

const INPUT_FILE: &str = "./test_data/test_file.md";
const OUTPUT_FILE: &str = "./test_data/output_file.rs";

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read file");
    let ast = input_to_mdast(&input).expect("Failed to parse input");
    let _slides = get_slides(&ast, input.as_str());

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
