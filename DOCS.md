# Documentation Generation

This document explains how to generate documentation for `cel-cxx` without compiling the underlying C++ code.

## Problem

The `cel-cxx` crate depends on Google's `cel-cpp` library, which requires:
- A C++17 compatible compiler
- Bazel build system
- Various system dependencies
- Significant compilation time (10+ minutes)

This makes documentation generation challenging in environments like docs.rs or CI/CD pipelines.

## Solution

We've implemented a `docs-only` feature that skips C++ compilation during documentation builds.

### How it works

1. **Feature flag**: The `docs-only` feature is defined in `cel-cxx-ffi/Cargo.toml`
2. **Build script detection**: The `build.rs` script detects documentation build environments
3. **Dummy artifacts**: Instead of compiling C++, it creates minimal build artifacts
4. **Environment detection**: Multiple methods are used to detect docs builds:
   - `docs-only` feature flag
   - `DOCS_RS` environment variable
   - `RUSTDOC_RUNNING` environment variable
   - Cargo profile analysis

### Usage

#### Local documentation generation

```bash
# Generate docs without C++ compilation
cargo doc --features docs-only --no-deps --open

# Or use the provided script
./scripts/docs.sh
```

#### docs.rs configuration

The `Cargo.toml` includes configuration for docs.rs:

```toml
[package.metadata.docs.rs]
features = ["derive", "async", "tokio", "async-std", "docs-only"]
rustdoc-args = ["--cfg", "docsrs"]
targets = ["x86_64-unknown-linux-gnu"]
```

#### CI/CD integration

Set the `DOCS_RS` environment variable to enable docs-only mode:

```bash
export DOCS_RS=1
cargo doc --features "derive,async,tokio,async-std,docs-only"
```

### Features included in documentation

When using `docs-only` mode, the following features are available:

- ✅ `derive` - Derive macros for custom types
- ✅ `async` - Async/await support
- ✅ `tokio` - Tokio runtime integration  
- ✅ `async-std` - async-std runtime integration
- ❌ Full C++ compilation (skipped)

### Limitations

- The generated documentation reflects the Rust API only
- C++ bridge code is not actually compiled
- Some advanced examples may not be fully testable
- Link-time optimizations are not performed

### Implementation details

The `docs-only` feature works by modifying the build process in `cel-cxx-ffi/build.rs`:

```rust
fn is_docs_build() -> bool {
    // Check for docs-only feature
    if cfg!(feature = "docs-only") {
        return true;
    }
    
    // Check for docs.rs environment
    if std::env::var("DOCS_RS").is_ok() {
        return true;
    }
    
    // Additional detection methods...
}
```

When a docs build is detected, the build script:
1. Prints a warning message
2. Creates dummy build artifacts
3. Skips all C++ compilation
4. Returns successfully

This allows the documentation to be generated quickly and reliably across different environments. 