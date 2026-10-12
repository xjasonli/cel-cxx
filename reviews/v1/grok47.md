# Review: Native Types design

Scope: `docs-cn/design/native-types/design.md` only, as the normative document. The findings below were checked against the design text, cel-cxx at the design's baseline behavior in this tree, cel-cpp `c84e883df5932dcc992dd3494a771c37e85ae1fe`, and prost / prost-build / prost-types 0.14.1. This review does not incorporate other review notes.

The design is detailed enough to implement against in many areas: registry closure checks, variant presence versus `Option`, builder slot states, and the decision to recheck imported ASTs are specified as contracts rather than slogans. The problems below are places where the normative text is internally inconsistent, specifies an API that does not compile, or leaves an algorithm that implementers will resolve in incompatible ways. Each item includes a replacement that can be applied to the design before implementation starts.

## 1. `Struct` / `TypedStruct` cannot be called on a concrete value

### Problem

Section 5.1 makes `TypedStruct` a subtrait of `Struct` and declares the same instance methods on both traits. The blanket `impl<T: TypedStruct> Struct for T` then forwards those methods. That is not the split used by the existing `Opaque` / `TypedOpaque` pair. `TypedOpaque` adds only the associated function `opaque_type()`; it does not redeclare `opaque_type(&self)`.

The following shape, which matches the sketch in section 5.1, fails on a concrete receiver:

```rust
let user = User; // User: TypedStruct, and therefore Struct via the blanket
let _ = user.get_field("age");
```

rustc reports `E0034: multiple applicable items in scope`, with one candidate on `impl Struct for T` and the other on `impl TypedStruct for User`. The design's remark that "UFCS can be used when necessary" understates this. Method call syntax is ambiguous for every concrete `TypedStruct` type, which is the type derive generates and the type hand-written adapters implement. A generic `fn f<T: TypedStruct>(t: &T)` can see only the supertrait bound and may compile, so the failure shows up at concrete call sites and inside generated method bodies, not at the trait definition. The trait definition itself is accepted; the API is not usable.

`definition()` versus `definition(&self)` is a different case. Those signatures do not collide in the same way, and that part of the `Opaque` analogy holds.

### Fix

Keep one method name on one trait. `TypedStruct` is the static definition and the derive hook. `Struct` is the only dynamic object API. Do not put `Struct` in the supertrait list: the blanket impl is what implements `Struct`, and a supertrait bound asks for `Struct` before that blanket can apply. Hooks use different names so a concrete value has one `get_field`.

Replace the section 5.1 traits with:

```rust
pub trait Struct: dyn_clone::DynClone + Send + Sync + private::Erased {
    fn definition(&self) -> StructDefinition;
    fn get_field(
        &self,
        name: &str,
        ctx: &mut ValueContext<'_>,
    ) -> Result<Value, StructAccessError>;
    fn has_field(&self, name: &str) -> Result<bool, StructAccessError>;
    fn eq_struct(&self, other: &dyn Struct) -> bool;
    fn is_zero(&self) -> bool;
}

pub trait TypedStruct: Clone + PartialEq + Send + Sync + 'static {
    fn definition() -> StructDefinition;

    fn read_field(
        &self,
        name: &str,
        ctx: &mut ValueContext<'_>,
    ) -> Result<Value, StructAccessError>;

    fn read_presence(&self, name: &str) -> Result<bool, StructAccessError>;

    fn is_zero_value(&self) -> bool {
        false
    }

    /// Used only when `definition().profile()` is Protobuf.
    /// Rust-profile types must not override equality here.
    fn protobuf_equal(&self, other: &dyn Struct) -> bool {
        other
            .downcast_ref::<Self>()
            .is_some_and(|rhs| self == rhs)
    }
}

impl<T: TypedStruct> Struct for T {
    fn definition(&self) -> StructDefinition {
        T::definition()
    }

    fn get_field(
        &self,
        name: &str,
        ctx: &mut ValueContext<'_>,
    ) -> Result<Value, StructAccessError> {
        self.read_field(name, ctx)
    }

    fn has_field(&self, name: &str) -> Result<bool, StructAccessError> {
        self.read_presence(name)
    }

    fn eq_struct(&self, other: &dyn Struct) -> bool {
        let def = T::definition();
        if other.definition().key() != def.key() {
            return false;
        }
        match def.profile() {
            Profile::Rust => other.downcast_ref::<T>().is_some_and(|rhs| self == rhs),
            Profile::Protobuf => self.protobuf_equal(other),
        }
    }

    fn is_zero(&self) -> bool {
        self.is_zero_value()
    }
}
```

Normative rules that go with this replacement:

- `#[derive(cel_cxx::Struct)]` implements `TypedStruct` only. It must not emit `impl Struct for T`. The blanket is the only `Struct` impl for a host type.
- A private view (`VariantView`, `SchemaDefault`, the Empty holder) implements `Struct` directly and does not implement `TypedStruct`. `private::Erased` stays a blanket over `Any + Send + Sync`, so a downstream `Struct` impl does not name the sealed trait. `Erased` has to be reachable as a supertrait of the public trait; model it on the existing `Opaque` seal rather than a trait that downstream crates must mention.
- Rust-profile `eq_struct` is closed over `PartialEq` inside the blanket. A hand-written Rust-profile type that needs different equality implements `PartialEq`, not a second comparison method. Protobuf-profile codegen overrides `protobuf_equal` and is the only supported override.
- `with_native::<T>()` continues to require `T: TypedStruct`. `Box<dyn Struct>` coercion works because the blanket implements `Struct`; it does not require `TypedStruct: Struct`.
- Concrete `user.get_field(...)` resolves only to `Struct::get_field`. Generic code that wants the derive hook calls `TypedStruct::read_field`. Add a compile-fail test that `user.get_field(...)` is unambiguous, and a compile-fail test that a type cannot implement both `Struct` and `TypedStruct`.
- Delete the sentence that treats UFCS as the way to disambiguate `get_field`. After this change, UFCS is only needed when both traits are explicitly named in a bound and the caller wants the hook rather than the object method.

Section 5.5's statement that `T: Struct` is not `T: TypedStruct` remains true. Update section 18.1 so the dyn-compatibility test calls `read_field` / `get_field` under these names.

## 2. The same Rust type does not have one CEL meaning, and narrowing is a truncating cast

### Problem

The design never gives a normative scalar and container map for Rust profile fields. Section 4.6 says "built-in scalars, string/bytes, Option, Box/Arc, list/map". Section 8.2 says field capability "converts according to the corresponding ability". Section 14.6 then uses `i32` and `String` as the example payload. Section 11.10 requires checked narrowing, but only for protobuf setters.

Today the library does not match that checked rule, and several Rust types already have more than one CEL reading:

| Rust type | What the library does now | Other reading the design also needs |
|---|---|---|
| `i32`, `i16`, `isize` | `TypedValue` is CEL int. `FromValue` uses `as i32` / `as i16` / `as isize`, which wraps. | Protobuf `Int32Value` is also `i32`, and a proto int32 field must reject out-of-range CEL ints. |
| `u32`, `u16`, `usize` | Same pattern, unsigned wrap. | Protobuf uint32 / fixed32 fields. |
| `f32` | `as f32`, so an out-of-range `f64` becomes infinity. | A proto float field. Section 11.10 says range-checked rounding. |
| `i8`, `u8`, `char` | No `TypedValue`. | Easy to assume they are scalars because neighboring integers are. |
| `Vec<u8>`, `Box<[u8]>`, `[u8]` | CEL bytes, not `list<int>`. | `BytesValue` is also generated as `Vec<u8>`. A repeated uint8 is not bytes. |
| `Option<f64>` | Rust profile: `optional<double>`. | Proto3 `optional double` is presence of a double. `google.protobuf.DoubleValue` is also `Option<f64>` because prost-build 0.14.1 maps that wrapper to `f64` and message fields are wrapped in `Option`. |
| `()` | CEL null. | prost-build maps `.google.protobuf.Empty` to `()`. prost implements `Name` and `Message` for `()`, with `NAME = "Empty"`. |

prost-build's default extern map is the source of the collisions, not an accident of naming:

- `.google.protobuf.DoubleValue` → `f64`
- `.google.protobuf.FloatValue` → `f32`
- `.google.protobuf.Int32Value` → `i32`
- `.google.protobuf.Empty` → `()`
- `.google.protobuf.BytesValue` → `Vec<u8>`
- `.google.protobuf.StringValue` → `String`

Message fields of those types are still `Type::Message`, so prost wraps them in `Option`. A proto3 `optional double` is a different prost attribute (`double, optional`) on the same `Option<f64>` type. `f64: prost::Name` reports `google.protobuf.DoubleValue` for every `f64`, including a plain proto3 double field. Classifying a field by `TypeId`, by `prost::Name`, or by "it is `Option<f64>`" selects the wrong CEL type.

`impl Message for f64` also treats `-0.0` as zero because it uses `!= 0.0`. Wrapper encode therefore drops the sign bit. That is a separate codec fact from in-memory equality, and section 11.5 records it only for plain float fields.

### Fix

Add a section 4.12, "Rust type is not a CEL type", and make sections 6.1, 8.2, 11.5, and 11.10 point at it. Field meaning is a property of the field descriptor in the definition, never of the Rust type alone. `prost::Name` and `prost::Message` on primitive carriers are not classifiers. In particular, do not write `impl<T: prost::Name> TypedValue for T` or any blanket that treats `()` as Empty or `f64` as DoubleValue.

Rust-profile derive uses this map and no other:

| Rust field type | CEL type | Decode from CEL | Encode to CEL |
|---|---|---|---|
| `bool` | bool | exact | exact |
| `i64` | int | exact | exact |
| `i32`, `i16` | int | reject unless the value is in range for that type | widen with `as i64` |
| `isize` | int | reject unless the value fits the target pointer width | widen |
| `u64` | uint | exact | exact |
| `u32`, `u16` | uint | reject unless the value is in range | widen |
| `usize` | uint | reject unless the value fits the target pointer width | widen |
| `f64` | double | exact, including NaN and `-0.0` | exact |
| `f32` | double | finite values with magnitude `<= f32::MAX` use round-ties-to-even; magnitude above that is a conversion error, not infinity; NaN and both signed zeros are preserved | widen |
| `String` | string | UTF-8 string | owned `String` |
| `Vec<u8>`, `Box<[u8]>`, `[u8]` | bytes | copy bytes | owned bytes |
| `Vec<T>`, `VecDeque<T>`, `LinkedList<T>` where `T: TypedValue` and `T` is not `u8` | list | element rules | element rules |
| `HashMap<K, V>`, `BTreeMap<K, V>`, `Vec<(K, V)>` where `K` is bool, int, uint, or string | map | element rules | element rules |
| `Option<T>` | `optional<τ(T)>` | strict optional decode from section 6.3 | preserve None / Some |

Explicitly unsupported in v1, with a derive diagnostic that names the field: `i8`, `u8`, `char`, `NonZero*`, arrays `[T; N]`, `std::time::Duration`, `SystemTime`, `PathBuf`, raw pointers, and `bytes::Bytes` on a Rust-profile field. Protobuf-profile codegen may still use `bytes::Bytes` when the prost config selects it, because that path reads prost attributes rather than this table.

`Vec<u8>` stays bytes. A protobuf repeated `uint32` or a Rust `Vec<i64>` is a list. A protobuf `bytes` field and a `BytesValue` field are bytes, but `BytesValue` is nullable: missing wrapper is null, `Some(empty)` is empty bytes. Those two states must not both become `Vec<u8>`'s Rust-profile reading.

Checked narrowing is global. It applies to `FromValue` / `FromOwnedValue`, Rust-profile field reads and builder slots, and protobuf setters. The current `as i32` / `as f32` implementations are a breaking change and belong in section 15.4. Overflow is a `FromValueError` outside evaluation and a CEL error inside a field or setter. It is not wraparound and not saturation.

Protobuf-profile metadata records a `FieldKind` that `Option<f64>` by itself cannot express:

| `FieldKind` | Typical Rust carrier | CEL read | `has` |
|---|---|---|---|
| `RustOptional` | `Option<T>` on a Rust-profile field | `optional<τ(T)>` | always true |
| `Proto3ImplicitScalar` | `f64`, `i64`, `String`, `Vec<u8>`, ... | the scalar, including the zero value | non-default; float uses non-zero bits, so `-0.0` and NaN are present |
| `ProtoOptionalScalar` | `Option<f64>` with prost `double, optional`, or the proto2 equivalent | scalar, default when `None` | `Option::is_some` |
| `Wrapper` | `Option<f64>` with prost `message, optional` and FDS type `DoubleValue` | primitive or null | wrapper message presence, not "inner value is non-zero" |
| `EmptyMessage` | `Option<()>` whose FDS type is `google.protobuf.Empty` | named `google.protobuf.Empty` | `Option::is_some` |
| `Message` | `Option<M>` or a non-optional `M` | message or schema default view | message presence |

A Rust function that returns `Option<f64>` uses `RustOptional` even if the `f64` was copied out of a wrapper field. Moving a carrier out of a message drops the field kind. Section 14.4's `next_age(person.?age)` remains the supported way to turn presence into `optional<int>`. Add the matching negative test: `next_age(person.age)` is a type error when `person.age` is a proto optional int64, because that read is `int`, not `optional<int>`.

Record the wrapper codec limit next to the plain-float limit in section 11.5: prost's `Message for f64` / `f32` uses `!= 0.0`, so encoding a `DoubleValue` or `FloatValue` drops `-0.0`. In-memory `has` and equality still see the stored bits. They are not required to match the bytes produced by prost's wrapper encoder.

## 3. Protobuf equality reuses the wrong comparison and can overflow the stack

### Problem

Section 12.2 says standard derived `PartialEq` may be reused, and that `Option<f64>` holding `Some(+0.0)` and `Some(-0.0)` compares equal when presence is explicit. That row is true for a proto3 `optional double`. It is false for a collapsed wrapper, which has the same Rust type.

`google.protobuf.DoubleValue.value` is a proto3 implicit double. prost stores the parent field as `Option<f64>`:

- `None` means the wrapper message is absent.
- `Some(+0.0)` means the wrapper is present and the inner implicit field is the default.
- `Some(-0.0)` means the wrapper is present and the inner field is present, because its bits are non-zero.

`MessageDifferencer::Equals` treats those two `Some` values as unequal. Derived `PartialEq` treats them as equal because `+0.0 == -0.0`. Applying the "explicit optional" row whenever the carrier is `Option<f64>` implements the wrong language. The same split exists for `Option<f32>` and `FloatValue`. Nested messages and map values of wrapper type have the same inner rule. The parent `Option` presence is the wrapper message presence; it does not turn the inner field into an explicit optional scalar.

The second part of the same section prefers derived `PartialEq` for real prost messages. Derived equality is recursive. Section 11.8 rejects cycles only on required message edges. Optional, repeated, map, and oneof message edges may cycle. `prost_types::Value`, `Struct`, and `ListValue` are cyclic and are in the supported WKT set. Section 12.2 compares storage, not CEL getters, so a user message that contains a deep `Value` still walks the prost tree.

Section 12.5 says a library-written comparison plan should use an explicit work stack, but the primary path is derived `PartialEq`, which has no work stack and is not covered by `max_native_depth`. Section 16 forbids a panic or unwind from crossing FFI. A stack overflow aborts the process instead. An untrusted `a == b` on a long `Node { optional Node next = 1; }` chain is enough. Host `PartialEq` for ordinary Rust types is a different contract: section 4.6 already requires the host predicate to terminate, and the library cannot rewrite arbitrary user code. Generated prost equality is the library's code.

### Fix

Replace the comparison-selection bullets in section 12.2 with this procedure. It runs after the definition and representation check. It does not call CEL getters, does not encode, and returns `bool`.

1. If either side is a schema default view, compare presence against the schema. Do not call `M::default()` or `M::PartialEq`. A real required field is present. The same field on a default view is absent. Different presence is `false`.
2. Otherwise both sides are real `M`. Walk fields from the frozen descriptor, not from `PartialEq`.
3. For a field whose kind is `Proto3ImplicitScalar` of float or double, compare `to_bits()`. Equal bits are equal. `+0.0` and `-0.0` are not equal. NaN bits compare by identity of the payload only as far as `to_bits` does; do not use `==`.
4. For a field whose kind is `ProtoOptionalScalar` or a oneof scalar float, if both sides are present, use numeric `==` (`+0.0` equals `-0.0`; NaN is not equal to NaN). If presence differs, the field is not equal.
5. For a `Wrapper` of `FloatValue` or `DoubleValue`, compare wrapper presence first. If both are present, compare the inner value with rule 3, not rule 4. This applies when the carrier is `Option<f32>` / `Option<f64>` and the FDS type is the wrapper. A proto3 `optional double` stays on rule 4 even though it is also `Option<f64>`.
6. Repeated fields compare in order. Map fields compare as sets of entries. Message fields recurse with this procedure. Bytes compare by content.
7. Non-float scalars, strings, enums, and bools may call the corresponding derived field comparison. That is a per-field subroutine, not a call to `M: PartialEq` for the whole message.

Derived `M: PartialEq` is a legal subroutine only when the generator proves the message graph is acyclic after deleting float/double fields and wrapper fields that need rule 3. A cycle through optional, repeated, map, oneof, or through `google.protobuf.Value` / `Struct` / `ListValue` disqualifies the whole-message `PartialEq`. The generator then emits the walker above. The walker uses an explicit heap stack of `(message pointer, field index)` and a generation-stamped color on the current pair of addresses so a cyclic pair returns "equal so far" instead of looping. Address identity is only the cycle brake for this walk. It is not an equality success shortcut for NaN or for distinct messages.

Generated `Clone` for a cyclic prost message uses the same shape of heap stack. Do not `#[derive(Clone)]` on those messages and then call it from the CEL output path. Acyclic messages may keep derived `Clone`.

Ordinary Rust-profile equality stays on `T::PartialEq`. A stack overflow in user `PartialEq` remains a violated host contract, not a CEL `false`.

Acceptance to add to section 18.6:

- Parent fields `Option<f64>` from `optional double`: `Some(+0.0)` equals `Some(-0.0)`.
- Parent fields `Option<f64>` from `DoubleValue`: those two values are not equal. `None` versus `Some(0.0)` is not equal. Map values and nested messages of `DoubleValue` use the same inner rule.
- `message Node { optional Node next = 1; int32 v = 2; }` with a chain of 50,000 nodes: `a == a` is `true`, `a == b` is a bool, and the process does not abort. The same depth for a `google.protobuf.Value` list nested inside a user message is a bool, not an abort.
- A Rust-profile struct whose `PartialEq` panics still follows section 16, not this walker.

## 4. The conversion budget rejects ordinary values and does not measure the walk it claims to limit

### Problem

Section 12.5 charges one operation for every Native field read, every `Option` / `Box` / `Arc` layer, and every list or map element. The defaults are `max_native_depth = 128` and `max_native_operations = 100_000`. Bytes add one unit per 64 bytes, rounded up, with a minimum of one.

Those defaults do not survive the values the rest of the design allows:

- A repeated field of 100,000 scalars costs 100,000 element charges before any field of the element is read. The limit is 100,000, so the last element fails or the whole list fails depending on whether the check is `>` or `>=`. The design does not say which.
- One 10 MiB `bytes` field costs `ceil(10 * 1024 * 1024 / 64) = 163,840` units on the copy alone, above the default, while section 18.7 only requires a 1 MiB sample to pass.
- A prost message with a few thousand sub-messages, which is a normal descriptor or a batch of events, exhausts the counter while evaluating `items.size()` or converting the value into Rust if the converter visits every element.
- `has`, zero, type, and debug are free, but equality is also outside the counter and, until issue 3 is fixed, outside the depth limit too. The counter therefore limits inexpensive scalar visits and does not limit the recursive comparison that can abort the process.

The counter is shared for a whole evaluation. That part is right. The unit of charging is not.

### Fix

Replace the charging paragraph in section 12.5 with three independent limits. Each is checked before the corresponding allocation or recursive call. Exhaustion marks the operation exhausted and returns one CEL resource error inside evaluation, or the existing local conversion error outside it. Later billable work in that operation fails the same way. Pure metadata, `has`, zero, type name, debug, moving an already-built builtin, and propagating an existing Error or Unknown do not bill.

| Limit | Default | What consumes one unit |
|---|---|---|
| `max_native_depth` | 128 | Entering one Native struct, one synthetic variant record, one schema-default message, or one `Option` / `Box` / `Arc` layer that still contains a Native or container value. Scalar leaves do not add depth. |
| `max_native_values` | 1,000,000 | One struct or variant record being read or materialized, one list element, or one map entry. Reading `.size()` or testing map membership bills one, not one per element. |
| `max_native_bytes` | 32 MiB | Bytes the library itself copies into a new string, bytes, list, or map buffer. A borrow of an existing buffer bills zero. The charge is the copy size, not `ceil(n / 64)`. |

Field reads of a scalar inside a struct are covered by the one charge for that struct. They do not each cost another value. Builder slots use the same table: one value per initialized field, plus bytes for copied strings and bytes, plus depth for a nested message.

State the inclusion rule: a value of `N` elements is accepted when `N <= max_native_values` at the start of the walk. The 1,000,000th element succeeds. The next one fails.

Equality and the cyclic clone walker from issue 3 do not consume `max_native_values`. They have their own heap stack and cannot return a false bool to signal budget failure. A non-cyclic library clone that copies a large string still consumes `max_native_bytes`.

Replace the section 18.7 byte example with three gates that must pass at the defaults:

- `items.size()` and `items[0]` on a repeated scalar field of 100,000 elements.
- A 10 MiB bytes field read once, billed as 10 MiB against `max_native_bytes`.
- A linked message list of depth 128 reads the leaf. Depth 129 is a resource error. The equality test from issue 3 stays outside this depth counter.

Host `Clone` that copies a megabyte inside `T::clone` remains unbilled. The acceptance text should keep that sentence so a large host clone is not mis-reported as a budget bug.

## 5. One status classifier cannot also be the Unimplemented fallback, and zero-panic must not return false

### Problem

Section 10.2 says every status produced by this library carries a private classification. Domain status becomes `OK` plus a CEL error. Infrastructure status is returned unchanged. Section 9.3 simultaneously says ordinary `GetFieldByNumber`, `HasFieldByNumber`, and `Qualify` return `Unimplemented` so the name-based path still runs.

Those are different protocols. In cel-cpp `extensions/select_optimization.cc`, `OptimizedSelectImpl::ApplySelect` falls back to field-by-field selection only when `status.code() == absl::StatusCode::kUnimplemented`. A domain classification that rewrites the status into `OK` plus an error never reaches that test. An infrastructure classification that keeps the code but attaches a payload is still `Unimplemented`, so a genuine internal failure in `Qualify` would also take the fallback and hide the failure. `PerformHas` in `eval/eval/select_step.cc` has the opposite bug today: every non-OK `HasFieldByName` becomes a CEL error, including infrastructure. The design already asks to split those. It does not leave a third channel for "the virtual method is not implemented, use the caller's fallback."

`IsZeroValue` is a separate hole. The C++ callback returns `bool` and has no arena, so it cannot use the operation context from section 9.6. Section 12.4 says a panic in the zero callback returns false and is reported. Section 16 says an equality panic is a non-OK status and must not become false. A false zero result is a successful bool. It participates in conditionals. Logging beside it does not undo that. There is no later status return on which a sticky flag could ride, because this call has no status.

### Fix

Define three library status classes. The normalizer runs only at the FFI boundary of this wrapper. It must not rewrite a status that is already a raw protocol signal.

| Class | Who produces it | What the caller sees |
|---|---|---|
| Domain | Unknown field, unselected variant read, type mismatch, closed enum, missing required, failed user validation | `absl::OkStatus` and a CEL `ErrorValue`. `HasFieldByName` cannot return a CEL value, so its domain failure is a status with the private domain payload; `PerformHas` and `PerformOptionalGet` translate that payload into a CEL error and return OK. |
| Infrastructure | Broken invariant, failed allocation outside the CEL budget, poisoned builder, panic in equality | The original non-OK status and its payload, unchanged. `PerformHas` / `PerformGet` return that status. They do not wrap it in `ErrorValue`. |
| Protocol unsupported | Ordinary Native `GetFieldByNumber`, `HasFieldByNumber`, and `Qualify` | `absl::UnimplementedError` with no domain or infrastructure payload. The normalizer does not touch it. |

`Qualify` on a protobuf-profile value may be implemented for real later. In v1 it uses the same raw `Unimplemented` as the default `CustomStructValueInterface::Qualify`. Select optimization's existing fallback then stays correct if optimization is ever enabled. Until then, section 10.3 still rejects select optimization when the Native registry is non-empty. The raw `Unimplemented` is what keeps a future enablement from turning every Native select into an infrastructure error.

Unselected variant `get` is domain, not an `Ok(Value::Error)` manufactured in Rust and also a `StructAccessError`. Pick one producer: `read_field` returns `Err(StructAccessError::Domain)`, and the wrapper performs the only translation to a CEL error. A CEL error that arrived as data inside a `dyn` field is passed through as `Ok(Value::Error)` and is not classified again.

Change the cel-cpp `IsZeroValue` callback that this wrapper implements from `bool` to `absl::StatusOr<bool>`. Update the custom-struct dispatcher and every caller the pin actually has. Panic and infrastructure failure return the non-OK status. A successful false means "this value is not the zero value", which remains the Rust-profile default when `is_zero_value` is the default method. Do not catch a panic and return false. Section 12.4's sentence about a conservative false result should be deleted. `DebugString` can keep a conservative placeholder string because it is not a boolean input to the expression; it still must not unwind.

Section 18.6 should assert three observable results: a domain field error is a CEL error that `|| true` can swallow; an injected infrastructure status from `HasFieldByName` aborts evaluation with that status; `Qualify` returning bare `Unimplemented` still selects by name under the optimizer fallback test, and the same status with a domain payload does not.

## 6. Prost field `Option` is missing from the six-position contract

### Problem

Section 6.1's table is the normative "same `Option` in every position" contract. Its message-field row is "future v4". The first version's message type is prost. The paragraph under the table corrects this in prose, and section 11.5 contradicts the table: a prost `age: Option<i64>` reads as `int`, while the same `Option<i64>` bound as a variable is `optional<int>`.

That split is implementable, but it is not in the table an implementer will treat as exhaustive. It is also incomplete at the function boundary. This function is legal Rust and has no specified CEL type:

```rust
fn age_of(person: Person) -> Option<i64> {
    person.age
}
```

The return type is `optional<int>` by the Rust `Option` rule. `person.age` inside CEL is `int` by the field rule. Both describe one prost field. Callers will write `age_of(person)` and `person.age` and expect the same type. The design never says the return crosses the Rust conversion boundary and therefore drops field kind.

A second missing row is `has(e.?C.x)`. Section 4.10 specifies `e.?C.x` and `has(e.C.x)` and does not specify the combination. The parser produces `has((e.?C).x)`. The receiver of the outer select is optional. The existing row for `has(m.C)` says an empty optional receiver is false. An implementation that treats every path containing an unselected variant as an error will reject a legal expression.

### Fix

Replace the section 6.1 table with seven positions. Keep the future v4 note as a footnote, not as the message-field row.

| Position | Contract |
|---|---|
| Variable, including a prost carrier moved out of a field and then bound | `Option<T>` is `optional<τ(T)>`. `None` is a present binding of `optional.none`. `has(x)` is not the existence test. |
| Expression result and `FromValue` | Only a CEL optional decodes to `Option`. Null, a bare `T`, and a missing protobuf field do not. |
| Function parameter | The declared Rust type is the CEL type. `Option<i64>` means `optional<int>`. A proto field read of type `int` does not match. Use `person.?age` to produce the optional. |
| Function return | `Ok(Some/None)` is an optional. The body is ordinary Rust. Returning `person.age` from `-> Option<i64>` applies `RustOptional` at the return boundary. It does not preserve protobuf presence, and it is not equivalent to the CEL expression `person.age`. |
| Rust-profile field | `Option<T>` is `optional<τ(T)>`. `has(obj.field)` is true for `None`. |
| Prost-profile field | Use `FieldKind` from issue 2. Presence `Option` is not CEL optional. `person.age` is the scalar or the default. `person.?age` is `optional` of that scalar. `has(person.age)` is protobuf presence. |
| Independent use of a v4 `_opt()` result | When that API exists, the returned `Option` is position 1. The message field itself stays on the schema rule. Not a v1 test. |

Add this row to the section 4.10 expression table, and the same assertion to section 18.3:

| Expression | Contract |
|---|---|
| `has(e.?C.x)` | Parse as `has` of a field select whose receiver is `e.?C`. If `C` is unselected, the receiver is `optional.none` and the result is false. If `C` is selected, the result is `has` of `x` on that record, which is true for an exposed field. This does not evaluate the failing read `e.C`. `has(e.C.x)` remains an error when `C` is unselected. `has(e.?C)` remains a parse/macro error. |

Add an API test, not only a CEL test, for `age_of(person)` versus `person.age`. The first result decodes as `Option<i64>`. The second, when the field is unset, decodes as `i64` equal to the protobuf default, and `into_owned::<Option<i64>>()` fails with a type mismatch.

## 7. Imported checked ASTs still trust source-env enum numbers

### Problem

Section 9.2.7 drops `type_map`, `reference_map`, and overload ids, then typechecks the expression again. It also says constant nodes keep their CEL values, and an enum constant that has already been folded to int does not recover its source. The same section says the import does not trust old metadata.

Those two sentences describe different nodes. An identifier or select that still says `example.Job.State.READY` is re-resolved and should pick up the target environment's number. A const node that the source planner folded from that identifier is a bare `1`. The target environment cannot see that it was an enum. If the target defines `READY = 2`, the imported program keeps `1` and looks successful. Section 9.2.6 already admits this and then section 9.2.7 claims the import removed trust in old metadata. The acceptance text in section 18.2 checks `User.age` changing from int to string. It does not check an enum number changing under an imported `READY` reference.

Folding is not hypothetical. The planner's constant folder replaces a resolved enum reference with a const. A checked AST saved after that pass, or any future exporter that serializes the planned expr, carries the source number.

### Fix

Specify the import rule by node kind. Apply it in step 2 of section 9.2.7 before `TypeChecker::Check`.

- Identifier and select expressions are kept and re-resolved in the target environment. `example.Job.State.READY` therefore becomes whatever number the target env publishes. If the target has no such constant, import fails with the target's resolution error.
- Const nodes that the user wrote as literals (`1`, `"ready"`, `true`) stay literals.
- A const node that replaced an enum reference must carry the enum full name and member name beside the number. On import, resolve that pair in the target env. If the target number differs, replace the const with the target number. If the member is absent, fail the import. Do not keep the source number.
- If the AST proto used by this pin cannot store that pair, the exporter must not serialize a folded enum const. It serializes the original identifier or select, and the import path above re-resolves it. Constant folding remains a planner-local optimization and is not part of the portable checked AST.
- A const with no enum annotation is a literal. Import does not try to guess which enum it came from.

Add the section 18.2 case: source env `READY = 1`, target env `READY = 2`, expression `example.Job.State.READY`. After import, the value is `2` and the type is int. The same expression already folded to a const without an annotation is rejected as a non-portable AST, not evaluated as `1`. The literal expression `1` still imports as `1`.

## 8. The evaluation operation is not connected to the functions that need it

### Problem

Section 9.6 correctly says cel-cpp passes `embedder_context` only to overloads whose descriptor is contextual. The current `function_step.cc` does this in one place: `if (overload.descriptor.is_contextual())`. Section 7.1 says every parameter decode uses the same `ValueContext`. It never says which overloads are registered as contextual.

A function `fn f(user: User) -> User` decodes a Native argument and validates a Native result. If it is registered as an ordinary overload, the call has no embedder context. The decoder then either fails closed, or calls `ValueContext::standalone()` and starts a fresh budget, which section 6.2 forbids on an inner conversion. A function `fn f(n: i64) -> User` has no Native argument and still needs the context on the way out. A function `fn f(v: Value) -> Value` can return a Native that was not visible in a scalar parameter list.

Section 7.3 says the async future must drop the mutable borrow of `ValueContext` before `.await` and that the operation guard outlives the future. It does not say how the future obtains a mutable context again to encode the return value. `ValueContext` contains `&mut`-driven budget counters. Holding that borrow across `.await` makes the future non-`Send` or rejects the code. Creating a second `standalone()` context resets the budget.

### Fix

Add a registration rule to section 7.1 and a call sequence to section 7.3.

An overload is contextual if any of these is true after expanding `Option`, `Box`, `Arc`, list, map, and the success type of `Result`:

- a parameter or the success return type is a `TypedStruct` host, a synthetic variant record, or `Value` / `dyn`;
- a parameter or the success return type contains one of those inside a container or optional;
- the overload is a borrowed or lending function, because the lease is tied to the operation guard.

Scalar-only overloads, including `Option<i64>`, `Vec<i64>`, and `Result<i64, E>`, stay non-contextual. Their decoders must not touch a Native operation. If a contextual overload is invoked and `embedder_context` is null, that is an infrastructure error. The decoder must not call `standalone()` as a fallback.

`standalone()` remains legal only for `Value::decode` / `into_owned` and the trait convenience methods when the caller is outside an evaluation. Those methods are not used by function adapters. Make that a type-level split if necessary: the public convenience methods live on `Value`, and the traits expose only `*_with_context`, so a function adapter cannot call the resetting entry by accident.

The lending async body is normative, not an example:

```rust
fn call<'a>(&'a self, args: S::Decoded<'a>)
    -> Pin<Box<dyn Future<Output = Result<Value, Error>> + Send + 'a>>
{
    Box::pin(async move {
        let decoded = {
            let mut ctx = guard.value_context()?; // mutable borrow starts
            decode_args(&args, &mut ctx)?
        }; // mutable borrow ends before the await
        let rust_result = user_future(decoded).await?;
        let mut ctx = guard.value_context()?; // new mutable borrow after the await
        encode_return(rust_result, &mut ctx)
    })
}
```

`NativeOperationContext` is owned by the guard and uses interior mutability for the counters, so a new `ValueContext` can be built after the await without sharing a `&mut` across the await point. The guard, the argument lease, and the future are dropped together on completion and on cancel. The returned `Value` must not borrow the guard. Add a compile-fail test that a future holding `&mut ValueContext` across `.await` is rejected by the adapter, and a runtime test that an await point does not reset `max_native_values`.

## 9. The zero-clone gate has no ownership API

### Problem

Section 5.4 and section 18.7 require a measurable result: when a full root is consumed, one owned `T` is returned, evaluation does not need a copy, and no source remains, `T::clone` runs zero times. `m` outputs of that root cost `m - 1` clones if the source is consumed, and `m` clones if the source is retained.

`Program::evaluate` already takes the activation by value, but the design never defines which activation values are movable. A binding that still has a Rust reference in the caller, a program constant, or a second name bound to the same object is "a source that remains". The extractor cannot see the caller's stack. Without a typed distinction between a moved binding and a borrowed binding, every implementation either clones defensively and fails the zero-clone test, or moves out of a binding the caller still holds.

Section 5.4 also says a C++ `unique_ptr<cel::Value>` does not prove the host is uniquely owned, and that tokens may alias the same `Box<T>` until the last C++ view is released. The clone counter is unspecified about token copies versus `T::clone`. An implementation can pass the test by counting the wrong object.

### Fix

Define the activation surface in section 5.4. `evaluate` continues to take the activation by value.

```rust
pub struct OwnedActivation { /* names -> OwnedBinding */ }
pub struct BorrowedActivation<'a> { /* names -> BorrowedBinding<'a> */ }

pub enum OwnedBinding {
    /// The activation owns this value. evaluate may move it.
    Value(Value),
}

impl OwnedActivation {
    pub fn bind_owned(&mut self, name: impl Into<String>, value: impl IntoValue) -> Result<(), Error>;
}

impl<'a> BorrowedActivation<'a> {
    /// The caller keeps the host. Evaluation may borrow it and must clone to return it.
    pub fn bind_ref<T: TypedStruct>(&mut self, name: impl Into<String>, value: &'a T) -> Result<(), Error>;
}
```

`Program::evaluate(OwnedActivation)` and `Program::evaluate(BorrowedActivation<'_>)` are the two entry points. There is no API that borrows an `OwnedActivation` and also promises zero clones. Dropping the activation at the end of `evaluate` is what "no source remains" means. If the caller needs the input after the call, they use `BorrowedActivation` or they clone before `bind_owned`. That clone is the caller's and is not part of the library counter.

The library counter counts `T::clone` and `Clone` impls generated for prost hosts. It does not count:

- copies of the C++ `CustomStructValue` token;
- `Arc` clones of the eval-storage handle;
- clones inside `T::clone` of fields that the host's own `Clone` performs, except when the test fixture is the deep-copy chain in section 18.7 and is explicitly counting those.

Movement rule, evaluated after the C++ views for that activation are released and before the activation is dropped:

| Binding | Result aliases of that binding | `T::clone` |
|---|---|---|
| Owned, and the result is that value | 1 | 0, then the `Box<T>` is moved out |
| Owned | `m > 1` | `m - 1`, then the last result is moved |
| Borrowed, or the caller used a program constant / cache | `m` | `m` |
| Owned, but the result is a field projection | any | the projection's own copy rule in section 5.2; the root is not moved out of the field |

"The result is that value" means the planned expression is the variable, or a list/map/optional constructor whose every leaf is that same variable and whose output slots are being filled. A field select, a function that returns a new `T`, or a comparison is not an identity result. Identity is decided from the plan, not by comparing pointers after the fact and skipping a clone that `PartialEq` would have required.

If any C++ view of an owned binding is still alive at extraction, extraction clones and does not move. That clone counts as a library clone and fails the zero-clone test. The test is the enforcement. Add it as a debug assertion in the extractor: move only when the token strong count is one.

Section 18.7 stays the acceptance test. Attach it to these types so a passing count cannot come from a borrowed activation that happened to be wrapped in `Arc`.

## 10. Enum policy and the built-in Syntax set are not algorithms

### Problem

Section 11.7 says open enums keep unknown numbers, closed enums reject them, and "a proto2 field that references an imported open enum uses the current C++ legacy closed rule." It does not name the function, does not say what happens for a proto2 field of a same-file proto3 enum, and does not say what a proto3 field of a proto2 enum does. "Imported" is narrower than the C++ rule implementers will actually need to match. Map, repeated, and oneof entries say they use "the field policy" without saying the policy is a property of the field, not of the enum type. Two generators can both claim to follow section 11.7 and disagree on `123`.

Section 11.12 then requires the built-in `google.protobuf.Syntax` constants to be the protobuf 35.1 set, including `SYNTAX_EDITIONS = 2`, while `prost_types::Syntax` can only decode `0` and `1`. Section 9.2.6 says two explicit enum definitions merge only when the full name-to-number set is identical. A user `FileDescriptorSet` produced by an older `protoc` still contains `google.protobuf.Syntax` with `{0, 1}` as soon as it contains descriptor.proto, which is common. Registering prost-types metadata in that environment, or collecting FDS enums into the same table, rejects the whole `Env`. The design states the 35.1 set and the merge rule separately and never says what an older well-known enum should do.

Rejecting editions as a schema language is right. Rejecting every environment that embeds an older `Syntax` enum is a different decision, and it is not written down.

### Fix

Add this procedure as the definition of effective enum policy. The generator and the C++ pool path both call it. Policy is stored on the field, including each repeated element, map value, and oneof member. It is not stored on the enum type alone.

Use the protobuf 35.1 predicate as the source of truth for the C++ pool path. The implementation in that release is:

```cpp
bool FieldDescriptor::legacy_enum_field_treated_as_closed() const {
  return type() == TYPE_ENUM &&
         (features().GetExtension(pb::cpp).legacy_closed_enum() ||
          enum_type()->is_closed());
}
```

`cpp.legacy_closed_enum` defaults to true for legacy/proto2 and false for proto3 (`cpp_features.proto` edition defaults). `EnumDescriptor::is_closed()` is true for a proto2 enum and false for a proto3 enum. A proto2 field of an open proto3 enum is therefore closed because the field feature is true, not because the enum was imported. The design's word "imported" is narrower than this predicate.

For prost, where that method is not available at codegen, compute the same bit from the FDS:

| Field file syntax | Enum definition | Effective policy |
|---|---|---|
| proto3 | proto3 enum | open |
| proto3 | proto2 enum | closed |
| proto2 | proto2 enum | closed |
| proto2 | proto3 enum, imported or reached through a dependency | closed, because the field's `legacy_closed_enum` is true |
| any | enum with `features.enum_type = CLOSED` once editions exist | closed; v1 does not reach this row because editions are rejected at build |
| any | `google.protobuf.NullValue` as an ordinary field | the policy of that field; the CEL type is still int, per section 11.7 |

Unknown numbers are kept on open fields and are a CEL error on closed fields at read, assign, and materialize. `E::try_from` is only for recovering a Rust enum that has that variant. It is not the storage of an open field. The acceptance rows already in section 18.5 should name one fixture for each row of the table, including a proto2 file that imports a proto3 enum and both accepts only known numbers on that field and still allows the proto3 enum to be open when used from a proto3 file.

For well-known enums shipped inside cel-cxx, separate the constant table from the carrier again, and define the merge with user FDS:

- The Rust carrier decode set stays `{0, 1}` for `prost_types::Syntax`. Decoding `2` is an unknown-variant error. This does not remove the constant `SYNTAX_EDITIONS`.
- When the user pool's `google.protobuf.Syntax` is a prefix of the pinned 35.1 set (same numbers for the names it has, missing only later names), the environment keeps the pinned superset. It does not reject the environment and does not append the sets in registration order. A user number that disagrees with the pinned set, or an extra user name, is still `AlreadyExists`.
- Apply that prefix rule only to the well-known enums listed in section 11.12. User enums keep the exact-set rule in section 9.2.6.
- Publishing `SYNTAX_EDITIONS = 2` does not enable edition schemas. `compile_protos` still rejects `syntax = "editions"` and `edition = ...` before prost-build parses them.

The section 18.5 Syntax test should load a descriptor set whose `Syntax` enum has only `PROTO2` and `PROTO3`, register the prost-types adapter, and still resolve `SYNTAX_EDITIONS` as `2`. A descriptor set that defines `SYNTAX_PROTO2 = 7` fails the build.

## 11. Generic hosts are required to have a name strategy and are not given one

### Problem

Section 4.1 requires every generic instance to have a distinct legal CEL name, says the derive fails if the strategy is missing, and says the strategy is a pure function that runs before field dependencies are loaded. It also says `type_name::<T>()` must not be copied into a CEL identifier, and that `i32` and `i64` cannot be distinguished by the CEL type name `int`. Section 4.2 lists derive attributes and does not include a generic naming attribute. `#[cel_cxx(type_name = "example.User")]` is a literal, so it cannot name `Node<i32>` and `Node<i64>` differently.

Const generics are required to participate in the name, and there is no syntax for them either. Two lawful readings of "pure name strategy" are a format string and a function item. They behave differently for raw identifiers, const generics, and lifetimes that have already been erased. The derive cannot be specified until one of them is chosen.

### Fix

Add the attribute to section 4.2. A generic `struct` or `enum` must have exactly one of these. A non-generic type keeps `type_name` and must not also have `type_name_with`.

```rust
#[derive(Clone, PartialEq, cel_cxx::Struct)]
#[cel_cxx(type_name_with = node_name)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

fn node_name(args: cel_cxx::TypeNameArgs) -> cel_cxx::TypeName {
    // args are Rust type identities, not CEL type names.
    cel_cxx::TypeName::from_static("example.Node")
        .segment(args.type_param::<T>())
}
```

`TypeNameArgs` is generated per type and exposes one method per type parameter and const parameter. `type_param::<T>()` returns a `TypeNameSegment` built from a stable spelling the macro emits from the Rust path of the concrete argument after alias normalization: `i32` and `i64` are different segments even though both are CEL `int`. Const parameters use their decimal or boolean literal. The function is a `const fn` if the Rust version allows the body the user wrote; the macro invokes it while expanding the concrete impl, not at `Env` build and not while walking field dependencies. It cannot call `definition()`, register a type, or read an `Env`.

The macro checks the result before it looks at fields:

- every segment is a CEL identifier, or an explicit quoted identifier if the parser option is on;
- the full name is unique among concrete impls of this generic;
- the same concrete `TypeId` always expands to the same string;
- a missing attribute on a generic type is a compile error that points at the type, not at the first field;
- a `type_name_with` function with the wrong signature is a compile error.

Lifetimes do not appear. A type that still has a lifetime parameter is rejected by section 4.9 before naming. Section 18.1's generic test should cover `Node<i32>` and `Node<i64>` receiving different CEL names, and `Node<i32>` compiled in two crates with the same function body receiving the same name.

## What this review did not treat as a defect

The variant-presence versus payload-`Option` split, the refusal to fall back from a Native field miss to a same-named protobuf descriptor, rechecking imported ASTs for ordinary type changes, and the separation of pool closure checks from field projection are consistent and should stay as written. The issues above are the contracts I would change before an implementation plan treats the current text as settled.
