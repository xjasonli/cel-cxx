#!/bin/bash

# Test script to verify docs-only feature works in different environments

set -e

echo "🧪 Testing docs-only feature in different environments..."

# Test 1: Using docs-only feature flag
echo "📝 Test 1: Using docs-only feature flag"
cargo doc --features "docs-only" --no-deps -q
if [ $? -eq 0 ]; then
    echo "✅ docs-only feature flag works"
else
    echo "❌ docs-only feature flag failed"
    exit 1
fi

# Test 2: Using DOCS_RS environment variable
echo "📝 Test 2: Using DOCS_RS environment variable"
export DOCS_RS=1
cargo doc --no-deps -q
if [ $? -eq 0 ]; then
    echo "✅ DOCS_RS environment variable works"
else
    echo "❌ DOCS_RS environment variable failed"
    exit 1
fi
unset DOCS_RS

# Test 3: Using RUSTDOC_RUNNING environment variable
echo "📝 Test 3: Using RUSTDOC_RUNNING environment variable"
export RUSTDOC_RUNNING=1
cargo doc --no-deps -q
if [ $? -eq 0 ]; then
    echo "✅ RUSTDOC_RUNNING environment variable works"
else
    echo "❌ RUSTDOC_RUNNING environment variable failed"
    exit 1
fi
unset RUSTDOC_RUNNING

# Test 4: Full feature set with docs-only
echo "📝 Test 4: Full feature set with docs-only"
cargo doc --features "derive,async,tokio,async-std,docs-only" --no-deps -q
if [ $? -eq 0 ]; then
    echo "✅ Full feature set with docs-only works"
else
    echo "❌ Full feature set with docs-only failed"
    exit 1
fi

# Test 5: Check that warning message appears
echo "📝 Test 5: Check warning message appears"
OUTPUT=$(cargo doc --features "docs-only" --no-deps 2>&1)
if echo "$OUTPUT" | grep -q "Skipping C++ compilation for documentation build"; then
    echo "✅ Warning message appears correctly"
else
    echo "❌ Warning message not found"
    echo "Output: $OUTPUT"
    exit 1
fi

echo ""
echo "🎉 All docs-only tests passed!"
echo "📖 Documentation is available at: target/doc/cel_cxx/index.html"
echo ""
echo "💡 Usage examples:"
echo "   Local docs:     cargo doc --features docs-only --no-deps --open"
echo "   CI/CD:          DOCS_RS=1 cargo doc --no-deps"
echo "   docs.rs:        Automatic (configured in Cargo.toml)" 