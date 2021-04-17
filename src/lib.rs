//! This crate provides extended methods to types that implement
//! [`Read`][std::io::Read] and [`Write`][std::io::Write].
//!
//! # Examples
//!
//! ```
//! use std::io;
//! use eio::*;
//!
//! fn main() -> io::Result<()> {
//!     let mut data = io::Cursor::new([0x37, 0x13, 0x12, 0x34]);
//!
//!     assert_eq!(data.read_le::<u16>()?, 0x1337);
//!     assert_eq!(data.read_be::<u16>()?, 0x1234);
//!
//!     Ok(())
//! }
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod read;
mod write;

pub use crate::read::*;
pub use crate::write::*;
