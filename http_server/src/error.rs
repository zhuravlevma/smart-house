use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CustomError {
    #[error("Endpoint is not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

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