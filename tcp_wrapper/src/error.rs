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

pub type SendResult = Result<(), SendError>;
#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
pub type ReceiveResult = Result<String, ReceiveError>;
#[derive(Debug, Error)]
pub enum ReceiveError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("bad encoding")]
    BadEncoding,
}

pub type RequestResult = Result<String, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] ReceiveError),
    #[error("bad encoding")]
    BadEncoding,
}
