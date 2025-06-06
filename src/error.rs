use thiserror::Error;
use std::io;
use std::process::ExitStatus;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ElifError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Command failed with exit code: {0}")]
    CommandFailed(ExitStatus),

    #[error("UTF-8 parsing error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, ElifError>;
