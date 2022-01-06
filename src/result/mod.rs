use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum GetDataError {
    NotFound,
}

pub enum AddDataError {
    UniqueConstraint,
}

impl Debug for AddDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddDataError: Unique Error")
    }
}

impl Display for AddDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Add data error")
    }
}

impl Error for AddDataError {}

impl Debug for GetDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Get data error")
    }
}

impl Display for GetDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Get data error")
    }
}

impl Error for GetDataError {}

pub enum RemoveDataError {
    NotFound,
}

impl Debug for RemoveDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remove data error")
    }
}

impl Display for RemoveDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remove data error")
    }
}

impl Error for RemoveDataError {}
