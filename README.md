# io-ext

Extended methods for types that implement `Read` and `Write`.

## 🤸 Usage

```rust
use std::io;
use io_ext::*;

fn main() -> io::Result<()> {
    let mut data = io::Cursor::new([0x37, 0x13, 0x12, 0x34]);

    assert_eq!(data.read_le::<u16>()?, 0x1337);

    assert_eq!(data.read_be::<u16>()?, 0x1234);
    Ok(())
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
