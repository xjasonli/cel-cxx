#!/bin/bash
# Cargo runner for Rust test executables on a booted Apple simulator.
set -euo pipefail

: "${CEL_CXX_SIMULATOR_UDID:?Set CEL_CXX_SIMULATOR_UDID to a booted simulator UUID}"
if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <test-executable> [arguments...]" >&2
    exit 2
fi

repo_dir=$(cd "$(dirname "$0")/.." && pwd)
simulator_home=$(xcrun simctl getenv "$CEL_CXX_SIMULATOR_UDID" HOME)

# simctl starts executables in the simulator's data directory, rather than
# Cargo's working directory. The protobuf suites open these relative paths.
mkdir -p "$simulator_home/tests"
cp -R "$repo_dir/tests/fixtures" "$simulator_home/tests/"

executable=$1
shift
case "$executable" in
    /*) ;;
    *) executable="$PWD/$executable" ;;
esac

exec xcrun simctl spawn "$CEL_CXX_SIMULATOR_UDID" "$executable" "$@"
