# CEL-CXX examples

Run these commands from the repository root. Each example checks its key results
with assertions and returns unexpected errors to the caller.

```sh
cargo run --example basic
cargo run --example runtime-bindings
cargo run --example time-boundaries
cargo run --example async-cancellation --features tokio
```

The native build prerequisites are described in the [project README](../README.md).
The default features include `derive`. When using `--no-default-features`, add
`derive` for `comprehensive`, the three runtime examples that use `Opaque`, and
both protobuf derive examples.

## Core language and embedding

| Example | What it demonstrates | Extra features |
| --- | --- | --- |
| [basic](basic.rs) | Typed variables, Rust functions, and reusing compiled expressions | None |
| [comprehensive](comprehensive.rs) | Functions, collections, opaque values, validation, and optional results | `derive` (default) |
| [standard-library](standard-library.rs) | Operators, conversions, strings, bytes, collections, and time | None |
| [optional](optional.rs) | Optional field/index access, construction, fallbacks, and presence | None |
| [macros](macros.rs) | Custom AST expansion and checked evaluation | None |
| [runtime-bindings](runtime-bindings.rs) | Typed overloads, per-evaluation overrides, declaration-only functions, and invalid bindings | None |
| [time-boundaries](time-boundaries.rs) | Nanosecond arithmetic, negative durations, UTC offsets, and invalid dates | None |

## Extensions

Each extension example enables the corresponding library with `Env::builder()`.
Run it using `cargo run --example NAME`; no additional Cargo features are needed.

| Example | What it demonstrates |
| --- | --- |
| [extensions-bindings](extensions-bindings.rs) | Local bindings and evaluation reuse |
| [extensions-comprehensions](extensions-comprehensions.rs) | Two-variable comprehensions over lists and maps |
| [extensions-encoders](extensions-encoders.rs) | Base64 encoding, decoding, and invalid input |
| [extensions-lists](extensions-lists.rs) | List transformations, flattening, slicing, and ordering |
| [extensions-math](extensions-math.rs) | Numeric helpers, bit operations, and arithmetic boundaries |
| [extensions-re](extensions-re.rs) | Capture groups, named captures, and extraction errors |
| [extensions-regex](extensions-regex.rs) | Extraction, replacement, and regular-expression boundaries |
| [extensions-sets](extensions-sets.rs) | Membership, equivalence, intersection, and duplicate handling |
| [extensions-strings](extensions-strings.rs) | Unicode-aware operations, formatting, and string boundaries |

## Protobuf

| Example | What it demonstrates | Extra features |
| --- | --- | --- |
| [protobuf](protobuf.rs) | Runtime descriptors, serialization, CEL field access, and message construction | None |
| [protobuf-advanced](protobuf-advanced.rs) | Nested/repeated/map fields, enums, well-known types, and presence | None |
| [protobuf-prost-derive](protobuf-prost-derive.rs) | Generated `prost` messages with `ProstValue` | `prost` |
| [protobuf-legacy-derive](protobuf-legacy-derive.rs) | Generated rust-protobuf messages with `ProtobufLegacyValue` | `protobuf-legacy` |

```sh
cargo run --example protobuf
cargo run --example protobuf-advanced
cargo run --example protobuf-prost-derive --features prost
cargo run --example protobuf-legacy-derive --features protobuf-legacy
```

The examples locate `tests/fixtures/test.proto` using `CARGO_MANIFEST_DIR` and
compile its descriptor set with `protox`. Running a built binary does not require
`protoc`. Building the derive examples also generates Rust code through the
package build script. Both derive examples require `protoc` on `PATH` and its
standard `.proto` include files. For `prost`, `PROTOC` and `PROTOC_INCLUDE` can
select the binary and standard include directory.

## Async evaluation

| Example | Command | What it demonstrates |
| --- | --- | --- |
| [tokio](async/tokio.rs) | `cargo run --example tokio --features tokio` | Sync/async opaque methods, providers, and owned captures |
| [smol](async/smol.rs) | `cargo run --example smol --features smol` | The same operations with smol |
| [async-cancellation](async/cancellation.rs) | `cargo run --example async-cancellation --features tokio` | Cancellation of a pending callback and resource cleanup |

Async callbacks own their captures (`'static`); share application state with
`Arc` when needed. Dropping an evaluation future cancels a pending async callback.
It does not preempt synchronous callbacks or undo completed side effects.

To compile every example, including all runtime and code-generation variants:

```sh
cargo build --examples --all-features
```

Compilation alone does not execute the demonstrations. Run the individual
commands above to exercise their assertions and expected-error paths.
