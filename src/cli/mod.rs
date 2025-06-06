pub mod args;

use clap::{Parser, Subcommand};
use args::{EncArgs, RecordArgs};

#[derive(Parser, Debug)]
#[command(name = "elif", version, about = "Your CLI helper for faster, simpler commands")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encrypt or decrypt a file
    Enc(EncArgs),

    /// Record audio or video
    Record(RecordArgs),
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
