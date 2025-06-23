#!/bin/bash

# Script to generate documentation locally, simulating docs.rs environment
# This skips C++ compilation for faster documentation builds

set -e

echo "🚀 Generating documentation with docs-only feature..."

if ! type cargo-docs-rs > /dev/null 2>&1; then
    echo "🚨 cargo-docs-rs is not installed. Installing it..."
    cargo install cargo-docs-rs
fi

# Generate documentation with all features except the ones requiring C++ compilation
echo "🔍 Running cargo docs-rs with nightly toolchain..."
cargo +nightly docs-rs -p cel-build-utils
cargo +nightly docs-rs -p cel-cxx-macros
cargo +nightly docs-rs -p cel-cxx-ffi
cargo +nightly docs-rs -p cel-cxx

echo "✅ Documentation generated successfully!"
echo "📖 Documentation available at: target/x86_64-unknown-linux-gnu/doc/cel_cxx/index.html" 
