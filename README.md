[![github]](https://github.com/xjasonli/cel-cxx)
[![crates-io]](https://crates.io/crates/cel-cxx)
[![docs-rs]](https://docs.rs/cel-cxx)
[![deepwiki]](https://deepwiki.com/xjasonli/cel-cxx)

[github]: https://img.shields.io/badge/GitHub-Repository-blue?style=flat&logo=github
[crates-io]: https://img.shields.io/crates/v/cel-cxx.svg
[docs-rs]: https://docs.rs/cel-cxx/badge.svg
[deepwiki]: https://deepwiki.com/badge.svg

- [CEL-CXX](#cel-cxx)
  - [Documentation](#documentation)
  - [Quick Start](#quick-start)
    - [Installation](#installation)
    - [Basic Expression Evaluation](#basic-expression-evaluation)
    - [Custom Types with Derive Macros](#custom-types-with-derive-macros)
  - [Platform Support](#platform-support)
    - [Cross-Compilation Support](#cross-compilation-support)
    - [Android Build Instructions](#android-build-instructions)
  - [CEL Feature Support](#cel-feature-support)
    - [Core Language Features](#core-language-features)
    - [Standard Library](#standard-library)
    - [Optional Value Support](#optional-value-support)
    - [Extension Libraries](#extension-libraries)
    - [Runtime Features](#runtime-features)
  - [Protobuf Integration](#protobuf-integration)
    - [Typed API with Derive Macros](#typed-api-with-derive-macros)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

# CEL-CXX

A type-safe Rust library for [Common Expression Language (CEL)](https://github.com/google/cel-spec), built on top of [cel-cpp](https://github.com/google/cel-cpp) with zero-cost FFI bindings via [cxx](https://github.com/dtolnay/cxx).

## Documentation

- [English Documentation](https://github.com/xjasonli/cel-cxx/tree/master/docs/) - Complete documentation and guides
- [中文文档](https://github.com/xjasonli/cel-cxx/tree/master/docs-cn/) - 中文文档和指南

For detailed guides on architecture, function registration, type system, and advanced features, see the [documentation directory](https://github.com/xjasonli/cel-cxx/tree/master/docs/).

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cel-cxx = "0.2.4"

# Optional features
cel-cxx = { version = "0.2.4", features = ["tokio"] }

# Protobuf derive macros (choose your backend)
cel-cxx = { version = "0.2.4", features = ["prost"] }
cel-cxx = { version = "0.2.4", features = ["protobuf-legacy"] }
```

### Basic Expression Evaluation

```rust,no_run
use cel_cxx::*;

// 1. Build environment with variables and functions
let env = Env::builder()
    .declare_variable::<String>("name")?
    .declare_variable::<i64>("age")?
    .register_global_function("adult", |age: i64| age >= 18)?
    .build()?;

// 2. Compile expression
let program = env.compile("'Hello ' + name + '! You are ' + (adult(age) ? 'an adult' : 'a minor')")?;

// 3. Create activation with variable bindings
let activation = Activation::new()
    .bind_variable("name", "Alice")?
    .bind_variable("age", 25i64)?;

// 4. Evaluate
let result = program.evaluate(&activation)?;
println!("{}", result); // "Hello Alice! You are an adult"
# Ok::<(), cel_cxx::Error>(())
```

### Custom Types with Derive Macros

```rust,no_run
use cel_cxx::*;

#[derive(Opaque, Debug, Clone, PartialEq)]
// Specify type name in CEL type system.
#[cel_cxx(type = "myapp.User")]
// Generates `std::fmt::Display` impl for User` with `Debug` trait.
#[cel_cxx(display)]
// or you can specify a custom format.
// Generates `std::fmt::Display` impl with custom format.
#[cel_cxx(display = write!(fmt, "User(name={name})", name = &self.name))]
struct User {
    name: String,
    age: i32,
    roles: Vec<String>,
}

impl User {
    // Struct methods can be registered directly as CEL member functions
    fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    fn get_role_count(&self) -> i64 {
        self.roles.len() as i64
    }
}

let env = Env::builder()
    .declare_variable::<User>("user")?
    // ✨ Register struct methods directly - &self becomes CEL receiver
    .register_member_function("has_role", User::has_role)?
    .register_member_function("is_adult", User::is_adult)?
    .register_member_function("get_role_count", User::get_role_count)?
    .build()?;

let program = env.compile("user.has_role('admin') && user.is_adult()")?;
# Ok::<(), cel_cxx::Error>(())
```

## Platform Support

<table>
<thead>
<tr>
<th>Platform</th>
<th>Target Triple</th>
<th>Status</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr>
<td rowspan="4"><strong>Linux</strong></td>
<td><code>x86_64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>aarch64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>armv7-unknown-linux-gnueabi</code></td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>i686-unknown-linux-gnu</code></td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><strong>Windows</strong></td>
<td><code>x86_64-pc-windows-msvc</code></td>
<td>✅</td>
<td>Tested (Visual Studio 2022+)</td>
</tr>
<tr>
<td rowspan="3"><strong>macOS</strong></td>
<td><code>x86_64-apple-darwin</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>aarch64-apple-darwin</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>arm64e-apple-darwin</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td rowspan="4"><strong>Android</strong></td>
<td><code>aarch64-linux-android</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td><code>armv7-linux-androideabi</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td><code>x86_64-linux-android</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td><code>i686-linux-android</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td rowspan="4"><strong>iOS</strong></td>
<td><code>aarch64-apple-ios</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-ios-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>x86_64-apple-ios</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>arm64e-apple-ios</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td rowspan="3"><strong>tvOS</strong></td>
<td><code>aarch64-apple-tvos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-tvos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>x86_64-apple-tvos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td rowspan="5"><strong>watchOS</strong></td>
<td><code>aarch64-apple-watchos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-watchos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>x86_64-apple-watchos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>arm64_32-apple-watchos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>armv7k-apple-watchos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td rowspan="2"><strong>visionOS</strong></td>
<td><code>aarch64-apple-visionos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-visionos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><strong>WebAssembly</strong></td>
<td><code>wasm32-unknown-emscripten</code></td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
</tbody>
</table>

**Legend:**
- ✅ **Tested**: Confirmed working with automated tests
- 🟡 **Should work**: Build configuration exists but not tested in CI

### Cross-Compilation Support

cel-cxx includes built-in support for cross-compilation via [cross-rs](https://github.com/cross-rs/cross). The build system automatically detects cross-compilation environments and configures the appropriate toolchains.

**Usage with cross-rs:**
```bash
# Install cross-rs
cargo install cross --git https://github.com/cross-rs/cross

# Build for aarch64
cross build --target aarch64-unknown-linux-gnu
```

**Note**: Not all cross-rs targets are supported due to CEL-CPP's build requirements. musl targets and some embedded targets may not work due to missing C++ standard library support or incompatible toolchains.

### Android Build Instructions

Android builds require additional setup beyond the standard Rust toolchain:

**Prerequisites:**
1. Install Android NDK and set `ANDROID_NDK_HOME`
2. Install `cargo-ndk` for simplified Android builds

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

**Building for Android:**
```bash
# Build for ARM64 (recommended)
cargo ndk --target aarch64-linux-android build

# Build for ARMv7
cargo ndk --target armv7-linux-androideabi build

# Build for x86_64 (emulator)
cargo ndk --target x86_64-linux-android build

# Build for i686 (emulator)
cargo ndk --target i686-linux-android build
```

**Why cargo-ndk is required:**
- `ANDROID_NDK_HOME` configures Bazel for CEL-CPP compilation
- `cargo-ndk` automatically sets up `CC_{target}` and `AR_{target}` environment variables needed for the Rust FFI layer
- This ensures both the C++ (CEL-CPP) and Rust (cel-cxx-ffi) components use compatible toolchains

## CEL Feature Support

### Core Language Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Basic Types** | ✅ | `null`, `bool`, `int`, `uint`, `double`, `string`, `bytes` |
| **Collections** | ✅ | `list<T>`, `map<K,V>` with full indexing and comprehensions |
| **Time Types** | ✅ | `duration`, `timestamp` with full arithmetic support |
| **Operators** | ✅ | Arithmetic, logical, comparison, and membership operators |
| **Variables** | ✅ | Variable binding and scoping |
| **Conditionals** | ✅ | Ternary operator and logical short-circuiting |
| **Comprehensions** | ✅ | List and map comprehensions with filtering |
| **Custom Types** | ✅ | Opaque types via `#[derive(Opaque)]` |
| **Protobuf Message Type** | ✅ | Native protobuf messages and enums as first-class CEL types with field access, message construction, and round-trip serialization |
| **Macros** | ✅ | CEL macro expansion support |
| **Function Overloads** | ✅ | Multiple function signatures with automatic resolution |
| **Type Checking** | ✅ | Compile-time type validation |

### Standard Library

| Feature | Status | Description |
|---------|--------|-------------|
| **Built-in Functions** | ✅ | Core CEL functions: `size()`, `type()`, `has()`, etc. |
| **String Functions** | ✅ | `contains()`, `startsWith()`, `endsWith()`, `matches()` |
| **List Functions** | ✅ | `all()`, `exists()`, `exists_one()`, `filter()`, `map()` |
| **Map Functions** | ✅ | Key/value iteration and manipulation |
| **Type Conversion** | ✅ | `int()`, `double()`, `string()`, `bytes()`, `duration()`, `timestamp()` |
| **Math Functions** | ✅ | Basic arithmetic and comparison operations |

### Optional Value Support

| Feature | Status | Description |
|---------|--------|-------------|
| **Optional Types** | ✅ | `optional<T>` with safe navigation and null handling |
| **Safe Navigation** | ✅ | `?.` operator for safe member access |
| **Optional Chaining** | ✅ | Chain optional operations without explicit null checks |
| **Value Extraction** | ✅ | `value()` and `hasValue()` functions for optional handling |
| **Optional Macros** | ✅ | `optional.of()`, `optional.ofNonZeroValue()` macros |

### Extension Libraries

| Extension | Status | Description |
|-----------|--------|-------------|
| **Strings Extension** | ✅ | Advanced string operations: `split()`, `join()`, `replace()`, `format()` |
| **Math Extension** | ✅ | Mathematical functions: `math.greatest()`, `math.least()`, `math.abs()`, `math.sqrt()`, bitwise ops |
| **Lists Extension** | ✅ | Enhanced list operations: `flatten()`, `reverse()`, `slice()`, `unique()` |
| **Sets Extension** | ✅ | Set operations: `sets.contains()`, `sets.equivalent()`, `sets.intersects()` |
| **Regex Extension** | ✅ | Regular expression support: `matches()`, `findAll()`, `split()` |
| **Encoders Extension** | ✅ | Encoding/decoding: `base64.encode()`, `base64.decode()`, URL encoding |
| **Bindings Extension** | ✅ | Variable binding and scoping enhancements |

### Runtime Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Custom Functions** | ✅ | Register custom Rust functions with automatic type conversion |
| **Async Support** | ✅ | Async function calls and evaluation with Tokio integration |
| **Custom Extensions** | ✅ | Build and register custom CEL extensions |
| **Performance Optimization** | ✅ | Optimized evaluation with caching and short-circuiting |

## Protobuf Integration

cel-cxx supports native Protocol Buffer messages as first-class CEL types. You can bind serialized protobuf messages as variables, access their fields in CEL expressions, construct new messages, and extract results back to Rust.

### Compiling Proto Descriptors

CEL needs a `FileDescriptorSet` (a binary descriptor file) to understand your `.proto` types.
The recommended approach is [`protox`](https://crates.io/crates/protox), a pure-Rust protobuf
compiler that requires no external binary:

```rust,no_run
use prost::Message;

let fds = protox::compile(["proto/my_service.proto"], ["proto/"]).unwrap();
let descriptor_bytes = fds.encode_to_vec();
```

Alternatively, you can use `protoc` directly:

```bash
protoc --descriptor_set_out=descriptors.bin \
       --include_imports \
       --proto_path=proto \
       proto/my_service.proto
```

### Setting Up the Environment

```rust,no_run
use cel_cxx::*;

# let descriptor_bytes: Vec<u8> = vec![];
let env = Env::builder()
    .with_file_descriptor_set(&descriptor_bytes)
    .declare_variable_with_type("msg", ValueType::Struct(StructType::new("my.package.MyMessage")))?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

### Binding Protobuf Input

Serialize your message to bytes (e.g., via `prost`) and bind it:

```rust,no_run
# use cel_cxx::*;
# let serialized_bytes: Vec<u8> = vec![];
let activation = Activation::new()
    .bind_variable_dynamic("msg", StructValue::from_bytes("my.package.MyMessage", serialized_bytes))?;
# Ok::<(), cel_cxx::Error>(())
```

### Accessing Fields in CEL

CEL expressions can access all protobuf field types:

```cel
msg.name                              // scalar fields
msg.address.city                      // nested message fields
msg.tags[0]                           // repeated fields
msg.labels["env"]                     // map fields
msg.status == my.package.Status.ACTIVE // enum constants
has(msg.optional_field)               // field presence
```

### Message Construction

Construct new protobuf messages directly in CEL:

```cel
my.package.MyMessage{name: "Alice", id: 42}
```

### Extracting Results

```rust,no_run
# use cel_cxx::*;
# fn example(result: Value, env: Env) -> Result<(), Error> {
// Borrow via as_struct()
let sv = result.as_struct().unwrap();
let type_name = sv.type_name();
let bytes = sv.to_bytes();

// Or extract an owned StructValue
let sv = StructValue::from_value(&result)?;

// Read individual fields from Rust
let name = env.get_struct_field(&sv, "name")?;
let has_name = env.has_struct_field(&sv, "name")?;
# Ok(())
# }
```

### Typed API with Derive Macros

If you have compile-time protobuf types (via `prost-build` or `protobuf-codegen`), derive
macros let you skip the manual `StructValue::from_bytes` plumbing and use the standard
typed API instead:

```rust,ignore
// With the `prost` or `protobuf-legacy` feature enabled:
let env = Env::builder()
    .with_file_descriptor_set(&descriptors)
    .declare_variable::<MyMessage>("msg")?  // instead of declare_variable_with_type(...)
    .build()?;

let activation = Activation::new()
    .bind_variable("msg", my_message)?;  // instead of bind_variable_dynamic(...)

let result = program.evaluate(&activation)?;
let recovered = MyMessage::from_value(&result)?;  // instead of StructValue::from_value(...)
```

The derives are injected during code generation in your `build.rs` -- one line per type
for prost, or a small `CustomizeCallback` for protobuf-codegen.

See the **[Protobuf Derive Macros Guide](docs/protobuf-derive.md)** for full setup
instructions, feature flags, and examples.

### Well-Known Type Handling

cel-cpp automatically converts well-known types to their CEL equivalents:

| Protobuf Type | CEL Type | Notes |
|--------------|----------|-------|
| `google.protobuf.Duration` | `duration` | Full arithmetic and comparison support |
| `google.protobuf.Timestamp` | `timestamp` | Full arithmetic and comparison support |
| `google.protobuf.Int64Value`, `StringValue`, `BoolValue`, ... | Primitives | Auto-unboxed to `int`, `string`, `bool`, etc. |
| `google.protobuf.Struct` | `map` | Dynamic map with dot-notation access |
| `google.protobuf.Any` | — | `has()` works; full unpacking depends on cel-cpp support |

### Performance Note

Protobuf messages cross the Rust/C++ FFI boundary via serialization: messages are serialized to bytes on the Rust side and deserialized into arena-allocated C++ messages for evaluation, then serialized back when extracting results. This adds overhead proportional to message size.

Zero-copy is not currently implemented for two reasons. First, the architectural change to keep C++ arena-allocated messages alive across the FFI boundary would be significant. Second, true zero-copy would require both cel-cxx and the Rust protobuf library to link against the *exact same* C++ protobuf library instance so that message pointers can be passed directly across the boundary. The official Google Rust protobuf crate ([`protobuf`](https://crates.io/crates/protobuf)) supports a C++ kernel backend that would make this possible in theory, but it currently requires Bazel (not cargo), and there is no shared `protobuf-cpp-sys` crate that both libraries could depend on. cel-cxx also compiles its own copy of C++ protobuf as part of cel-cpp via cmake, so integrating a shared dependency would require reworking the build. Until the ecosystem converges, serialization/deserialization is the only correct approach.

## License

Licensed under the Apache License 2.0. See [LICENSE](https://github.com/xjasonli/cel-cxx/blob/master/LICENSE) for details.

## Acknowledgements

- [`google/cel-cpp`](https://github.com/google/cel-cpp) - The foundational C++ CEL implementation
- [`dtolnay/cxx`](https://github.com/dtolnay/cxx) - Safe and efficient Rust-C++ interop
- [`rmanoka/async-scoped`](https://github.com/rmanoka/async-scoped) - Scoped async execution for safe lifetime management
- The CEL community and other Rust CEL implementations for inspiration and ecosystem growth
