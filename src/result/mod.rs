pub enum GetDataError {
    NotFound,
}

pub enum AddDataError {
    UniqueConstraint,
}

pub enum RemoveDataError {
    NotFound,
}
