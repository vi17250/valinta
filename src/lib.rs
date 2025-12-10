#![warn(missing_docs)]
//! This crate provide an easy and zero config multiple selector in terminal.
//!
//! # Usage
//!
//! ## First: add this crate to your project
//!
//! ```bash
//! cargo add valinta
//! ```
//!
//! ## Next:
//!
//! ```
//! use valinta::select;
//!
//! fn main() -> std::io::Result<()> {
//!
//!     let animals = vec![
//!         "🦍 gorilla",
//!         "🪼 jellyfish",
//!         "🦁 lion",
//!         "🐝 honeybee",
//!         "🐗 boar",
//!         "🦇 bat",
//!         "🐌 snail",
//!         "🐨 koala",
//!         "🦉 owl",
//!         "🐢 turtle",
//!         "🐬 dolphin",
//!     ];
//!
//!     let selected_animals = select(&animals).expect("Error message");
//!
//!     Ok(())
//! }
//! ```
//! 
//! ## Or by using *ValintaError*
//! 
//! ```rust
//! use valinta::{select, ValintaError};
//! fn main() -> Result<(), ValintaError> {
//!
//!     let animals = vec![
//!         "🦍 gorilla",
//!         "🪼 jellyfish",
//!         "🦁 lion",
//!         "🐝 honeybee",
//!         "🐗 boar",
//!         "🦇 bat",
//!         "🐌 snail",
//!         "🐨 koala",
//!         "🦉 owl",
//!         "🐢 turtle",
//!         "🐬 dolphin",
//!     ];
//!
//!     let selected_animals = select(&animals)?;
//!
//!     Ok(())
//! }
//! ```
//! 
//!
//! # User interactions
//!
//! |*key pressed*|Action|
//! |-|-|
//! |*↓*|Next item|
//! |*↑*|Previous item|
//! |*a* **or** *A*|Select all|
//! |*n* **or** *N*|Deselect all|
//! |*i* **or** *I*|Invert selection|
//! |*space*|Toggle current|
//! |*enter*|Confirm selection|
//! |*esc*|Break|
//!
//! # Returned type
//!
//! A tupple which include the selected data and the indexes of selected data
//!
//! ```
//! pub type Returned<T> = (Vec<T>, Vec<usize>);
//! ```
//!

// Internal modules
mod def;
mod terminal;
mod usekey;
mod utils;

/// Exposed and flattenned modules
mod error;
mod select;
pub use error::ValintaError;
pub use select::select;
