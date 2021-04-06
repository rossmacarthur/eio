use std::io;
use std::io::prelude::*;
use std::mem::size_of;

/// Conversion of a type to bytes in little/big endian order.
pub trait ToBytes<const N: usize> {
    fn to_be_bytes(self) -> [u8; N];
    fn to_le_bytes(self) -> [u8; N];
}

/// Provides extended methods to types that implement [`Write`].
pub trait WriteExt: Write {
    /// Write `T` to the destination in big endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use io_ext::WriteExt;
    ///
    /// let mut w = Vec::new();
    /// w.write_be(0x12345678).unwrap();
    /// assert_eq!(w, &[0x12, 0x34, 0x56, 0x78]);
    /// ```
    fn write_be<T: ToBytes<N>, const N: usize>(&mut self, t: T) -> io::Result<()> {
        self.write_all(&t.to_be_bytes())
    }

    /// Write `T` to the destination in little endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use io_ext::WriteExt;
    ///
    /// let mut w = Vec::new();
    /// w.write_le(0x12345678).unwrap();
    /// assert_eq!(w, &[0x78, 0x56, 0x34, 0x12]);
    /// ```
    fn write_le<T: ToBytes<N>, const N: usize>(&mut self, t: T) -> io::Result<()> {
        self.write_all(&t.to_le_bytes())
    }
}

macro_rules! impl_to_bytes {
    ($($ty:ident)+) => ($(
        impl ToBytes<{ size_of::<$ty>() }> for $ty {
            fn to_be_bytes(self) -> [u8; size_of::<$ty>()] {
                self.to_be_bytes()
            }

            fn to_le_bytes(self) -> [u8; size_of::<$ty>()] {
                self.to_le_bytes()
            }
        }
    )+)
}

impl_to_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize }

impl<W> WriteExt for W where W: Write {}
