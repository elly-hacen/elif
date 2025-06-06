use clap::Args;

#[derive(Args, Debug)]
pub struct EncArgs {
    /// Decrypt a file
    #[arg(short, long)]
    pub decrypt: bool,

    /// The file path
    pub file: String,

    /// The password to use
    pub password: String,
}

#[derive(Args, Debug)]
pub struct RecordArgs {
    /// video or audio
    #[arg(value_enum)]
    pub media_type: MediaType,

    /// Output file name
    pub output: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum MediaType {
    Video,
    Audio,
}
