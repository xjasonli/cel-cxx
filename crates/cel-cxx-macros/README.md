# cel-cxx-macros

[![Crates.io](https://img.shields.io/crates/v/cel-cxx-macros.svg)](https://crates.io/crates/cel-cxx-macros)
[![Docs.rs](https://docs.rs/cel-cxx-macros/badge.svg)](https://docs.rs/cel-cxx-macros)

Procedural macros for the [cel-cxx](https://crates.io/crates/cel-cxx) crate.

This crate provides derive macros that enable seamless integration of custom Rust types 
with CEL expressions.

## Macros

### `#[derive(Opaque)]`

Automatically implements the necessary traits to use custom Rust types in CEL expressions:

```rust
use cel_cxx_macros::Opaque;

#[derive(Opaque, Debug, Clone)]
#[cel_cxx(type = "myapp.User")]
struct User {
    name: String,
    age: i32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User({})", self.name)
    }
}
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cel-cxx = { version = "0.1.0", features = ["derive"] }
```

Or use directly:

```toml
[dependencies]
cel-cxx-macros = "0.1.0"
```

## License

Licensed under the Apache License 2.0. See the [LICENSE](../../LICENSE) file for details. 