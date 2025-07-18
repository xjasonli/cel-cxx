# CEL Extensions

## Table of Contents

- [CEL Extensions](#cel-extensions)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
  - [2. Available Extensions](#2-available-extensions)
    - [2.1 Strings Extension](#21-strings-extension)
    - [2.2 Math Extension](#22-math-extension)
    - [2.3 Lists Extension](#23-lists-extension)
    - [2.4 Sets Extension](#24-sets-extension)
    - [2.5 Regular Expression Extension](#25-regular-expression-extension)
    - [2.6 C++ Regular Expression Extension (re)](#26-c-regular-expression-extension-re)
    - [2.7 Encoders Extension](#27-encoders-extension)
    - [2.8 Bindings Extension](#28-bindings-extension)
    - [2.9 Protocol Buffers Extension](#29-protocol-buffers-extension)
  - [3. Enabling Extensions](#3-enabling-extensions)
  - [4. Extension Compatibility](#4-extension-compatibility)

## 1. Overview

CEL extensions provide additional functionality beyond the standard library. These extensions are optional and must be explicitly enabled in the CEL environment. They offer specialized functions for common use cases in various domains.

Extensions are designed to be:
- **Modular**: Each extension can be enabled independently
- **Safe**: All extensions maintain CEL's safety guarantees
- **Performant**: Extensions are implemented efficiently in native code
- **Composable**: Extensions can work together seamlessly

## 2. Available Extensions

### 2.1 Strings Extension

**Documentation**: [strings.md](extensions/strings.md)

The strings extension provides advanced string manipulation functions that go beyond the basic string operations in the standard library.

**Key Features**:
- Character access (`charAt`)
- String search (`indexOf`, `lastIndexOf`)
- String extraction (`substring`)
- String quoting and escaping (`strings.quote`)
- String trimming (`trim`)
- String joining and splitting (`join`, `split`)
- Case conversion (`lowerAscii`, `upperAscii`)
- String replacement (`replace`)
- String formatting (`format`)
- String reversal (`reverse`)

**Example Usage**:
```cel
"Hello, World!".charAt(0)                    // "H"
"hello world".indexOf("world")               // 6
["a", "b", "c"].join(", ")                   // "a, b, c"
"Hello, %s!".format(["World"])               // "Hello, World!"
```

### 2.2 Math Extension

**Documentation**: [math.md](extensions/math.md)

The math extension provides mathematical functions and operations beyond basic arithmetic.

**Key Features**:
- Min/max operations (`math.min`, `math.max`)
- Absolute value (`math.abs`)
- Sign function (`math.sign`)
- Rounding functions (`math.ceil`, `math.floor`, `math.round`, `math.trunc`)
- Bitwise operations (`math.bitAnd`, `math.bitOr`, `math.bitXor`, etc.)
- Floating point helpers (`math.isInf`, `math.isNaN`, `math.isFinite`)
- Square root (`math.sqrt`)

**Example Usage**:
```cel
math.min([1, 2, 3])                          // 1
math.abs(-5)                                 // 5
math.ceil(3.14)                              // 4.0
math.bitAnd(12, 10)                          // 8
```

### 2.3 Lists Extension

**Documentation**: [lists.md](extensions/lists.md)

The lists extension provides advanced list processing functions.

**Key Features**:
- List slicing (`slice`)
- List flattening (`flatten`)
- List deduplication (`distinct`)
- List reversal (`reverse`)
- List sorting (`sort`, `sortBy`)
- Number ranges (`lists.range`)

**Example Usage**:
```cel
[1, 2, 2, 3, 1].distinct()                   // [1, 2, 3]
[[1, 2], [3, 4]].flatten()                   // [1, 2, 3, 4]
[3, 1, 4, 1, 5].sort()                       // [1, 1, 3, 4, 5]
lists.range(5)                               // [0, 1, 2, 3, 4]
```

### 2.4 Sets Extension

**Documentation**: [sets.md](extensions/sets.md)

The sets extension provides set operations on lists, treating them as mathematical sets.

**Key Features**:
- Set containment (`sets.contains`)
- Set equivalence (`sets.equivalent`)
- Set intersection (`sets.intersects`)

**Example Usage**:
```cel
sets.contains([1, 2, 3, 4], [2, 3])          // true
sets.equivalent([1, 2, 3], [3, 2, 1])        // true
sets.intersects([1, 2, 3], [3, 4, 5])        // true
```

### 2.5 Regular Expression Extension

**Documentation**: [regex.md](extensions/regex.md)

The regex extension provides pattern matching capabilities using regular expressions. It supports both the standard cross-platform regex functions and optional types for safe pattern matching.

**Key Features**:
- Pattern matching (`matches`)
- Pattern finding (`find`, `findAll`)
- String splitting (`split`)
- Optional type support for safe operations

**Example Usage**:
```cel
"hello@example.com".matches(r"[\w]+@[\w.]+")  // true
"test123".find(r"\d+")                        // optional("123")
"a,b,c".split(",")                           // ["a", "b", "c"]
```

### 2.6 C++ Regular Expression Extension (re)

**Documentation**: [re.md](extensions/re.md)

The `re` extension provides C++ specific regular expression functions built on the RE2 library. This extension offers additional functionality for string pattern matching, extraction, and capture operations.

**Key Features**:
- Pattern extraction and rewriting (`re.extract`)
- Group capturing (`re.capture`, `re.captureN`)
- Direct error handling
- RE2 library performance and safety

**Example Usage**:
```cel
re.extract("Hello World", r"(\w+) (\w+)", r"$2, $1")  // "World, Hello"
re.capture("user@example.com", r"(\w+)@[\w.]+")       // "user"
re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")  // {"first": "John", "last": "Doe"}
```

### 2.7 Encoders Extension

**Documentation**: [encoders.md](extensions/encoders.md)

The encoders extension provides encoding and decoding functions for common formats.

**Key Features**:
- Base64 encoding/decoding (`base64.encode`, `base64.decode`)

**Example Usage**:
```cel
base64.encode(b"hello")                      // "aGVsbG8="
base64.decode("aGVsbG8=")                    // b"hello"
```

### 2.8 Bindings Extension

**Documentation**: [bindings.md](extensions/bindings.md)

The bindings extension provides the `cel.bind()` macro for creating local variable bindings within CEL expressions. This enables more readable and efficient expressions by avoiding repeated computations.

**Key Features**:
- Local variable binding (`cel.bind`)
- Nested bindings support
- Expression optimization
- Improved readability

**Example Usage**:
```cel
cel.bind(x, "hello", x + " world")           // "hello world"
cel.bind(users, getUsers(), users.size())    // Number of users
```

### 2.9 Protocol Buffers Extension

**Documentation**: [proto.md](extensions/proto.md)

The protobuf extension provides enhanced support for working with Protocol Buffer messages.

**Key Features**:
- Enhanced message construction with macros
- Extension field access (`proto.getExt`, `proto.hasExt`)
- Type-safe operations

**Example Usage**:
```cel
proto.getExt(msg, google.expr.proto2.test.int32_ext)
proto.hasExt(msg, google.expr.proto2.test.int32_ext)
```

## 3. Enabling Extensions

Extensions are enabled when building the CEL environment using the `EnvBuilder`:

```rust
use cel_cxx::*;

let env = Env::builder()
    .with_ext_strings(true)
    .with_ext_math(true)
    .with_ext_lists(true)
    .with_ext_sets(true)
    .with_ext_regex(true)
    .with_ext_re(true)
    .with_ext_encoders(true)
    .with_ext_bindings(true)
    .with_ext_proto(true)
    .build()?;
```

You can enable only the extensions you need:

```rust
// Enable only strings and math extensions
let env = Env::builder()
    .with_ext_strings(true)
    .with_ext_math(true)
    .build()?;
```

## 4. Extension Compatibility

- **All extensions are compatible** with each other and can be used together
- **Extensions are optional** - you only pay for what you use
- **Extensions maintain CEL's safety guarantees** - no memory leaks, no crashes
- **Extensions are deterministic** - same input always produces same output
- **Extensions are side-effect free** - they don't modify global state

For detailed documentation on each extension, see the individual files in the [extensions/](extensions/) directory. 