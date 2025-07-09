# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/xjasonli/cel-cxx/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/xjasonli/cel-cxx/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/xjasonli/cel-cxx/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/xjasonli/cel-cxx/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/xjasonli/cel-cxx/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/xjasonli/cel-cxx/releases/tag/v0.1.0 