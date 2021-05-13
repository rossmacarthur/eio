//! Read and write numbers in big-endian and little-endian.
//!
//! # Examples
//!
//! ```
//! use eio::ReadExt;
//!
//! # fn main() -> std::io::Result<()> {
//! #
//! // `Cursor` implements `Read`
//! let mut rdr = std::io::Cursor::new([
//!   0x37, 0x13,
//!   0x12, 0x34, 0x56, 0x78,
//!   0x00, 0x09, 0x10,
//! ]);
//!
//! // Read a two byte `u16` in little-endian order
//! let i: u16 = rdr.read_le()?;
//! assert_eq!(i, 0x1337);
//!
//! // Read a four byte `i32` in big-endian order
//! let i: i32 = rdr.read_be()?;
//! assert_eq!(i, 0x12345678);
//!
//! // Read a three byte array
//! let a: [u8; 3] = rdr.read_array()?;
//! assert_eq!(a, [0x00, 0x09, 0x10]);
//! #
//! # Ok(()) }
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod read;
mod write;

pub use crate::read::*;
pub use crate::write::*;
