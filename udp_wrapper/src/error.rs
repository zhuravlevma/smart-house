use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Convert error: {0}")]
    ConvertError(#[from] std::string::FromUtf8Error),
    #[error("Receive error: {0}")]
    ReceiveError(#[from] ReceiveError),
}

#[derive(Debug, Error)]
pub enum ReceiveError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Convert error: {0}")]
    ConvertError(#[from] std::string::FromUtf8Error),
}
