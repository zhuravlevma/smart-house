use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetDataError {
    #[error("Not found data")]
    NotFound,
}
pub type GetDataResult<T> = Result<T, GetDataError>;

#[derive(Debug, Error)]
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

// #[derive(Debug, Error)]
// pub enum DeviceError {
//     #[error("Resource busy at the moment")]
//     ThermometerError(#[from] ThermometerError)
// }

#[derive(Debug, Error)]
pub enum ThermometerError {
    #[error("Resource busy at the moment: ip `{0}`")]
    Busy(String),
    #[error("Receiving finish with error")]
    ReceiveError(#[from] udp_wrapper::error::ReceiveError),
    #[error("Thermometer binding with error")]
    BindError(#[from] udp_wrapper::error::BindError),
    #[error("Parsing temperature to float finish with error: `{0}`")]
    ParseFloatError(String),
}

#[derive(Debug, Error)]
pub enum RosetteError {
    #[error("Resource busy at the moment")]
    ServerNotFound(#[from] tcp_wrapper::error::ConnectError),
    #[error("Getting data for rosette finish with error")]
    GetDataError(#[from] tcp_wrapper::error::RequestError),
    #[error("Parsing power to int finish with error `{0}`")]
    ParsePowerError(String),
}
