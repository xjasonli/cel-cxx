# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-09-21

Migrated to the cel-cpp `feat/runtime-function-overload-resolution` branch
(pinned at `c84e883d`), replacing the `runtime-function-with-overload-id` hack
with ID-based overload lookup and full-type descriptors.

### Changed
- **cel-cpp pin** bumped to `feat/runtime-function-overload-resolution`
  (`c84e883df5932dcc992dd3494a771c37e85ae1fe`); Windows `fix_msvc.patch` trimmed
  to `common/values/optional_value.cc` (nine hunks).
- **Native build dependencies** now require/bootstrap Bazel `8.7.0` and request
  abseil-cpp `20260526.0`, matching the rebased cel-cpp dependency graph.
- **`function::FunctionOverloads`** flattened from a two-layer kind-grouped
  structure to a flat list of `OverloadEntry<T>`. `FunctionKindOverload` and
  `argument_kinds()` are removed; `entries()` now yields `OverloadEntry<T>` and
  deduplication is by `(member, full argument types)` so same-kind overloads
  (e.g. `list<int>` and `list<string>`) coexist. `find`/`find_mut` accept
  `&[ValueType]` instead of `&[Kind]`; `remove` accepts `&[ValueType]` instead of
  an iterator of `Kind`.
- **Runtime registration** registers one type-based+id `FunctionDescriptor`
  per overload (`FunctionDescriptor::new_with_overload_id`) instead of one
  kind-based entry per kind group; overload selection happens in cel-cpp by
  descriptor `overload_id` + full `cel::Type`.
- **`FfiFunction::invoke`** (cel-cxx-ffi) no longer takes an `overload_id`
  span; the cel-cpp `Function::Invoke` override is the 2-arg `InvokeContext`
  form. `FfiFunctionImpl` is single-impl (no id dispatch). The obsolete
  `Span<CxxString>` bridge and its size helper are removed.
- **`FfiActivationImpl::new`** (internal cel-cxx activation) is now fallible and takes
  `arena` + `descriptor_pool`; bindings without a compile-time declaration matching
  their name, full argument types, and call style are rejected at eval time.
- **`AnyFfiFunctionPredicate`/`FfiFunctionPredicate`** (cel-cxx-ffi) second
  parameter is now `&OverloadDecl` (matching the new cel-cpp `FunctionPredicate`),
  with `OverloadDecl` accessors (`id`, `member`, `result`, `args`) exposed.
- **Extension registration** (cel-cxx-ffi) exposes the version/options args:
  `math`/`lists` `register_functions` and `compiler_library` take `version: i32`
  (+ `latest_version()`); `strings` `register_functions`/`compiler_library` take
  `&StringsExtensionOptions` (mapped like `RuntimeOptions`: `new()` +
  `version`/`max_precision` getter/`_mut`).
- `RuntimeOptions::enable_type_level_overload` getter/setter exposed at the FFI
  layer only. Statically typed same-kind calls resolve using checker IDs;
  dynamic same-kind arguments still depend on runtime type matching.
- Remove the cel-cxx-ffi optional patches (`optional.unwrap`/`unwrapOpt` checker
  declarations and `orValue` runtime registration) and the local Sets checker
  implementation; use the upstream libraries directly.
- `RuntimeBuilder::enable_optional` (cel-cxx-ffi) no longer takes the unused
  `RuntimeOptions` argument.
- Async evaluation requires registered functions, bindings, and providers to own
  their captures (`'static`); synchronous evaluation still accepts borrowed captures.
- Opaque FFI construction and type conversion return errors for unresolved
  message/enum type names; well-known protobuf types use CEL's native mappings.
- Evaluation contexts share the thread-safe protobuf message factory.
- Refresh all examples with checked results, boundary cases, accurate comments,
  and plain-text output; document commands and feature requirements in an index.

### Fixed
- Export the UTF-8 headers required by protobuf `35.1` in native build artifacts.
- Adapt Abseil status error constructors to the new defaulted source-location
  parameter while preserving their Rust API.
- Preserve arena lifetimes through type-based function descriptors, registries,
  and activation references; declaration/type getter references borrow their owner.
- Reject zero-argument member function bindings before overload-ID generation.
- Construct async activation descriptors inside the evaluation worker so they
  remain within their arena's lifetime.
- Spawn async workers through a safe API so forgetting an evaluation future
  cannot leave dangling borrowed captures.
- Propagate rejected environment declarations/registrations and list/map/optional
  conversion failures instead of silently dropping them.
- Compare full binding types even when their display-based overload IDs collide.
- Preserve callable error codes and return an error for asynchronous callables
  supplied through a custom synchronous activation.
- Release protobuf arena messages before their factory and descriptors.
- Preserve nanosecond precision in duration/timestamp conversion, including
  negative fractional durations and timestamps before the Unix epoch.
- Return `None` when a native opaque value is downcast to a Rust opaque wrapper.
- Correct the FFI size of `TypeParamType` and `TypeParameters` to prevent native
  writes past Rust storage; check the corresponding native sizes and alignments
  at compile time.
- Preserve global versus member call syntax when converting macro ASTs to C++.
- Enable optional support in examples that use optional operations and preserve
  empty protobuf strings when encoding repeated elements and oneof values.
- Resolve named IANA timezones on Windows through the OS-managed timezone
  database via the `windows_chrono_timezone.patch` cel-cpp patch: Windows ships
  no TZif tree for cctz, so named zones previously failed with `InvalidArgument`
  (numeric offsets and UTC were unaffected). `TZDIR` still takes precedence, an
  unknown zone name or unavailable database still errors instead of degrading
  to UTC, and results follow the host's database version.
- Rebuild `libcel` when the Bazel workspace inputs change (MODULE.bazel,
  BUILD.bazel, `.bazelrc`, `.bzl`, and patch files) by tracking them with
  `cargo:rerun-if-changed`, instead of silently linking a stale library.
- Build Android targets from Windows hosts (`cargo ndk`): normalize the NDK
  path passed to `rules_android_ndk` (its symlink setup strips a backslash
  `ANDROID_NDK_HOME` prefix with a forward-slash `replace` and then writes
  outside the repository), neutralize the Android SDK repository environment
  for every target (libcel compiles C++ through the NDK toolchain only and
  hosts may only have an unparseable `android-*.0` platform), fall back to the
  flat ANTLR grammar output when cross-compiling from Windows
  (`antlr_cross_host_output.patch`: the copy actions select on the target OS
  while the tool's layout follows the executing host), and enable the
  `archive_param_file` feature for the NDK archiver
  (`rules_android_ndk_archive_param_file.patch` through an `archive_override`)
  so archiving ~600 objects uses an `@response` file instead of exceeding
  CreateProcessW's 32767-character command line.

### Added
- `function::OverloadEntry<T>`.
- `Clone` for function/variable binding collections used by async evaluation snapshots.
- Examples for per-evaluation function bindings, time boundaries, and async
  cancellation with resource cleanup.

### Removed
- **`async-std` runtime support**: the `async-std` feature, the `AsyncStd` runtime
  type, `use_async_std` methods on `Env`/`EnvBuilder`/`Program`, the `async-std`
  example, and its tests. The [async-std](https://github.com/async-rs/async-std)
  project has been unmaintained for over a year and its ecosystem recommends
  [smol](https://github.com/smol-rs/smol), which cel-cxx already supports via the
  `smol` feature. Migrate by switching `use_async_std()` to `use_smol()`.

### Tests
- Cover same-kind global/member overloads, binding overrides and sibling retention,
  declaration-only bindings, undeclared signatures/names, invalid member bindings,
  and async overload bindings.
- Add compile-fail coverage for descriptor arena lifetimes and declaration getter
  owner borrows.
- Cover scalar extremes, Unicode/NUL and bytes, nested collections/optionals and
  types, nanosecond arithmetic, malformed protobuf data, descriptor isolation,
  all non-OK callable status codes, and registration/binding boundaries.
- Verify cancellation, resource release, concurrent binding isolation, and owned
  nested values across Tokio and smol with channel synchronization.
- Replace collection print-only checks and optional type-tag checks with exact
  result assertions.
- Cover macro-generated global/member calls and copied callback expressions.
- Cover named-zone timestamp accessors end-to-end through CEL evaluation: DST
  and standard-time offsets chosen for the instant, spring-forward and
  fall-back transition boundaries, pre-epoch and post-2038 instants, IANA
  links, and a date boundary; assert unknown zones fail with `InvalidArgument`
  on every host, and verify `TZDIR` precedence over the Windows OS database
  with a crafted constant-UTC TZif fixture.

## [0.2.5] - 2026-07-01

### Added
- **Native protobuf support** for binding and evaluating protobuf messages through CEL-CPP type adapters
  - Descriptor pool loading from serialized `FileDescriptorSet` data
  - Dynamic `StructValue` values carrying protobuf type names and serialized bytes
  - Protobuf variable declaration, dynamic variable binding, struct field access, enum constant registration, and per-thread dynamic message factories
- **Protobuf derive macros** for generated Rust protobuf types
  - Added `ProstValue`, `ProtobufLegacyValue`, and `ProtobufValue` derive support behind the `prost`, `protobuf-legacy`, and `protobuf` feature flags
  - Added build integration plus examples for prost and protobuf-legacy backends
- **Protobuf examples, tests, and documentation** covering basic and advanced evaluation, nested messages, repeated fields, maps, enums, well-known types, and multi-threaded evaluation
- **Build override support** via `CEL_PREBUILT_DIR` to use prebuilt CEL artifacts and skip the Bazel build

### Changed
- Made `StructValue` opaque and exposed constructor/accessor methods (`from_bytes`, `type_name`, `to_bytes`) instead of public fields
- Renamed protobuf-specific APIs to generic dynamic struct APIs:
  - `declare_protobuf_variable` → `declare_variable_with_type`
  - `bind_protobuf_variable` → `bind_variable_dynamic`
  - `get_protobuf_field` / `has_protobuf_field` → `get_struct_field` / `has_struct_field`
- Cleaned up unused CXX bridge type aliases and suppressed upstream nullability completeness warnings

### Fixed
- Fixed undefined behavior risks in `Span`/`MutSpan` for empty vectors and safer size hints
- Fixed unchecked indexing in `TypeParameters`
- Fixed async evaluation null-result handling to match synchronous evaluation
- Replaced `Unknown` conversion `todo!()` panics with proper error values
- Fixed 12 doctests and refreshed macro documentation/examples

## [0.2.4] - 2025-12-17

### Added
- **Macro System**: Compile-time expression expansion support
  - Global macros for function-like invocations
  - Receiver macros for method-like invocations
  - Variable-argument macro support
  - MacroExprFactory for expression building during expansion
  - `EnvBuilder::register_macro()` for macro registration

### Changed
- Enhanced expression manipulation capabilities with macro support

## [0.2.3] - 2025-11-28

### Changed
- Update dependencies version
  - cxx 1.0.189
  - cel-cpp 0.14.0

## [0.2.2] - 2025-11-26

### Added
- **Comprehensions Extension**: Support for list comprehensions with filtering
- **Documentation System**: Restructured documentation with separate English (`docs/`) and Chinese (`docs-cn/`) directories
- **CEL Reference Guide**: Complete reference documentation for CEL language features and extensions

### Changed
- **Member Function Constraints**: Enforced non-empty arguments requirement for member functions at compile-time and runtime
- **cel-cpp Update**: Updated to latest cel-cpp with improved AST implementation (AstImpl renamed to Ast)

## [0.2.1] - 2025-08-14

### Added
- **Enhanced Opaque Derive Macro**
  - Added `display` attribute support for automatic `std::fmt::Display` implementation
  - Support for both automatic Debug-based formatting and custom format expressions
  - Improved macro documentation with comprehensive usage examples
  - Better developer experience for implementing Display trait on opaque types

## [0.2.0] - 2025-07-23

### Added
- **Complete CEL Standard Library Implementation**
  - Full support for all CEL standard library functions with comprehensive test coverage
  - String operations, list/map manipulations, type conversions, and utility functions
  - Mathematical operations, comparison functions, and type checking utilities

- **Optional Types Support**
  - Native `Optional<T>` wrapper type for safe null handling
  - Safe navigation operators and optional value processing
  - Integration with all CEL extensions and standard library functions

- **Comprehensive CEL Extensions Support**
  - **Strings Extension**: Advanced string manipulation (join, split, quote, etc.)
  - **Math Extension**: Mathematical functions (abs, ceil, floor, bitwise operations, etc.)
  - **Lists Extension**: List processing utilities (flatten, reverse, slice, etc.)
  - **Sets Extension**: Set operations (contains, equivalent, intersects)
  - **Encoders Extension**: Base64 encoding/decoding operations
  - **Regex Extension**: Pattern extraction (`regex.extract`, `regex.extractAll`, `regex.replace`)
  - **RE Extension**: C++ specific regex functions (`re.extract`, `re.capture`, `re.captureN`)
  - **Bindings Extension**: Variable binding macros (`cel.bind`)

- **Enhanced Documentation System**
  - Complete English documentation for all CEL features and extensions
  - Comprehensive Chinese (Simplified) documentation translation
  - Detailed API references with practical examples and usage patterns
  - RE2 regex syntax documentation links for all regex-related functions

- **Improved Runtime Features**
  - Automatic absl logging initialization to prevent runtime warnings
  - Enhanced extension configuration and registration system
  - Modular extension loading with dependency checking

### Changed
- **Enhanced Build System**
  - Updated build configuration for new CEL-CPP dependencies
  - Improved Bazel integration with MODULE.bazel updates
  - Better platform compatibility including MSVC support fixes

- **API Improvements**
  - Enhanced `EnvBuilder` with `with_ext_*` methods for all extensions
  - Improved type conversion handling between Rust and CEL types
  - Better error handling and debugging capabilities

### Fixed
- **CEL-CPP Implementation Issues**
  - Applied patches to fix missing functionality in upstream CEL-CPP
  - Resolved optional type registration issues
  - Fixed regex extension compatibility problems

- **Documentation Corrections**
  - Fixed incorrect function signatures and behavior descriptions
  - Corrected regex capture group syntax (`\1`, `\2` instead of `$1`, `$2`)
  - Updated extension documentation to match actual implementation

- **Runtime Stability**
  - Fixed clippy warnings and code quality issues
  - Improved memory management and resource handling
  - Enhanced error propagation and handling

### Technical Details
- Added `optional.h/optional.cc` for Optional<T> type support
- Extended `extensions.h/extensions.cc` with all standard CEL extensions
- Comprehensive test suites for all new functionality (`tests/standard-library.rs`, `tests/optional.rs`, `tests/extensions-*.rs`)
- Practical examples demonstrating real-world usage scenarios (`examples/standard-library.rs`, `examples/extensions-*.rs`)

## [0.1.5] - 2025-07-11

### Added
- **Windows MSVC support** for `x86_64-pc-windows-msvc` target
  - Full compilation and runtime support on Windows with MSVC(2022) toolchain
  - Windows-specific build configuration and dependency management

## [0.1.4] - 2025-07-09

### Added
- **Cross-compilation support** via [cross-rs](https://github.com/cross-rs/cross)
  - Automatic toolchain detection and configuration for cross-compilation environments
  - Support for select targets including ARM Linux, MIPS, PowerPC, and RISC-V
  - Seamless integration with cross-rs containers and toolchains
- **Consolidated build system** with unified configuration
  - Merged platform-specific build directories into `cel/` and `cel-windows/`
  - Simplified build configuration and reduced duplication
- **Separate async examples crate** (`examples-async/`)
  - Extracted async examples into dedicated crate for better organization
  - Improved documentation and examples for async usage patterns

### Changed
- **Refactored build system architecture**
  - Consolidated `build-{android,apple,linux,windows}` directories into unified structure
  - Improved build configuration management and platform detection
- **Enhanced Value struct** with additional convenience methods
- **Improved async examples organization** with dedicated crate and documentation

### Fixed
- **Fixed test compilation errors** when async features are disabled
  - Added missing `#[cfg(feature = "async")]` guards to prevent compilation errors
  - Improved conditional compilation for async-related test code

## [0.1.3] - 2025-06-29

### Added
- **Android platform support** with NDK integration
  - ARM64 (`aarch64-linux-android`)
  - ARMv7 (`armv7-linux-androideabi`) 
  - x86_64 (`x86_64-linux-android`)
  - x86 (`i686-linux-android`)
- **iOS platform support** for device and simulator
  - iOS device (`aarch64-apple-ios`)
  - iOS Simulator ARM64 (`aarch64-apple-ios-sim`)
  - iOS Simulator x86_64 (`x86_64-apple-ios`)

### Changed
- **Platform-separated build architecture** to isolate platform-specific dependencies
  - `build-linux/` - Pure Linux builds without Android/Apple dependencies
  - `build-apple/` - macOS and iOS builds with Apple toolchain
  - `build-android/` - Android builds with NDK integration
  - `build-windows/` - Windows builds (placeholder)
- **Improved target detection** and automatic build directory selection
- **Enhanced platform-specific configurations** in `.bazelrc` files

## [0.1.2] - 2025-06-28

### Added
- Added support for smol async runtime

## [0.1.1] - 2025-06-28

### Fixed
- Fixed macOS compatibility issues

## [0.1.0] - 2025-06-24

### Added

#### Core Features
- **Zero-annotation function system** using Generic Associated Types (GATs)
- **Dual function architecture** separating compile-time and runtime concerns
- **Smart reference handling** with automatic lifetime management
- **Type-safe value conversions** between Rust and CEL types
- **Function overloading** with automatic signature resolution
- **Custom opaque types** via `#[derive(Opaque)]` macro

#### Type System
- Full support for CEL primitive types (`null`, `bool`, `int`, `uint`, `double`, `string`, `bytes`)
- Collection types (`list<T>`, `map<K,V>`) with comprehensive Rust integration
- Time types (`duration`, `timestamp`) with chrono integration
- Optional types (`optional<T>`) with safe navigation
- Automatic conversions for standard Rust collections (`Vec`, `HashMap`, `BTreeMap`, etc.)

#### Async Support
- **Innovative dual-threading architecture** for async function integration
- Support for Tokio and async-std runtimes
- Async-to-blocking and blocking-to-async bridges
- Seamless integration of async Rust code with synchronous CEL-CPP evaluation

#### CEL Language Features
- All standard CEL operators (arithmetic, logical, comparison, membership)
- Built-in functions and custom function registration
- Variable binding and scoping
- Conditional expressions with short-circuiting
- List and map comprehensions with filtering
- CEL language extensions and custom operators
- CEL macro expansion support

#### Developer Experience
- Comprehensive documentation with examples
- Builder pattern APIs for environment construction
- Compile-time type checking and validation
- Detailed error messages and debugging support
- Zero-cost FFI with direct C++ integration

#### Crates Structure
- **cel-cxx**: Main crate with high-level APIs
- **cel-cxx-ffi**: Low-level FFI bindings to CEL-CPP
- **cel-cxx-macros**: Procedural macros for custom types
- **cel-build-utils**: Build utilities for CEL-CPP integration

#### Platform Support
- Linux: Full support and testing
- macOS: Untested but expected to work
- Windows: Not supported due to CEL-CPP build limitations

### Technical Implementation
- Built on top of [google/cel-cpp](https://github.com/google/cel-cpp)
- Safe interop via [cxx](https://github.com/dtolnay/cxx) crate
- Bazel integration for CEL-CPP compilation
- Comprehensive test suite and examples
- CI/CD pipeline for automated testing

[Unreleased]: https://github.com/xjasonli/cel-cxx/compare/v0.2.5...HEAD
[0.2.5]: https://github.com/xjasonli/cel-cxx/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/xjasonli/cel-cxx/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/xjasonli/cel-cxx/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/xjasonli/cel-cxx/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/xjasonli/cel-cxx/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/xjasonli/cel-cxx/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/xjasonli/cel-cxx/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/xjasonli/cel-cxx/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/xjasonli/cel-cxx/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/xjasonli/cel-cxx/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/xjasonli/cel-cxx/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/xjasonli/cel-cxx/releases/tag/v0.1.0 