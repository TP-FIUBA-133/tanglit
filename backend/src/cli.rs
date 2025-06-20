use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Tangle a code block from a markdown file
    Tangle(TangleArgs),
    // Exclude marked parts from the markdown 
    Exclude(ExcludeArgs),
}

#[derive(Args)]
pub struct TangleArgs {
    #[arg(
        long,
        value_name = "INPUT_FILE_PATH",
        help = "Path to the input markdown file.",
        help_heading = "Tangle Args",
        env = "INPUT_FILE_PATH"
    )]
    pub input_file_path: String,
    #[arg(
        long,
        value_name = "OUTPUT_DIR",
        help = "Path to the directory where the output file will be saved.",
        help_heading = "Tangle Args",
        env = "OUTPUT_DIR"
    )]
    pub output_dir: String,
    #[arg(
        long,
        value_name = "TARGET_BLOCK",
        help = "Tag of the code block to tangle.",
        help_heading = "Tangle Args",
        env = "TARGET_BLOCK"
    )]
    pub target_block: String,
}

#[derive(Args)]
pub struct ExcludeArgs {
    #[arg(
        long,
        value_name = "INPUT_FILE_PATH",
        help = "Path to the input markdown file.",
        help_heading = "Tangle Args",
        env = "INPUT_FILE_PATH"
    )]
    pub input_file_path: String,
    #[arg(
        long,
        value_name = "OUTPUT_FILE_PATH",
        help = "Path to the file where the output will be saved.",
        help_heading = "Tangle Args",
        env = "OUTPUT_DIR"
    )]
    pub output_file_path: String,
}
