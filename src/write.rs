use std::io;
use std::io::prelude::*;

mod private {
    pub trait Sealed: Sized {}
}

/// Conversion of bytes in little endian order to a new type.
pub trait ToLeBytes: private::Sealed {
    fn to_le_bytes<W: Write>(&self, bytes: W) -> io::Result<()>;
}

/// Conversion of bytes in big endian order to a new type.
pub trait ToBeBytes: private::Sealed {
    fn to_be_bytes<W: Write>(&self, bytes: W) -> io::Result<()>;
}

/// Provides extended methods to types that implement [`Write`].
pub trait WriteExt: Write {
    /// Write `T` to the destination in little endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use io_ext::WriteExt;
    /// #
    /// # fn main() -> std::io::Result<()> {
    /// #
    /// let x = 0x12345678;
    /// let mut buf = &mut [0u8, 0, 0, 0];
    /// buf.write_le(x)?;
    /// assert_eq!(buf, &[0x78, 0x56, 0x34, 0x12]);
    /// #
    /// # Ok(()) }
    /// ```
    fn write_le<T: ToLeBytes>(&mut self, t: T) -> io::Result<()> {
        t.to_le_bytes(self)
    }

    /// Write `T` to the destination in big endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use io_ext::WriteExt;
    /// #
    /// # fn main() -> std::io::Result<()> {
    /// #
    /// let x = 0x12345678;
    /// let mut buf = &mut [0u8, 0, 0, 0];
    /// buf.write_be(x)?;;
    /// assert_eq!(buf, &[0x12, 0x34, 0x56, 0x78]);
    /// #
    /// # Ok(()) }
    /// ```
    fn write_be<T: ToBeBytes>(&mut self, t: T) -> io::Result<()> {
        t.to_be_bytes(self)
    }
}

macro_rules! impl_to_bytes {
    ($($ty:ident)+) => ($(
        impl ToLeBytes for $ty {
            fn to_le_bytes<W: Write>(&self, mut bytes: W) -> io::Result<()> {
                bytes.write_all(&$ty::to_le_bytes(*self))
            }
        }

        impl ToBeBytes for $ty {
            fn to_be_bytes<W: Write>(&self, mut bytes: W) -> io::Result<()> {
                bytes.write_all(&$ty::to_be_bytes(*self))
            }
        }

        impl private::Sealed for $ty {}
    )+)
}

impl_to_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize }

impl<W> WriteExt for W where W: Write {}
