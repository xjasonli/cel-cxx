# musl Target Support

cel-cxx supports the `*-unknown-linux-musl` targets:

| Target | Status | Notes |
|--------|--------|-------|
| `x86_64-unknown-linux-musl` | ✅ Tested | Full test suite passes |
| `aarch64-unknown-linux-musl` | ✅ Tested | |
| `i686-unknown-linux-musl` | 🟡 Should work | Same mechanism, not exercised in CI |
| `armv7-unknown-linux-musleabi` | 🟡 Toolchain gap | No GCC ≥ 10 soft-float prebuilt exists; see [armv7 notes](#armv7-notes) |

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

## Toolchain Options

### Option 1: Native musl host (e.g. Alpine)

On a host whose default toolchain already targets musl, plain
`cargo build` works with no extra setup: Bazel's auto-configured C++ toolchain
targets musl and matches the musl platform (which deliberately declares only
cpu + os constraints). No cross-rs involved.

### Option 2: Host-installed musl cross toolchain

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

### Option 3: cross-rs with a custom image

The **stock cross-rs musl images cannot build cel-cxx**: they compile their
toolchain with musl-cross-make pinned at GCC 9.2, which Abseil rejects
(`#error "This package requires GCC 10 or higher."`).

Layer a newer toolchain on top of the cross image, so it shadows GCC 9.2 on
`PATH` and its runtime libraries are visible to the musl dynamic loader (the
stock sysroot keeps a linker script where `libstdc++.so` would be, which the
loader cannot use):

```dockerfile
# Dockerfile.cross-musl-x86_64
FROM ghcr.io/cross-rs/x86_64-unknown-linux-musl:main
COPY x86_64-linux-musl-cross /opt/musl-gcc11/x86_64-linux-musl-cross
ENV PATH=/opt/musl-gcc11/x86_64-linux-musl-cross/bin:$PATH
RUN sed -i '1i /opt/musl-gcc11/x86_64-linux-musl-cross/x86_64-linux-musl/lib' \
    /etc/ld-musl-x86_64.path
```

```bash
curl -sSL https://musl.cc/x86_64-linux-musl-cross.tgz | tar xz
docker build -t cel-cxx-musl:x86_64 -f Dockerfile.cross-musl-x86_64 .
```

Then point cross at it with a `Cross.toml` in the project root:

```toml
[target.x86_64-unknown-linux-musl]
image = "cel-cxx-musl:x86_64"
```

and build normally:

```bash
cross build --target x86_64-unknown-linux-musl --example basic
```

Repeat per architecture (`aarch64-linux-musl-cross.tgz`,
`i686-linux-musl-cross.tgz` from musl.cc; adjust the `FROM` image and the
`ld-musl-<arch>.path` file name accordingly).

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

## armv7 Notes

`armv7-unknown-linux-musleabi` (soft-float) needs a soft-float musl C++
toolchain with GCC ≥ 10. No public prebuilt provides one today — musl.cc and
Bootlin only ship `armv7` hard-float (`musleabihf`) builds — so your options
are:

- build [musl-cross-make](https://github.com/richfelker/musl-cross-make)
  yourself with a newer `GCC_VER` and `--with-float=soft`, or
- target `armv7-unknown-linux-musleabihf` instead, for which both a musl.cc
  toolchain and a cross-rs image exist (add the triple to `SUPPORTED_TARGETS`
  in `crates/cel-build-utils/src/lib.rs` and a platform to
  `crates/cel-build-utils/cel/BUILD.bazel`).

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
