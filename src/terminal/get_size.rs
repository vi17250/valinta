use crate::error::{Result, ValintaError};
use crossterm::terminal::size;

pub fn get_width() -> Result<u16> {
    let size = size();
    match size {
        Ok(size) => Ok(size.0),
        Err(_) => Err(ValintaError::TerminalSizeNotFound),
    }
}
