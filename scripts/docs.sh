#!/bin/bash

# Script to generate documentation locally, simulating docs.rs environment
# This skips C++ compilation for faster documentation builds

set -e

echo "🚀 Generating documentation with docs-only feature..."

# Set environment variable to simulate docs.rs
export DOCS_RS=1

# Generate documentation with all features except the ones requiring C++ compilation
cargo doc \
    --features "derive,async,tokio,async-std,docs-only" \
    --no-deps \
    --document-private-items \
    --open

echo "✅ Documentation generated successfully!"
echo "📖 Documentation available at: target/doc/cel_cxx/index.html" 