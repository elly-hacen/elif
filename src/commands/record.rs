use crate::log;
use crate::cli::args::MediaType;


pub fn handle_record(media_type: MediaType, output: String) {
    match media_type {
        MediaType::Video => {
            log!("Recording video to file: {}", output);
        }
        MediaType::Audio => {
            log!("Recording audio to file: {}", output);
        }
    }
}
