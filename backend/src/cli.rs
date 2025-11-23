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
    #[command(about = "Tangle a specific code block from a markdown file and export to a file")]
    Tangle(TangleArgs),
    #[command(about = "Execute a specific code block from a markdown file and read its output")]
    Execute(ExecuteArgs),
    #[command(about = "Tangle and export all marked code blocks from a markdown file")]
    TangleAll(TangleAllArgs),
    #[command(about = "Generates a PDF from an markdown file, skipping the items with % markers")]
    GeneratePDF(GenerateDocArgs),
    #[command(about = "Generates an HTML from an markdown file, skipping the items with % markers")]
    GenerateHTML(GenerateDocArgs),
    #[command(about = "Generates markdown slides from a markdown file")]
    GenerateSlidesMd(GenerateSlidesMdArgs),
    #[command(about = "Generates a PDF with slides from a markdown file")]
    GenerateSlidesPdf(GenerateSlidesPdfArgs),
}

#[derive(Args, Debug)]
pub struct InputFileArg {
    #[arg(
        index = 1,
        value_name = "INPUT_FILE_PATH",
        help = "Path to the input markdown file.",
        help_heading = "General Args",
        env = "INPUT_FILE_PATH"
    )]
    pub in_file: String,
}

#[derive(Args, Debug)]
pub struct OutputDirArg {
    #[arg(
        long("output-dir"),
        short('o'),
        value_name = "OUTPUT_DIR",
        help = "Path to the directory where the output files will be saved.",
        env = "OUTPUT_DIR",
        help_heading = "General Args"
    )]
    pub out_dir: String,
}

#[derive(Args, Debug)]
pub struct OutputFileArg {
    #[arg(
        long("output-file"),
        short('o'),
        value_name = "OUTPUT_FILE_PATH",
        help = "Path to the file where the output will be saved.",
        help_heading = "General Args",
        env = "OUTPUT_FILE_PATH"
    )]
    pub out_file: String,
}

#[derive(Args)]
pub struct TangleArgs {
    #[command(flatten)]
    pub input: InputFileArg,
    #[command(flatten)]
    pub output: OutputDirArg,
    #[arg(
        long,
        short,
        value_name = "TARGET_BLOCK",
        help = "Tag of the code block to tangle.",
        help_heading = "Tangle Args",
        env = "TARGET_BLOCK"
    )]
    pub target_block: String,
}

#[derive(Args)]
pub struct ExecuteArgs {
    #[command(flatten)]
    pub input: InputFileArg,
    #[arg(
        long,
        short,
        value_name = "TARGET_BLOCK",
        help = "Tag of the code block to execute.",
        help_heading = "Execute Args",
        env = "TARGET_BLOCK"
    )]
    pub target_block: String,
}

#[derive(Args)]
pub struct GenerateDocArgs {
    #[command(flatten)]
    pub input: InputFileArg,
    #[command(flatten)]
    pub output: OutputFileArg,
    #[arg(
        long,
        short,
        value_name = "THEME",
        help = "Theme to use for the generated document ('pico', 'water', 'sakura' or 'latex'; the default is 'pico').",
        help_heading = "Document Generation Args",
        env = "THEME"
    )]
    pub theme: Option<String>,
}

#[derive(Args)]
pub struct TangleAllArgs {
    #[command(flatten)]
    pub input: InputFileArg,
    #[command(flatten)]
    pub output: OutputDirArg,
}

#[derive(Args)]
pub struct GenerateSlidesMdArgs {
    #[command(flatten)]
    pub input: InputFileArg,
    #[command(flatten)]
    pub output: OutputDirArg,
}

#[derive(Args)]
pub struct GenerateSlidesPdfArgs {
    #[command(flatten)]
    pub input: InputFileArg,
    #[command(flatten)]
    pub output: OutputFileArg,
    #[arg(
        long,
        short,
        value_name = "THEME",
        help = "Theme to use for the generated slides ('black', 'white', 'league', 'beige', 'sky', 'night', 'solarized', ...)",
        help_heading = "Slide Generation Args",
        env = "THEME",
        default_value = "black"
    )]
    pub theme: String,
}
