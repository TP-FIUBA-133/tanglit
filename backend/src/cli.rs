use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(flatten)]
    pub tangle_args: TangleArgs,
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
