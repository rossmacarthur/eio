# eio

Read and write numbers in big-endian and little-endian.

## 🚀 Getting started

Add the following to your Cargo manifest.
```toml
[dependencies]
eio = "0.1"
```

And bring the `ReadExt` and/or `WriteExt` traits into scope.
```rust
use eio::{ReadExt, WriteExt};
```

## 🤸 Usage

The most common usage is parsing numbers from a source. You can do this using
the `read_le()` and `read_be()` methods on anything that implements `Read`.

```rust
use eio::ReadExt;

// `Cursor` implements `Read`
let mut rdr = std::io::Cursor::new([
  0x37, 0x13,
  0x12, 0x34, 0x56, 0x78,
  0x00, 0x09, 0x10,
]);

// Read a two byte `u16` in little-endian order
let i: u16 = rdr.read_le()?;
assert_eq!(i, 0x1337);

// Read a four byte `i32` in big-endian order
let i: i32 = rdr.read_be()?;
assert_eq!(i, 0x12345678);

// Read a three byte array
let a: [u8; 3] = rdr.read_array()?;
assert_eq!(a, [0x00, 0x09, 0x10]);
```

Serialization of numbers can be done using the `write_le()` and `write_be()`.
This can be done on anything that implements `Write`.

```rust
use eio::WriteExt;

// `&mut [u8]` implements `Write`.
let mut wtr = Vec::new();

// Write a four byte `f32` in little-endian order
wtr.write_le(1_f32)?;
// Write a one byte `u8`
wtr.write_be(7_u8)?;

assert_eq!(wtr, &[0, 0, 0x80, 0x3f, 0x07]);
```

In `no_std` contexts the `FromBytes` and `ToBytes` traits can be used directly.
```rust
use eio::{FromBytes, ToBytes};

let x: u32 = FromBytes::from_be_bytes([0, 0, 0, 7]);
assert_eq!(x, 7);

let data = ToBytes::to_le_bytes(x);
assert_eq!(data, [7, 0, 0, 0]);
```

## 💡 Prior art

`eio` provides the same capabilities as the popular [`byteorder`] crate but with
a very different API. The advantages of `eio` are the following:

- It is extendible, anyone can implement `FromBytes` or `ToBytes` for their own
  integer types.
- Uses the core/std `{from,to}_{le,be}_bytes` functions to do the conversion for
  floats and integers. [`byteorder`] reimplements these.
- Doesn't require turbofish type annotations all the time.
  ```rust
  // byteorder
  let i = rdr.read_u16::<BigEndian>()?;
  // eio
  let i: u16 = rdr.read_be()?;
  ```

[`byteorder`]: https://crates.io/crates/byteorder

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
