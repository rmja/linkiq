![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# linkiq

Kamstrup OpenlinkIQ protocol stack and controller written in Rust.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
linkiq = { git = "https://github.com/rmja/linkiq", features = [] }
```

where the list of features are:
* `ctrl`: Adds transceiver controller for managing channel hopping, etc.

## References
The OpenlinkIQ specification can be obtained from https://www.openlinkiq.org.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
