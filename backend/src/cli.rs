use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tangle code blocks from a file
    Tangle(TangleArgs),
}

#[derive(Args)]
struct TangleArgs {
    file_path: String,
    main_block: String,
}
