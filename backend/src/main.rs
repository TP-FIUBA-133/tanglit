use backend::{
    parser::exclude::exclude_from_markdown, parser::parse_blocks_from_file, tangle::tangle_blocks,
};
use std::fs::write;

const INPUT_FILE: &str = "./test_data/test_file.md";
const OUTPUT_FILE: &str = "./test_data/output_file.rs";

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file");
    let ast_with_exclusions = exclude_from_markdown(input.as_str());
    let output = mdast_util_to_markdown::to_markdown(&ast_with_exclusions)
        .expect("Failed to convert to markdown");
    // Write the output to a file
    match write(OUTPUT_FILE, output) {
        Ok(_) => println!("Output written to {}", OUTPUT_FILE),
        Err(e) => println!("Error writing to file: {}", e),
    };

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
