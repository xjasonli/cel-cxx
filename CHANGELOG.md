# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/xjasonli/cel-cxx/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/xjasonli/cel-cxx/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/xjasonli/cel-cxx/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/xjasonli/cel-cxx/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/xjasonli/cel-cxx/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/xjasonli/cel-cxx/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/xjasonli/cel-cxx/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/xjasonli/cel-cxx/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/xjasonli/cel-cxx/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/xjasonli/cel-cxx/releases/tag/v0.1.0 