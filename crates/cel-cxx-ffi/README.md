# cel-cxx-ffi

[![Crates.io](https://img.shields.io/crates/v/cel-cxx-ffi.svg)](https://crates.io/crates/cel-cxx-ffi)
[![Docs.rs](https://docs.rs/cel-cxx-ffi/badge.svg)](https://docs.rs/cel-cxx-ffi)

FFI bindings for the [cel-cxx](https://crates.io/crates/cel-cxx) crate.

This crate provides low-level FFI bindings to [google/cel-cpp](https://github.com/google/cel-cpp) 
using the [cxx](https://crates.io/crates/cxx) crate for safe interop. It is primarily intended 
for internal use by the `cel-cxx` crate and is not recommended for direct use.

## Features

- **Safe C++ interop** via the `cxx` crate
- **Zero-cost abstractions** for CEL-CPP integration
- **Type-safe bindings** for CEL values, expressions, and environments
- **Memory management** with proper RAII patterns
- **Error handling** with Rust-native error types

## Usage

This crate is automatically included when you use `cel-cxx`. You typically don't need to 
depend on it directly.

## Build Requirements

- **C++17 compatible compiler**
- **Bazel** (automatically managed by cel-build-utils)
- **Google CEL-CPP** (automatically downloaded and built)

## License

Licensed under the Apache License 2.0. See the [LICENSE](../../LICENSE) file for details. 
