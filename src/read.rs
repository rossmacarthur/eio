use core::mem::size_of;

/// Conversion of bytes in little/big endian order to a type.
pub trait FromBytes<const N: usize> {
    fn from_be_bytes(bytes: [u8; N]) -> Self;
    fn from_le_bytes(bytes: [u8; N]) -> Self;
}

/// Provides extended methods to types that implement [`std::io::Read`].
#[cfg(feature = "std")]
pub trait ReadExt<const N: usize>: std::io::Read {
    /// Read an array of size `N` from the source.
    ///
    /// # Examples
    ///
    /// ```
    /// use eio::ReadExt;
    ///
    /// let buf: Vec<u8> = vec![0x12, 0x34, 0x56];
    /// let arr: [u8; 3] = buf.as_slice().read_array().unwrap();
    /// assert_eq!(arr, [0x12, 0x34, 0x56]);
    /// ```
    fn read_array(&mut self) -> std::io::Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Read `T` from the source in big endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use eio::ReadExt;
    ///
    /// let buf: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
    /// let x: u32 = buf.as_slice().read_be().unwrap();
    /// assert_eq!(x, 0x12345678);
    /// ```
    fn read_be<T: FromBytes<N>>(&mut self) -> std::io::Result<T> {
        self.read_array().map(T::from_be_bytes)
    }

    /// Read `T` from the source in little endian order.
    ///
    /// # Examples
    ///
    /// ```
    /// use eio::ReadExt;
    ///
    /// let buf: Vec<u8> = vec![0x78, 0x56, 0x34, 0x12];
    /// let x: u32 = buf.as_slice().read_le().unwrap();
    /// assert_eq!(x, 0x12345678);
    /// ```
    fn read_le<T: FromBytes<N>>(&mut self) -> std::io::Result<T> {
        self.read_array().map(T::from_le_bytes)
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

#[cfg(feature = "std")]
impl<R, const N: usize> ReadExt<N> for R where R: std::io::Read {}
