use std::io;

pub type Result<T> = std::result::Result<T, ValintaError>;

/// Errors for this crate
#[derive(Debug)]
pub enum ValintaError {
    /// Items cannot be empty
    ItemsCannotBeEmpty,
    /// The standard IO Error
    IoError(io::Error),
    /// Unable to find current shell width 
    TerminalSizeNotFound
}

impl core::fmt::Display for ValintaError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for ValintaError {}

