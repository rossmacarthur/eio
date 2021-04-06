use std::io;
use std::io::prelude::*;
use std::mem;

mod private {
    pub trait Sealed: Sized {}
}

/// Conversion of bytes in little endian order to a new type.
pub trait FromLeBytes: private::Sealed {
    fn from_le_bytes<R: Read>(bytes: R) -> io::Result<Self>;
}

/// Conversion of bytes in big endian order to a new type.
pub trait FromBeBytes: private::Sealed {
    fn from_be_bytes<R: Read>(bytes: R) -> io::Result<Self>;
}

/// Provides extended methods to types that implement [`Read`].
pub trait ReadExt: Read {
    /// Read `T` from the source in little endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// #
    /// # fn main() -> io::Result<()> {
    /// #
    /// let buf = io::Cursor::new(&[0x78, 0x56, 0x34, 0x12]);
    /// let x: u32 = buf.read_le()?;
    /// assert_eq!(x, 0x12345678);
    /// #
    /// # Ok(()) }
    /// ```
    fn read_le<T: FromLeBytes>(&mut self) -> io::Result<T> {
        T::from_le_bytes(self)
    }

    /// Read `T` from the source in big endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// #
    /// # fn main() -> io::Result<()> {
    /// #
    /// let buf = io::Cursor::new(&[0x12, 0x34, 0x56, 0x78]);
    /// let x: u32 = buf.read_be()?;
    /// assert_eq!(x, 0x12345678);
    /// #
    /// # Ok(()) }
    /// ```
    fn read_be<T: FromBeBytes>(&mut self) -> io::Result<T> {
        T::from_be_bytes(self)
    }
}

macro_rules! impl_from_bytes {
    ($($ty:ident)+) => ($(
        impl FromLeBytes for $ty {
            fn from_le_bytes<R: Read>(mut bytes: R) -> io::Result<Self> {
                let mut buf = [0; mem::size_of::<Self>()];
                bytes.read_exact(&mut buf)?;
                Ok(Self::from_le_bytes(buf))
            }
        }

        impl FromBeBytes for $ty {
            fn from_be_bytes<R: Read>(mut bytes: R) -> io::Result<Self> {
                let mut buf = [0; mem::size_of::<Self>()];
                bytes.read_exact(&mut buf)?;
                Ok(Self::from_be_bytes(buf))
            }
        }

        impl private::Sealed for $ty {}
    )+)
}

impl_from_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize }

impl<R> ReadExt for R where R: Read {}
