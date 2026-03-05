# Protobuf Derive Macros

cel-cxx provides derive macros that generate `TypedValue`, `IntoValue`, and `FromValue`
implementations for protobuf-generated Rust types. This enables the standard typed API
(`declare_variable::<T>()`, `bind_variable("x", msg)`, `T::from_value(&result)`) instead
of the lower-level dynamic API (`declare_variable_with_type`, `bind_variable_dynamic`,
`StructValue::from_bytes`).

## Feature Flags

| Feature | Derive Macro | Protobuf Crate | Status |
|---------|-------------|----------------|--------|
| `prost` | `#[derive(ProstValue)]` | [prost](https://crates.io/crates/prost) | Fully supported |
| `protobuf-legacy` | `#[derive(ProtobufLegacyValue)]` | [protobuf](https://crates.io/crates/protobuf) v3 (stepancheg) | Fully supported |
| `protobuf` | `#[derive(ProtobufValue)]` | protobuf v4 (cpp kernel) | Stub (not yet implemented) |

Enable the feature matching your protobuf library:

```toml
[dependencies]
cel-cxx = { version = "0.2", features = ["prost"] }
# or
cel-cxx = { version = "0.2", features = ["protobuf-legacy"] }
```

## Setup

The derive macros are injected during protobuf code generation in your `build.rs`.
Both backends support this with minimal configuration.

### prost

**Dependencies:**

```toml
[dependencies]
cel-cxx = { version = "0.2", features = ["prost"] }
prost = "0.14"

[build-dependencies]
prost-build = "0.14"
```

**build.rs:**

```rust
fn main() {
    prost_build::Config::new()
        .enable_type_names()  // required: generates prost::Name impls
        .type_attribute("my.package.MyMessage", "#[derive(::cel_cxx::ProstValue)]")
        .type_attribute("my.package.Address", "#[derive(::cel_cxx::ProstValue)]")
        .compile_protos(&["proto/my_service.proto"], &["proto/"])
        .unwrap();
}
```

The key points:
- `.enable_type_names()` is required -- `ProstValue` uses `prost::Name::full_name()` to
  derive the protobuf type name.
- `.type_attribute()` injects the derive on specific message types. You can also use
  `"."` as the path to apply it to all types.

### protobuf-legacy (stepancheg v3)

**Dependencies:**

```toml
[dependencies]
cel-cxx = { version = "0.2", features = ["protobuf-legacy"] }
protobuf = "3"

[build-dependencies]
protobuf = "3"
protobuf-codegen = "3"
```

**build.rs:**

```rust
use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::{Codegen, Customize, CustomizeCallback};

struct DeriveProtobufLegacyValue;

impl CustomizeCallback for DeriveProtobufLegacyValue {
    fn message(&self, _message: &MessageDescriptor) -> Customize {
        Customize::default().before("#[derive(::cel_cxx::ProtobufLegacyValue)]")
    }
}

fn main() {
    Codegen::new()
        .cargo_out_dir("protos")
        .input("proto/my_service.proto")
        .include("proto/")
        .customize_callback(DeriveProtobufLegacyValue)
        .run_from_script();
}
```

The `customize_callback` API injects arbitrary attributes before generated items.
The callback receives a `MessageDescriptor`, so you can selectively derive on
specific messages if needed.

## Usage

Once the derives are in place, protobuf types integrate with the standard cel-cxx API:

```rust,ignore
use cel_cxx::*;

// Include the generated types
include!(concat!(env!("OUT_DIR"), "/my.package.rs"));  // prost
// or
// include!(concat!(env!("OUT_DIR"), "/protos/my_service.rs"));  // protobuf-legacy

// 1. Compile descriptors (needed by cel-cpp at runtime)
let descriptors: Vec<u8> = { /* protox::compile(...) or include_bytes!(...) */ };

// 2. Build environment with typed declarations
let env = Env::builder()
    .with_file_descriptor_set(&descriptors)
    .declare_variable::<MyMessage>("msg")?
    .build()?;

// 3. Compile and evaluate
let program = env.compile("msg.name == 'Alice' && msg.id > 0")?;

let msg = MyMessage {
    name: "Alice".into(),
    id: 42,
    ..Default::default()
};

let activation = Activation::new().bind_variable("msg", msg)?;
let result = program.evaluate(&activation)?;
assert_eq!(result, Value::Bool(true));
```

### Round-trip: Rust -> CEL -> Rust

```rust,ignore
// CEL can construct messages using Type{field: value} syntax
let program = env.compile("my.package.MyMessage{name: 'Bob', id: 99}")?;
let result = program.evaluate(&Activation::new())?;

// Recover the Rust type from the CEL result
let recovered = MyMessage::from_value(&result)?;
assert_eq!(recovered.name, "Bob");
assert_eq!(recovered.id, 99);

// TryFrom<Value> also works
let recovered: MyMessage = result.try_into()?;
```

## Attributes

Both `ProstValue` and `ProtobufLegacyValue` support the `#[cel_cxx(...)]` attribute:

| Attribute | Description | Default |
|-----------|-------------|---------|
| `type_name = "..."` | Override the protobuf fully-qualified type name | Derived from the protobuf library |
| `crate = ...` | Custom path to the cel_cxx crate | `::cel_cxx` |

Example:

```rust,ignore
#[derive(ProstValue)]
#[cel_cxx(type_name = "custom.package.MyType")]
pub struct MyMessage { /* ... */ }
```

## How It Works

The derive macros generate implementations based on serialization:

1. **`TypedValue`** -- returns `ValueType::Struct(StructType::new(full_name))` where the
   full name comes from the protobuf library (`prost::Name::full_name()` or
   `protobuf::MessageFull::descriptor().full_name()`).

2. **`IntoValue`** -- serializes the message to bytes and wraps it in a
   `StructValue::from_bytes(type_name, bytes)`.

3. **`FromValue`** -- extracts the bytes from a `StructValue` and deserializes back
   to the Rust type.

This means messages cross the Rust/C++ FFI boundary via serialization. See the
[Performance Note](../README.md#performance-note) in the main README for details.

## Limitations

- Generic types are not supported (the derive macro will emit a compile error).
- The `protobuf` feature (v4 cpp kernel) is a stub -- all methods panic with
  `unimplemented!()`. This will be implemented when the Rust protobuf v4 ecosystem
  stabilizes.
