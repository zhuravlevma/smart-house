use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetDataError {
    #[error("Not found data")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum AddDataError {
    #[error("Unique constraint error")]
    UniqueConstraint,
}

#[derive(Debug, Error)]
pub enum RemoveDataError {
    #[error("Not found element")]
    NotFound,
}
