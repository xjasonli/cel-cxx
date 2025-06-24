#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
WAIT_TIME=30
PUBLISH_ALL=false
DRY_RUN=false
TARGET_CRATES=()

# Define crates in dependency order
declare -A CRATES=(
    ["cel-build-utils"]="crates/cel-build-utils"
    ["cel-cxx-macros"]="crates/cel-cxx-macros"
    ["cel-cxx-ffi"]="crates/cel-cxx-ffi"
    ["cel-cxx"]="."
)

CRATE_ORDER=("cel-build-utils" "cel-cxx-macros" "cel-cxx-ffi" "cel-cxx")

# Help function
show_help() {
    echo "Usage: $0 [OPTIONS] [CRATE_NAME...]"
    echo
    echo "Publish cel-cxx crates to crates.io"
    echo
    echo "Arguments:"
    echo "  CRATE_NAME...       One or more crate names to publish"
    echo "                      (cel-build-utils, cel-cxx-macros, cel-cxx-ffi, cel-cxx)"
    echo "                      Will be published in dependency order automatically"
    echo
    echo "Options:"
    echo "  --all               Publish all crates in dependency order"
    echo "  --dry-run           Only run checks and dry-run, don't actually publish"
    echo "  --wait SECONDS      Wait time between crate publications (default: 30)"
    echo "  -h, --help          Show this help message"
    echo
    echo "Examples:"
    echo "  $0 cel-cxx                           # Publish only cel-cxx crate"
    echo "  $0 cel-cxx-ffi cel-cxx               # Publish cel-cxx-ffi and cel-cxx in dependency order"
    echo "  $0 cel-cxx cel-build-utils           # Publish in correct order: cel-build-utils then cel-cxx"
    echo "  $0 --all                             # Publish all crates"
    echo "  $0 --all --wait 60                   # Publish all crates with 60s wait"
    echo "  $0 cel-cxx-ffi cel-cxx --wait 45     # Publish specified crates with custom wait"
    echo "  $0 --all --dry-run                   # Check all crates without publishing"
    echo "  $0 cel-cxx --dry-run                 # Check only cel-cxx crate"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --all)
            PUBLISH_ALL=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --wait)
            if [[ -n $2 && $2 =~ ^[0-9]+$ ]]; then
                WAIT_TIME=$2
                shift 2
            else
                echo -e "${RED}❌ --wait requires a numeric argument${NC}"
                exit 1
            fi
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        -*)
            echo -e "${RED}❌ Unknown option: $1${NC}"
            show_help
            exit 1
            ;;
        *)
            TARGET_CRATES+=("$1")
            shift
            ;;
    esac
done

# Validate arguments
if [[ "$PUBLISH_ALL" == true && ${#TARGET_CRATES[@]} -gt 0 ]]; then
    echo -e "${RED}❌ Cannot specify both --all and specific crate names${NC}"
    exit 1
fi

if [[ "$PUBLISH_ALL" == false && ${#TARGET_CRATES[@]} -eq 0 ]]; then
    echo -e "${RED}❌ Must specify either --all or one or more crate names${NC}"
    show_help
    exit 1
fi

# Validate crate names if specified
if [[ ${#TARGET_CRATES[@]} -gt 0 ]]; then
    invalid_crates=()
    for crate in "${TARGET_CRATES[@]}"; do
        if [[ -z "${CRATES[$crate]}" ]]; then
            invalid_crates+=("$crate")
        fi
    done
    
    if [[ ${#invalid_crates[@]} -gt 0 ]]; then
        echo -e "${RED}❌ Unknown crate(s): ${invalid_crates[*]}${NC}"
        echo "Available crates: ${!CRATES[@]}"
        exit 1
    fi
fi

# Function to sort crates by dependency order
sort_crates_by_dependency() {
    local input_crates=("$@")
    local sorted_crates=()
    
    # Go through the dependency order and add crates that are in input
    for crate in "${CRATE_ORDER[@]}"; do
        for input_crate in "${input_crates[@]}"; do
            if [[ "$crate" == "$input_crate" ]]; then
                sorted_crates+=("$crate")
                break
            fi
        done
    done
    
    echo "${sorted_crates[@]}"
}

# Determine which crates to publish
if [[ "$PUBLISH_ALL" == true ]]; then
    CRATES_TO_PUBLISH=("${CRATE_ORDER[@]}")
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${GREEN}🔍 Dry-run: Checking all crates${NC}"
    else
        echo -e "${GREEN}🚀 Publishing all crates to crates.io${NC}"
    fi
else
    # Sort the target crates by dependency order
    readarray -t CRATES_TO_PUBLISH < <(sort_crates_by_dependency "${TARGET_CRATES[@]}")
    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${GREEN}🔍 Dry-run: Checking selected crates${NC}"
    else
        echo -e "${GREEN}🚀 Publishing selected crates to crates.io${NC}"
    fi
    echo -e "${BLUE}Selected crates (in dependency order): ${CRATES_TO_PUBLISH[*]}${NC}"
fi

if [[ "$DRY_RUN" == false ]]; then
    echo -e "${BLUE}Wait time between publications: ${WAIT_TIME}s${NC}"
fi
echo

# Check if we're on the right branch and everything is committed (skip in dry-run mode)
if [[ "$DRY_RUN" == false && -n "$(git status --porcelain)" ]]; then
    echo -e "${RED}❌ Working directory is not clean. Please commit all changes first.${NC}"
    exit 1
fi

# Pre-publish checks function
run_checks() {
    local crate_path=$1
    local crate_name=$2
    
    echo -e "${YELLOW}📋 Running pre-publish checks for ${crate_name}...${NC}"
    
    # Run tests for specific crate
    echo "Running tests..."
    if [[ "$crate_path" == "." ]]; then
        if ! cargo test --all-features; then
            echo -e "${RED}❌ Tests failed for ${crate_name}${NC}"
            return 1
        fi
    else
        if ! (cd "$crate_path" && cargo test --all-features); then
            echo -e "${RED}❌ Tests failed for ${crate_name}${NC}"
            return 1
        fi
    fi
    
    # Check formatting
    echo "Checking code formatting..."
    if [[ "$crate_path" == "." ]]; then
        if ! cargo fmt --all -- --check; then
            echo -e "${RED}❌ Code formatting check failed for ${crate_name}${NC}"
            echo -e "${YELLOW}💡 Run 'cargo fmt --all' to fix formatting issues${NC}"
            return 1
        fi
    else
        if ! (cd "$crate_path" && cargo fmt -- --check); then
            echo -e "${RED}❌ Code formatting check failed for ${crate_name}${NC}"
            echo -e "${YELLOW}💡 Run 'cd ${crate_path} && cargo fmt' to fix formatting issues${NC}"
            return 1
        fi
    fi
    
    # Run clippy
    echo "Running clippy..."
    if [[ "$crate_path" == "." ]]; then
        if ! cargo clippy --all-features -- -D warnings; then
            echo -e "${RED}❌ Clippy check failed for ${crate_name}${NC}"
            return 1
        fi
    else
        if ! (cd "$crate_path" && cargo clippy --all-features -- -D warnings); then
            echo -e "${RED}❌ Clippy check failed for ${crate_name}${NC}"
            return 1
        fi
    fi
    
    # Check documentation
    echo "Checking documentation..."
    if [[ "$crate_path" == "." ]]; then
        if ! cargo +nightly docs-rs; then
            echo -e "${RED}❌ Documentation check failed for ${crate_name}${NC}"
            return 1
        fi
    else
        if ! (cd "$crate_path" && cargo +nightly docs-rs); then
            echo -e "${RED}❌ Documentation check failed for ${crate_name}${NC}"
            return 1
        fi
    fi
    
    echo -e "${GREEN}✅ Pre-publish checks passed for ${crate_name}!${NC}"
    return 0
}

# Run all pre-publish checks for all crates
run_all_checks() {
    echo -e "${YELLOW}🔍 Running pre-publish checks for all crates...${NC}"
    
    for crate_name in "${CRATES_TO_PUBLISH[@]}"; do
        local crate_path="${CRATES[$crate_name]}"
        echo
        if ! run_checks "$crate_path" "$crate_name"; then
            echo -e "${RED}❌ Pre-publish checks failed for ${crate_name}${NC}"
            echo -e "${RED}❌ Aborting publish process${NC}"
            exit 1
        fi
    done
    
    echo
    echo -e "${GREEN}✅ All pre-publish checks passed!${NC}"
    
    # Run dry-run for all crates
    echo
    echo -e "${YELLOW}🔍 Running cargo publish --dry-run for all crates...${NC}"
    for crate_name in "${CRATES_TO_PUBLISH[@]}"; do
        local crate_path="${CRATES[$crate_name]}"
        echo
        echo -e "${YELLOW}Performing dry run for ${crate_name}...${NC}"
        if ! (cd "$crate_path" && cargo publish --dry-run); then
            echo -e "${RED}❌ Dry-run failed for ${crate_name}${NC}"
            echo -e "${RED}❌ Aborting publish process${NC}"
            exit 1
        fi
        echo -e "${GREEN}✅ ${crate_name} dry-run completed successfully!${NC}"
    done
    
    echo
    echo -e "${GREEN}✅ All dry-run checks passed!${NC}"
}

# Publish function (simplified, only handles actual publishing)
publish_crate() {
    local crate_name=$1
    local crate_path="${CRATES[$crate_name]}"
    local is_last=$2
    
    echo
    echo -e "${YELLOW}📦 Publishing ${crate_name}...${NC}"
    
    # Ask for confirmation
    echo -e "${YELLOW}Ready to publish ${crate_name}. Continue? (y/N)${NC}"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        echo "Publishing ${crate_name}..."
        if ! (cd "$crate_path" && cargo publish); then
            echo -e "${RED}❌ Failed to publish ${crate_name}${NC}"
            exit 1
        fi
        echo -e "${GREEN}✅ ${crate_name} published successfully!${NC}"
        
        # Wait if not the last crate
        if [[ "$is_last" != true ]]; then
            echo "Waiting ${WAIT_TIME} seconds for crate to be available..."
            sleep "$WAIT_TIME"
        fi
    else
        echo -e "${RED}❌ Skipping ${crate_name}${NC}"
        exit 1
    fi
}

# Main publishing logic
if [[ "$DRY_RUN" == true ]]; then
    echo -e "${YELLOW}🔍 Checking crates in dependency order...${NC}"
    # Run all checks for dry-run mode
    run_all_checks
else
    echo -e "${YELLOW}📦 Publishing crates in dependency order...${NC}"
    # First, run all checks for all crates
    run_all_checks
    
    echo
    echo -e "${YELLOW}📦 All checks passed! Ready to publish crates...${NC}"
    
    # Then publish each crate
    for i in "${!CRATES_TO_PUBLISH[@]}"; do
        crate_name="${CRATES_TO_PUBLISH[$i]}"
        is_last=$((i == ${#CRATES_TO_PUBLISH[@]} - 1))
        publish_crate "$crate_name" "$is_last"
    done
fi

echo
if [[ "$DRY_RUN" == true ]]; then
    if [[ ${#CRATES_TO_PUBLISH[@]} -eq 1 ]]; then
        echo -e "${GREEN}🎉 ${CRATES_TO_PUBLISH[0]} dry-run completed successfully!${NC}"
    else
        echo -e "${GREEN}🎉 All ${#CRATES_TO_PUBLISH[@]} crates dry-run completed successfully!${NC}"
    fi
    echo -e "${BLUE}💡 Run without --dry-run to actually publish the crates.${NC}"
else
    if [[ ${#CRATES_TO_PUBLISH[@]} -eq 1 ]]; then
        echo -e "${GREEN}🎉 ${CRATES_TO_PUBLISH[0]} published successfully!${NC}"
    else
        echo -e "${GREEN}🎉 All ${#CRATES_TO_PUBLISH[@]} crates published successfully!${NC}"
    fi
fi

echo
echo "You can now:"
echo "1. Create a git tag: git tag v0.1.0 && git push origin v0.1.0"
echo "2. Create a GitHub release"
echo "3. Update documentation links" 