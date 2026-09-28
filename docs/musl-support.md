# musl Target Support

cel-cxx supports the `*-unknown-linux-musl` targets:

| Target | Status | Notes |
|--------|--------|-------|
| `x86_64-unknown-linux-musl` | ✅ Tested | Full test suite passes via cross-rs |
| `aarch64-unknown-linux-musl` | ✅ Tested | Test binaries run under qemu-user |
| `i686-unknown-linux-musl` | ✅ Tested | |
| `armv7-unknown-linux-musleabi` | ✅ Tested | Soft-float toolchain; test binaries run under qemu-user |

Binaries are static-pie (`crt-static` is the Rust default for musl), so the
C++ runtime is linked statically into the final executable.

## Requirements

The Bazel build compiles all of libcel (cel-cpp, Abseil, Protobuf, ANTLR
runtime) with the C++ compiler it resolves for the target platform, so a
musl-targeting **C++** toolchain is required — not just `musl-gcc`:

- `$(prefix)g++` with C++17 support and its headers,
- a static `libstdc++.a` in the sysroot,
- **GCC 10 or newer.** Abseil-cpp (20260526.x, the version cel-cpp requires)
  hard-errors on GCC 9 and older via `absl/base/policy_checks.h`.

`CXX_<target>` (with underscores) must point at that compiler for the cxx
bridge/FFI compilation, and `CARGO_TARGET_<TARGET>_LINKER` for the final link.

## Recommended: cross-rs with the bundled images

The repo's `Cross.toml` wires all four musl targets to images built from the
Dockerfiles under `docker/cross/`, so no manual setup is needed:

```bash
cargo install cross --git https://github.com/cross-rs/cross
cross test --target x86_64-unknown-linux-musl
```

cross builds each image on first use (docker's layer cache makes later runs
incremental). The images are based on
[rust-musl-cross](https://github.com/rust-cross/rust-musl-cross), whose
toolchains (GCC 12.4, prefix `<triple>-`, already on `PATH` under
`/usr/local/musl`) satisfy the requirements above, and add:

- `CROSS_TOOLCHAIN_PREFIX` — activates the Bazel cc toolchain wiring from
  [rules_cross_rs](https://github.com/xjasonli/rules_cross_rs) for the libcel
  build;
- `CARGO_TARGET_<TARGET>_LINKER` / `CC_<target>` / `CXX_<target>` — point
  cargo and the cxx bridge at the same toolchain;
- `qemu-user` as the cargo runner for aarch64 and armv7 (test binaries are
  static, so plain `qemu-<arch>` executes them directly);
- for x86_64 only, a `/lib/ld-musl-x86_64.so.1` fixup: the exec configuration
  resolves to the musl toolchain on that target, and its protoc is
  dynamically linked against the toolchain's musl libc — the image's own
  `ld-musl` symlink points at a glibc linker script, so it is re-pointed at
  the real musl libc (see the Dockerfile comments).

The stock cross-rs musl images cannot be used instead: their toolchain is
musl-cross-make pinned at GCC 9.2, which Abseil rejects.

## Alternative: native musl host (e.g. Alpine)

On a host whose default toolchain already targets musl, plain
`cargo build` works with no extra setup: Bazel's auto-configured C++ toolchain
targets musl and matches the musl platform (which deliberately declares only
cpu + os constraints). No cross-rs involved.

## Alternative: host-installed musl cross toolchain

Install a musl cross toolchain with GCC ≥ 10 (e.g. the
[musl.cc](https://musl.cc/) GCC 11.2.1 builds, produced with
[musl-cross-make](https://github.com/richfelker/musl-cross-make)) and expose it
to the build:

```bash
curl -sSL https://musl.cc/x86_64-linux-musl-cross.tgz | tar xz -C ~/.cache/muslcc
T=~/.cache/muslcc/x86_64-linux-musl-cross

env PATH="$T/bin:$PATH" \
  CROSS_TOOLCHAIN_PREFIX=x86_64-linux-musl- \
  CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc \
  CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++ \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc \
  cargo build --target x86_64-unknown-linux-musl --example basic
```

`CROSS_TOOLCHAIN_PREFIX` activates the Bazel toolchain wiring from
[rules_cross_rs](https://github.com/xjasonli/rules_cross_rs), which discovers
`$(prefix)gcc`, `$(prefix)g++`, `$(prefix)ar`, `$(prefix)ld` on `PATH` and
registers a matching cc toolchain. On a glibc host this is what makes the musl
toolchain win over the local glibc toolchain during Bazel platform resolution.

## How It Works

- `cel-build-utils` accepts the musl triples and runs the Bazel build with
  `--platforms=//:x86_64-unknown-linux-musl` (and friends). The musl platforms
  declare the same `[cpu, os:linux]` constraints as their gnu counterparts —
  no libc-specific constraint — so native musl hosts resolve their
  auto-configured toolchain without cross-rs.
- Under cross-rs, `rules_cross_rs` generates a cc toolchain from the
  `CROSS_TOOLCHAIN_PREFIX` tools. Its registration must come from the **root
  module** (`crates/cel-build-utils/cel/MODULE.bazel`): Bzlmod ranks
  root-module registrations above the auto-configured local toolchain, which
  is what makes the musl toolchain win when target and host share an
  architecture (e.g. x86_64 → x86_64-musl, where both toolchains match the
  same platform constraints). Outside cross-rs the generated toolchain is a
  stub that never matches, so native builds are unaffected.
- `cel-cxx` enables `link-cplusplus`'s `libstdc++` feature for
  `cfg(target_env = "musl")`: rustc links libstdc++ automatically on gnu
  targets but has no default C++ runtime for musl.
- 32-bit ARM links `libatomic` (static on musl) for abseil/protobuf's 64-bit
  atomics. rustc resolves the archive through the compiler driver's
  `-print-file-name`, because cross toolchains keep it outside the library
  paths rustc searches on its own.
- The prebuilt libstdc++ for 32-bit ARM contains objects compiled for pre-v6
  cores, whose locale/string refcounting calls the `__sync_*` runtime
  helpers. musl toolchains keep them in libgcc.a, but rustc links with
  `-nodefaultlibs` and its compiler_builtins only ships the
  `__sync_fetch_and_*` half of the family — so the bridge defines the
  missing `__sync_*_and_fetch_*` and `__sync_synchronize` with armv7
  instructions (`crates/cel-cxx-ffi/src/arm_sync_shim.inc`) in the runtime
  object every link pulls.

## armv7 Notes

`armv7-unknown-linux-musleabi` (soft-float) needs a soft-float musl C++
toolchain with GCC ≥ 10. rust-musl-cross ships one (GCC 12.4, the base of the
bundled image), which is what makes the target testable today. Do not reach
for the widely available hard-float `armv7`/`musleabihf` toolchains instead:
the Rust target passes floats in general-purpose registers across the FFI
boundary, and a hard-float compiler would silently corrupt them.

## Caveats

- Linking is memory-hungry: each test/example binary statically links the full
  C++ runtime, and `cargo test` runs many links concurrently. If the linker
  gets OOM-killed (`ld terminated with signal 9`), lower the job count
  (`cargo test -j 4`) and skip doc tests if necessary — rustdoc doctest
  compilation does not respect cargo's `-j`.
- The Bazel action cache under cross-rs containers is not persisted
  (`--output_user_root=/tmp/bazel` lives in the ephemeral container), so every
  `cross` invocation recompiles libcel from scratch. Reusing a path under
  `target/` would make cross builds incremental.
