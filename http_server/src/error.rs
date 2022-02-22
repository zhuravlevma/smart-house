use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CustomError {
    #[error("Endpoint is not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum DomainError {
    #[error("HouseError")]
    HouseError,
    #[error("ApartmentError")]
    ApartmentError,
    #[error("RosetteError")]
    RosetteError,
    #[error("ThermometerError")]
    ThermometerError,
    #[error("CustomError: {0}")]
    CustomError(#[from] CustomError),
}

impl actix_web::error::ResponseError for DomainError {}

impl From<mongodb::error::Error> for CustomError {
    fn from(source: mongodb::error::Error) -> Self {
        Self::InternalError(source.to_string())
    }
}

impl From<mongodb::bson::de::Error> for CustomError {
    fn from(source: mongodb::bson::de::Error) -> Self {
        Self::InternalError(source.to_string())
    }
}

impl From<mongodb::bson::ser::Error> for CustomError {
    fn from(source: mongodb::bson::ser::Error) -> Self {
        Self::InternalError(source.to_string())
    }
}

impl From<mongodb::bson::oid::Error> for CustomError {
    fn from(source: mongodb::bson::oid::Error) -> Self {
        Self::NotFound(source.to_string())
    }
}
