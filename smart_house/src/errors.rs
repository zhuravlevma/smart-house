use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetDataError {
    #[error("Not found data")]
    NotFound,
}
pub type GetDataResult<T> = Result<T, GetDataError>;

#[derive(Error, Debug)]
pub enum AddDataError {
    #[error("Unique constraint error")]
    UniqueConstraint,
}
pub type AddDataResult<T> = Result<T, AddDataError>;

#[derive(Debug, Error)]
pub enum RemoveDataError {
    #[error("Not found element")]
    NotFound,
}
pub type RemoveDataResult<T> = Result<T, RemoveDataError>;
