# cel-build-utils

[![Crates.io](https://img.shields.io/crates/v/cel-build-utils.svg)](https://crates.io/crates/cel-build-utils)
[![Docs.rs](https://docs.rs/cel-build-utils/badge.svg)](https://docs.rs/cel-build-utils)

Build utilities for [google/cel-cpp](https://github.com/google/cel-cpp).

This crate provides utilities for downloading, building, and linking against the CEL-CPP library. 
It is primarily intended for use in `build.rs` scripts and is used internally by the 
[cel-cxx-ffi](https://crates.io/crates/cel-cxx-ffi) crate.

## Features

- **Automatic CEL-CPP download and compilation**
- **Bazel integration** for building CEL-CPP
- **Cross-platform build support** (Linux, macOS)
- **Artifact management** for compiled libraries and headers

## Usage

Add this to your `build.rs`:

```rust
use cel_build_utils::{Build, Artifacts};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifacts = Build::new().build();
    artifacts.print_cargo_metadata();
    Ok(())
}
```

Add to your `Cargo.toml`:

```toml
[build-dependencies]
cel-build-utils = "0.1.0"
```

## License

Licensed under the Apache License 2.0. See the [LICENSE](../../LICENSE) file for details. 