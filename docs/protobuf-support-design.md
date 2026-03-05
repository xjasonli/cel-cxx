# Protobuf Support Design

## Background

cel-cxx wraps [cel-cpp](https://github.com/google/cel-cpp), a C++ implementation of the
[Common Expression Language](https://cel.dev). One frequently requested feature is first-class
support for Protocol Buffer message types as CEL variables, enabling field access like `p.x`
rather than the opaque-type workaround of `p.x()`.

This document records the design decisions for protobuf support, explaining the constraints,
tradeoffs, and the chosen approach.

## Why It Is Non-Trivial

The fundamental constraint is that Rust and C++ cannot share a protobuf-cpp runtime today
(except via the future protobuf v4 cpp kernel). Any Rust protobuf library that is not backed
by protobuf-cpp maintains a separate `DescriptorPool`, `MessageFactory`, and arena allocator.
There is no common ABI through which a Rust-side `Message*` can be passed as a C++
`google::protobuf::Message*` — doing so would invoke undefined behavior.

However, **serialization is not the only solution**. cel-cpp's `StructValue` supports multiple
internal variants, one of which — `CustomStructValue` — is an abstract C++ class that user
code can subclass to provide custom field-access behavior. Whether a Rust protobuf library
can use this path instead of serialization depends on whether it exposes reflection APIs for
its generated message types.

## Rust Protobuf Ecosystem

Three Rust protobuf implementations are relevant:

### 1. [prost](https://docs.rs/prost)

- Most popular, used by tonic and the broader ecosystem.
- No descriptor or reflection support for generated message types.
- [prost-reflect](https://docs.rs/prost-reflect/) adds reflection, but only for
  `DynamicMessage`. Using it with generated types requires: get descriptor →
  serialize the message → construct a `DynamicMessage` from bytes. This is strictly
  worse than passing bytes directly.
- **Transport: serialization only.**

### 2. [protobuf v3](https://docs.rs/protobuf/3.7.2) (stepancheg/rust-protobuf)

- Has full descriptor, reflection, and dynamic message support for both generated
  and dynamic messages, via the `MessageFull` trait.
- [Announced end-of-life](https://github.com/stepancheg/rust-protobuf#end-of-life);
  the project is being transferred to the protobuf team. Not a suitable long-term
  foundation, but reflection support makes it usable without serialization.
- **Transport: `CustomStructValue` via reflection callbacks (no serialization on input).**

### 3. [protobuf v4](https://docs.rs/protobuf/4.34.0-release) ([protocolbuffers/protobuf/rust](https://github.com/protocolbuffers/protobuf/tree/main/rust))

- The official Google implementation.
- Currently pre-release; only the upb backend is available via crates.io.
- The upb and protobuf-cpp backends **cannot be linked into the same binary**.
- Descriptor, reflection, and dynamic message APIs do not yet exist in the crate.
- **When the protobuf-cpp backend becomes stable**, Rust and C++ will share the same
  underlying runtime. `Message*` pointers can be passed directly across the FFI
  boundary with zero overhead.
- **Transport: zero-copy pointer (future).**

## FFI Transport Strategies

### Strategy A: Serialization (prost)

Rust serializes the message to wire-format bytes. C++ deserializes into an
arena-allocated `ParsedMessageValue` using the custom `DescriptorPool`. Field access
is handled entirely by cel-cpp.

```
Rust (prost message) --encode_to_vec()--> [bytes] --ParseFromArray()--> C++ ParsedMessageValue
```

Straightforward but pays a full serialize/deserialize cost per FFI crossing.

### Strategy B: CustomStructValue via Reflection (protobuf-rust v3)

Rust wraps the `protobuf::MessageFull` trait object in a C++ `CustomStructValue` subclass.
cel-cpp calls virtual methods on the subclass during expression evaluation; each call
dispatches back to Rust via FFI, where protobuf-rust's reflection APIs (`MessageFull`,
`FieldDescriptor`, etc.) respond to the field query.

```
Rust (MessageFull) ---pointer---> C++ CustomStructValue subclass
                                       |
                    cel-cpp calls GetFieldByName("x")
                                       |
                     FFI callback --> Rust: msg.get_field_by_name("x")
                                       |
                     returns cel::Value (scalar, nested CustomStructValue, etc.)
```

No message-level serialization. Individual field values are converted to `cel::Value`
on demand. Nested messages are recursively wrapped as `CustomStructValue`.

This path has more implementation complexity (C++ abstract class bridge, per-field FFI
dispatch) but avoids the upfront serialize/deserialize cost. Whether the per-field
dispatch overhead is preferable to a one-time full serialization depends on the access
pattern.

### Strategy C: Zero-Copy Pointer (protobuf v4 cpp kernel, future)

When protobuf v4 uses the protobuf-cpp backend, the Rust message IS a C++ message.
Its descriptor is already in `generated_pool()`. A pointer cast is sufficient — no
serialization, no virtual dispatch.

```
Rust (protobuf::Message backed by protobuf-cpp) --ptr cast--> C++ ParsedMessageValue
```

## Rust → C++ Direction Summary

| Library | Strategy | Cost |
|---|---|---|
| prost | Serialization | full encode per binding |
| protobuf-rust v3 | CustomStructValue | per-field FFI dispatch |
| protobuf v4 cpp kernel | zero-copy pointer | pointer cast |

## C++ → Rust Direction

The return path depends on how the message was created inside CEL:

### Case 1: CEL-constructed message

When a CEL expression constructs a new message (e.g. `pkg.Point{x: 1.0, y: 2.0}`),
cel-cpp allocates a fresh `ParsedMessageValue` on an internal arena. This message has no
corresponding Rust object. It must be serialized to bring it back to Rust, then decoded
by the Rust pb library.

```
C++ ParsedMessageValue --SerializeToString()--> [bytes] --decode--> Rust message
```

This is the only option regardless of which Rust pb library is in use.

### Case 2: Round-trip of a Rust-originated CustomStructValue

When a Rust-originated `CustomStructValue` (Strategy B) is passed into CEL and returned
unmodified (or field-selected from), the C++ side can detect that the returned `StructValue`
is the original `CustomStructValue` subclass, retrieve the embedded Rust object pointer,
and hand it back to Rust without any deserialization.

```
Rust MessageFull --(in)--> C++ CustomStructValue --(out)--> Rust MessageFull (same object)
```

Note: this optimization is only applicable when the value has not been transformed by CEL.
If CEL constructs a new message or merges fields, the result is a `ParsedMessageValue`
and Case 1 applies.

## Design Goals

1. Provide protobuf message support without locking the public API into any particular
   transport strategy.
2. Leave a clear, non-breaking upgrade path as better transports become available.
3. Allow multiple protobuf libraries to coexist in one project.
4. Without any protobuf feature enabled, CEL protobuf message support is simply absent.
   No fallback "raw bytes" API is provided — it would leak implementation details and
   provide no meaningful abstraction.

## Core Design: Opaque `StructValue`

`StructValue` is the Rust-side representation of a protobuf message. Its internal layout
is an implementation detail — callers must not depend on it.

```rust
pub struct StructValue {
    inner: StructValueInner,
}

enum StructValueInner {
    // Strategy A: prost — wire-format bytes.
    // Also used for C++ → Rust transport of CEL-constructed messages (all backends).
    #[cfg(feature = "prost")]
    Bytes {
        type_name: String,
        bytes: Vec<u8>,
    },

    // Strategy B: protobuf-rust v3 — holds the Rust message, bridged as CustomStructValue.
    // On the C++ side, a CustomStructValue subclass holds a pointer to this box and
    // dispatches field access back to Rust via FFI.
    #[cfg(feature = "protobuf-legacy")]
    ReflectionMessage {
        type_name: String,
        message: Box<dyn protobuf::MessageFull>,
    },

    // Strategy C: protobuf v4 cpp kernel (future) — direct pointer to shared C++ message.
    #[cfg(feature = "protobuf")]
    CppMessage {
        type_name: String,
        ptr: CppMessageRef,
    },
}
```

Key properties:
- Fields are private. Callers construct `StructValue` only through feature-gated derive
  macros or named constructors on `StructValue` itself.
- The variants are mutually exclusive by feature; when multiple features are enabled,
  each message type uses the variant associated with the library it was compiled with.
- The `Bytes` variant also serves as the output representation when cel-cpp constructs a
  new message, regardless of which input strategy was used.

## Trait Implementations: Derive Macros, Not Blanket Impls

The natural wish is to provide blanket `TypedValue + IntoValue + FromValue` impls for all
prost messages, all protobuf-rust messages, and so on. This does not work: when multiple
features are enabled, two blanket impls over `M: prost::Message` and `M: protobuf::MessageFull`
are considered potentially overlapping by Rust's coherence rules, even if no concrete type
actually implements both.

The correct approach is **derive macros** that generate concrete impls for specific types:

```
cel-cxx-derive (or cel-cxx-macros)
├── ProstValue          — feature = "prost"
├── ProtobufLegacyValue — feature = "protobuf-legacy"  (protobuf-rust v3)
└── ProtobufValue       — feature = "protobuf"         (protobuf v4 cpp kernel, future)
```

Users apply these macros via their protobuf code generator's attribute injection:

```rust
// build.rs
prost_build::Config::new()
    .type_attribute("pkg.Point", "#[derive(cel_cxx::ProstValue)]")
    .compile_protos(&["proto/point.proto"], &["proto/"])?;
```

Each derive macro generates `TypedValue`, `IntoValue`, and `FromValue` for the specific
struct it is applied to. No blanket impls, no coherence issues, multiple libraries coexist.

### What the Macros Generate

For `#[derive(ProstValue)]` on a type `Point` (Strategy A, serialization):

```rust
impl cel_cxx::TypedValue for Point {
    fn value_type() -> cel_cxx::ValueType {
        cel_cxx::ValueType::Struct(
            cel_cxx::types::StructType::new(<Point as prost::Name>::full_name())
        )
    }
}

impl cel_cxx::IntoValue for Point {
    fn into_value(self) -> cel_cxx::Value {
        // Encodes to bytes; StructValueInner::Bytes
        cel_cxx::Value::Struct(cel_cxx::StructValue::from_prost(self))
    }
}

impl cel_cxx::FromValue for Point {
    type Output<'a> = Point;
    fn from_value<'a>(value: &'a cel_cxx::Value) -> Result<Point, cel_cxx::FromValueError> {
        match value {
            cel_cxx::Value::Struct(sv) => sv.decode_prost::<Point>()
                .map_err(|_| cel_cxx::FromValueError::new(value.clone(), "Point")),
            _ => Err(cel_cxx::FromValueError::new(value.clone(), "Point")),
        }
    }
}
```

For `#[derive(ProtobufLegacyValue)]` (Strategy B, CustomStructValue):

```rust
impl cel_cxx::IntoValue for Point {
    fn into_value(self) -> cel_cxx::Value {
        // Stores as ReflectionMessage; bridged to CustomStructValue at FFI boundary
        cel_cxx::Value::Struct(cel_cxx::StructValue::from_protobuf_legacy(self))
    }
}

impl cel_cxx::FromValue for Point {
    type Output<'a> = Point;
    fn from_value<'a>(value: &'a cel_cxx::Value) -> Result<Point, cel_cxx::FromValueError> {
        match value {
            cel_cxx::Value::Struct(sv) => {
                // If this is a round-trip of the original object, retrieve it directly.
                // If it is a CEL-constructed message (Bytes variant), deserialize.
                sv.try_recover_protobuf_legacy::<Point>()
                    .or_else(|_| sv.decode_protobuf_legacy::<Point>())
                    .map_err(|_| cel_cxx::FromValueError::new(value.clone(), "Point"))
            }
            _ => Err(cel_cxx::FromValueError::new(value.clone(), "Point")),
        }
    }
}
```

## Unified API via Existing Trait Infrastructure

Once `TypedValue + IntoValue + FromValue` are implemented for a message type via derive,
the **existing** `declare`, `bind`, and `FromValue`-based extraction methods work without
any protobuf-specific additions:

```rust
// Declaration — no new method needed
env_builder.declare::<Point>("p")?

// Binding — no new method needed
activation.bind("p", my_point)?

// Extraction — no new method needed
let p: Point = result.unwrap::<Point>()?;
```

No `bind_protobuf_variable`, `declare_protobuf_variable`, or `get_protobuf_field` methods
are part of the public API. Without a protobuf feature enabled, CEL protobuf message
support is simply absent.

## Descriptor Pool Strategy

`EnvBuilder::with_file_descriptor_set` registers custom protobuf types with cel-cpp's
type checker and runtime. The behavior and necessity differ by runtime:

### prost / protobuf-rust v3

Rust and C++ use separate runtimes. C++'s `generated_pool()` does not know about Rust-side
proto types. A new `DescriptorPool` must be constructed from the provided `FileDescriptorSet`
bytes, using `generated_pool()` as an underlay:

```
lookup order: custom pool -> generated_pool (underlay)
```

This preserves access to well-known types (Duration, Timestamp, etc.) while adding custom
types. `with_file_descriptor_set` is **required** for these backends.

```rust
Env::builder()
    .with_file_descriptor_set(file_descriptor_set_bytes)
    .build()?
```

### protobuf v4 cpp kernel (future)

Rust proto types are registered in `generated_pool()` via C++ static initializers at
program startup. `with_file_descriptor_set` is **not needed** — the default env already
uses `generated_pool()`.

```rust
Env::builder()
    .build()?
```

Mixed projects (protobuf v4 types + prost types) can call `with_file_descriptor_set` for
the prost descriptors only; protobuf v4 types remain accessible via the underlay.


## Public API Summary

```
EnvBuilder
  .with_file_descriptor_set(bytes)     required for prost / protobuf-legacy
                                       not needed for protobuf v4 cpp kernel
  .declare::<M>(name)                  primary declaration path (requires derive macro)

Activation
  .bind(name, message)                 primary binding path (requires derive macro)

Value::Struct(StructValue)             opaque; no public fields
  StructValue::from_prost(msg)         feature = "prost"
  StructValue::from_protobuf_legacy(m) feature = "protobuf-legacy"
  StructValue::from_message(msg)       feature = "protobuf" (future, zero-copy)
  StructValue::decode_prost::<M>()     feature = "prost"
  StructValue::decode_protobuf_legacy::<M>()  feature = "protobuf-legacy"
  StructValue::as_message::<M>()       feature = "protobuf" (future)
```

## Migration Path

The design is explicitly additive:

1. Today with prost: `Bytes` variant + `ProstValue` derive, serialization transport.
2. Today with protobuf-rust v3: `ReflectionMessage` variant + `ProtobufLegacyValue` derive,
   CustomStructValue transport (no serialization on input; round-trip recovery on output).
3. When protobuf v4 cpp kernel stabilizes: `CppMessage` variant + `ProtobufValue` derive,
   zero-copy transport. Existing prost/legacy derive users are unaffected.
4. Upgrading from prost to protobuf v4: change the derive macro attribute in `build.rs`,
   remove `with_file_descriptor_set`. The `declare`, `bind`, and extraction call sites
   are identical.
