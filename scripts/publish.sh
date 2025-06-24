#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Publishing cel-cxx crates to crates.io${NC}"
echo

# Check if we're on the right branch and everything is committed
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}❌ Working directory is not clean. Please commit all changes first.${NC}"
    exit 1
fi

echo -e "${YELLOW}📋 Pre-publish checks...${NC}"

# Run tests
echo "Running tests..."
cargo test --workspace --all-features

# Check formatting
echo "Checking code formatting..."
cargo fmt --all -- --check

# Run clippy
echo "Running clippy..."
cargo clippy --workspace --all-features -- -D warnings

# Check documentation
echo "Checking documentation..."
cargo doc --workspace --all-features --no-deps

echo -e "${GREEN}✅ All pre-publish checks passed!${NC}"
echo

# Define publish order (dependencies first)
CRATES=(
    "crates/cel-build-utils"
    "crates/cel-cxx-macros" 
    "crates/cel-cxx-ffi"
    "."
)

CRATE_NAMES=(
    "cel-build-utils"
    "cel-cxx-macros"
    "cel-cxx-ffi"
    "cel-cxx"
)

echo -e "${YELLOW}📦 Publishing crates in dependency order...${NC}"

for i in "${!CRATES[@]}"; do
    crate_path="${CRATES[$i]}"
    crate_name="${CRATE_NAMES[$i]}"
    
    echo
    echo -e "${YELLOW}Publishing ${crate_name}...${NC}"
    
    # Dry run first
    echo "Performing dry run..."
    (cd "$crate_path" && cargo publish --dry-run)
    
    # Ask for confirmation
    echo -e "${YELLOW}Ready to publish ${crate_name}. Continue? (y/N)${NC}"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "Publishing ${crate_name}..."
        (cd "$crate_path" && cargo publish)
        echo -e "${GREEN}✅ ${crate_name} published successfully!${NC}"
        
        # Wait a bit for the crate to be available
        if [ "$i" -lt $((${#CRATES[@]} - 1)) ]; then
            echo "Waiting 30 seconds for crate to be available..."
            sleep 30
        fi
    else
        echo -e "${RED}❌ Skipping ${crate_name}${NC}"
        exit 1
    fi
done

echo
echo -e "${GREEN}🎉 All crates published successfully!${NC}"
echo
echo "You can now:"
echo "1. Create a git tag: git tag v0.1.0 && git push origin v0.1.0"
echo "2. Create a GitHub release"
echo "3. Update documentation links" 