use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Unexpected handshake response: {0}")]
    BadHandshake(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
pub type ConnectResult<T> = Result<T, ConnectError>;

#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
