use std::io;
use std::io::prelude::*;
use std::mem::size_of;

/// Conversion of bytes in little/big endian order to a type.
pub trait FromBytes<const N: usize> {
    fn from_be_bytes(bytes: [u8; N]) -> Self;
    fn from_le_bytes(bytes: [u8; N]) -> Self;
}

/// Provides extended methods to types that implement [`Read`].
pub trait ReadExt<const N: usize>: Read {
    /// Read `T` from the source in big endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use io_ext::ReadExt;
    ///
    /// let buf: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
    /// let x: u32 = buf.as_slice().read_be().unwrap();
    /// assert_eq!(x, 0x12345678);
    /// ```
    fn read_be<T: FromBytes<N>>(&mut self) -> io::Result<T> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(T::from_be_bytes(buf))
    }

    /// Read `T` from the source in little endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use io_ext::ReadExt;
    ///
    /// let buf: Vec<u8> = vec![0x78, 0x56, 0x34, 0x12];
    /// let x: u32 = buf.as_slice().read_le().unwrap();
    /// assert_eq!(x, 0x12345678);
    /// ```
    fn read_le<T: FromBytes<N>>(&mut self) -> io::Result<T> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(T::from_le_bytes(buf))
    }
}

macro_rules! impl_from_bytes {
    ($($ty:ident)+) => ($(
        impl FromBytes<{ size_of::<$ty>() }> for $ty {
            fn from_be_bytes(bytes: [u8; size_of::<$ty>()]) -> Self {
                Self::from_be_bytes(bytes)
            }

            fn from_le_bytes(bytes: [u8; size_of::<$ty>()]) -> Self {
                Self::from_le_bytes(bytes)
            }
        }
    )+)
}

impl_from_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64 }

impl<R, const N: usize> ReadExt<N> for R where R: Read {}
