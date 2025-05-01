use backend::parser::parse_blocks_from_file;
use std::fs::write;

const INPUT_FILE: &str = "input_file.txt";
const OUTPUT_FILE: &str = "output_file.txt";

fn main() {
    let blocks = match parse_blocks_from_file(INPUT_FILE) {
        Ok(blocks) => blocks,
        Err(e) => {
            println!("Error parsing blocks: {}", e);
            return;
        }
    };

    let output = blocks
        .iter()
        .map(|block| format!("{}\n", block))
        .collect::<String>();

    match write(OUTPUT_FILE, output) {
        Ok(_) => println!("Blocks written to {}", OUTPUT_FILE),
        Err(e) => println!("Error writing to file: {}", e),
    };
}
