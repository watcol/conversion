# Conversion
![status](https://img.shields.io/badge/status-Active-brightgreen)
[![crates.io](https://img.shields.io/crates/v/conversion)](https://crates.io/crates/conversion)
[![docs.rs](https://docs.rs/conversion/badge.svg)](https://docs.rs/conversion)
[![Downloads](https://img.shields.io/crates/d/conversion)](https://crates.io/crates/conversion)
[![Downloads (latest)](https://img.shields.io/crates/dv/conversion)](https://crates.io/crates/conversion)
[![License](https://img.shields.io/crates/l/conversion)](https://github.com/watcol/conversion/blob/main/LICENSE)

An abstraction crate to convert iterators on the fly.

## Demo
```rust
use conversion::converter::encoding::utf8::{UTF8Decoder, UTF8Encoder};
use conversion::converter::IterConverter;
use conversion::iter::{ConvertedIterator, ConvertedTryIterator};

// An original byte string.
let iter = b"stra\xc3\x9fe".into_iter().cloned();

// Decoding UTF-8 byte string.
let decoded = ConvertedIterator::new(iter, UTF8Decoder::new());
assert_eq!(Ok(String::from("straße")), decoded.clone().collect());

// Convert to uppercase. (use ConvertedTryIterator because `decoded` returns Result items.)
let uppered = ConvertedTryIterator::new(decoded, IterConverter::new(char::to_uppercase));
assert_eq!(Ok(String::from("STRASSE")), uppered.clone().collect());

// Re-encode the value.
let encoded = ConvertedTryIterator::new(uppered, UTF8Encoder::new());
assert_eq!(Ok(b"STRASSE".to_vec()), encoded.collect());
```

## Documentation
API Documentations are available on [here](https://docs.rs/conversion).

## Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
conversion = "0.1.0"
conversion = { version = "0.1.0", features = ["async"] } # If you want to use asynchronous stream.
conversion = { version = "0.1.0", default-features = false } # no_std support.
```

## License
This program is licensed under the MIT license.
See [LICENSE](https://github.com/watcol/conversion/blob/main/LICENSE) for details.
