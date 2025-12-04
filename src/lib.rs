//! Easy and zero config multiselect in terminal

// Internal modules
mod def;
mod terminal;
mod utils;
mod usekey;

// Exposed and flattenned modules
mod select; 
mod error;
pub use select::select;
pub use error::ValintaError;
