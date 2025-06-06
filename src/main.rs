mod cli;
mod commands;
#[macro_use]
mod macros;
mod error;

use cli::{parse_args, Commands};
use commands::{enc::handle_enc, record::handle_record};

fn main() -> error::Result<()> {
    let args = parse_args();

    match args.command {
    Commands::Enc(enc_args) => {
        handle_enc(enc_args.decrypt, enc_args.file, enc_args.password);
    }
    Commands::Record(record_args) => {
        handle_record(record_args.media_type, record_args.output);
    }
}

    Ok(())
}
