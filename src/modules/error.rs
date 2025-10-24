use std::io;

pub type Result<T> = std::result::Result<T, ValintaError>;

#[derive(Debug)]
pub enum ValintaError {
    Custom(String),
    IoError(io::Error),
}

impl core::fmt::Display for ValintaError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for ValintaError {}

impl From<io::Error> for ValintaError {
    fn from(err: io::Error) -> Self {
        ValintaError::IoError(err)
    }
}
