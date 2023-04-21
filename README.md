# `mac_address2`

[![crates.io](https://img.shields.io/crates/v/mac_address2.svg)](https://crates.io/crates/mac_address2)
[![Released API docs](https://docs.rs/mac_address2/badge.svg)](https://docs.rs/mac_address2)

`mac_address2` provides a cross-platform way to retrieve the [MAC address](https://en.wikipedia.org/wiki/MAC_address) of network hardware.

Supported platforms: Linux, Windows, macOS, FreeBSD, OpenBSD, Android

## Example

```rust
use mac_address2::get_mac_address;

fn main() {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("No MAC address found."),
        Err(e) => println!("{:?}", e),
    }
}
```

## License

`mac_address2` is licensed under both MIT and Apache 2.0
