//! This crate provides extended methods to types that implement
//! [`Read`][std::io::Read] and [`Write`][std::io::Write].
//!
//! # Examples
//!
//! ```
//! use std::io;
//! use io_ext::*;
//!
//! fn main() -> io::Result<()> {
//!     let mut data = io::Cursor::new(vec![0x37, 0x13, 0x13, 0x37]);
//!
//!     let x: u16 = data.read_le()?;
//!     assert_eq!(x, 0x1337);
//!
//!     let y: u16 = data.read_be()?;
//!     assert_eq!(y, 0x1337);
//!
//!     Ok(())
//! }
//! ```

mod read;
mod write;

pub use crate::read::{FromBytes, ReadExt};
pub use crate::write::{ToBytes, WriteExt};
