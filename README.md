[![github]](https://github.com/xjasonli/cel-cxx)
[![crates-io]](https://crates.io/crates/cel-cxx)
[![docs-rs]](https://docs.rs/cel-cxx)
[![deepwiki]](https://deepwiki.com/xjasonli/cel-cxx)

[github]: https://img.shields.io/badge/GitHub-Repository-blue?style=flat&logo=github
[crates-io]: https://img.shields.io/crates/v/cel-cxx.svg
[docs-rs]: https://docs.rs/cel-cxx/badge.svg
[deepwiki]: https://deepwiki.com/badge.svg

- [CEL-CXX](#cel-cxx)
  - [Documentation](#documentation)
  - [Quick Start](#quick-start)
    - [Installation](#installation)
    - [Basic Expression Evaluation](#basic-expression-evaluation)
    - [Custom Types with Derive Macros](#custom-types-with-derive-macros)
  - [Platform Support](#platform-support)
    - [Cross-Compilation Support](#cross-compilation-support)
    - [Apple Builds and Simulator Tests](#apple-builds-and-simulator-tests)
    - [Android Build Instructions](#android-build-instructions)
    - [Windows GNU Builds](#windows-gnu-builds)
  - [CEL Feature Support](#cel-feature-support)
    - [Core Language Features](#core-language-features)
    - [Standard Library](#standard-library)
    - [Optional Value Support](#optional-value-support)
    - [Extension Libraries](#extension-libraries)
    - [Runtime Features](#runtime-features)
  - [Protobuf Integration](#protobuf-integration)
    - [Typed API with Derive Macros](#typed-api-with-derive-macros)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

# CEL-CXX

A type-safe Rust library for [Common Expression Language (CEL)](https://github.com/google/cel-spec), built on top of [cel-cpp](https://github.com/google/cel-cpp) with zero-cost FFI bindings via [cxx](https://github.com/dtolnay/cxx).

## Documentation

- [English Documentation](https://github.com/xjasonli/cel-cxx/tree/master/docs/) - Complete documentation and guides
- [中文文档](https://github.com/xjasonli/cel-cxx/tree/master/docs-cn/) - 中文文档和指南
- [Runnable examples](examples/README.md) - Example index, feature requirements, and run commands

For detailed guides on architecture, function registration, type system, and advanced features, see the [documentation directory](https://github.com/xjasonli/cel-cxx/tree/master/docs/).

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cel-cxx = "0.3.0"

# Optional features
cel-cxx = { version = "0.3.0", features = ["tokio"] }

# Protobuf derive macros (choose your backend)
cel-cxx = { version = "0.3.0", features = ["prost"] }
cel-cxx = { version = "0.3.0", features = ["protobuf-legacy"] }
```

### Basic Expression Evaluation

```rust,no_run
use cel_cxx::*;

// 1. Build environment with variables and functions
let env = Env::builder()
    .declare_variable::<String>("name")?
    .declare_variable::<i64>("age")?
    .register_global_function("adult", |age: i64| age >= 18)?
    .build()?;

// 2. Compile expression
let program = env.compile("'Hello ' + name + '! You are ' + (adult(age) ? 'an adult' : 'a minor')")?;

// 3. Create activation with variable bindings
let activation = Activation::new()
    .bind_variable("name", "Alice")?
    .bind_variable("age", 25i64)?;

// 4. Evaluate
let result = program.evaluate(&activation)?;
println!("{}", result); // "Hello Alice! You are an adult"
# Ok::<(), cel_cxx::Error>(())
```

### Custom Types with Derive Macros

```rust,no_run
use cel_cxx::*;

#[derive(Opaque, Debug, Clone, PartialEq)]
// Specify type name in CEL type system.
#[cel_cxx(type = "myapp.User")]
// Generates `std::fmt::Display` impl for User` with `Debug` trait.
#[cel_cxx(display)]
// or you can specify a custom format.
// Generates `std::fmt::Display` impl with custom format.
#[cel_cxx(display = write!(fmt, "User(name={name})", name = &self.name))]
struct User {
    name: String,
    age: i32,
    roles: Vec<String>,
}

impl User {
    // Struct methods can be registered directly as CEL member functions
    fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    fn get_role_count(&self) -> i64 {
        self.roles.len() as i64
    }
}

let env = Env::builder()
    .declare_variable::<User>("user")?
    // ✨ Register struct methods directly - &self becomes CEL receiver
    .register_member_function("has_role", User::has_role)?
    .register_member_function("is_adult", User::is_adult)?
    .register_member_function("get_role_count", User::get_role_count)?
    .build()?;

let program = env.compile("user.has_role('admin') && user.is_adult()")?;
# Ok::<(), cel_cxx::Error>(())
```

## Platform Support

<table>
<thead>
<tr>
<th>Platform</th>
<th>Target Triple</th>
<th>Build</th>
<th>Test</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr>
<td rowspan="9"><strong>Linux</strong></td>
<td><code>x86_64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>aarch64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>armv7-unknown-linux-gnueabi</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>armv7-unknown-linux-gnueabihf</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>i686-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>i586-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>powerpc64le-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>riscv64gc-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>loongarch64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td rowspan="8"><strong>Linux (musl)</strong></td>
<td><code>x86_64-unknown-linux-musl</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs; the repo's <code>Cross.toml</code> builds GCC 12.4 images from rust-musl-cross (stock cross-rs musl images ship GCC 9.2, too old for Abseil) — see [musl support](docs/musl-support.md)</td>
</tr>
<tr>
<td><code>aarch64-unknown-linux-musl</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs (qemu-user)</td>
</tr>
<tr>
<td><code>i686-unknown-linux-musl</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>i586-unknown-linux-musl</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>armv7-unknown-linux-musleabi</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs (qemu-user); soft-float toolchain, see [armv7 notes](docs/musl-support.md#armv7-notes)</td>
</tr>
<tr>
<td><code>armv7-unknown-linux-musleabihf</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs (qemu-user); hard-float toolchain, same armv7 notes</td>
</tr>
<tr>
<td><code>loongarch64-unknown-linux-musl</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs (qemu-user); stock cross-rs image (crosstool-NG 14.2)</td>
</tr>
<tr>
<td><code>riscv64gc-unknown-linux-musl</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs (qemu-user); stock cross-rs image (crosstool-NG 14.2)</td>
</tr>
<tr>
<td><strong>Windows</strong></td>
<td><code>x86_64-pc-windows-msvc</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested (Visual Studio 2022+; named IANA timezones resolve through the OS-managed database)</td>
</tr>
<tr>
<td><strong>Windows</strong></td>
<td><code>x86_64-pc-windows-gnu</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested from an MSYS2 UCRT64 shell (MinGW-w64 GCC); see [Windows GNU builds](#windows-gnu-builds). Named IANA timezones resolve only through a <code>TZDIR</code> tree</td>
</tr>
<tr>
<td rowspan="2"><strong>macOS</strong></td>
<td><code>x86_64-apple-darwin</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed through Rosetta on macOS 27.0.1 (2026-10-09); see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td><code>aarch64-apple-darwin</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed natively on macOS 27.0.1; see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td rowspan="2"><strong>Mac Catalyst</strong></td>
<td><code>aarch64-apple-ios-macabi</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed natively on macOS 27.0.1 (2026-10-09); see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td><code>x86_64-apple-ios-macabi</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed through Rosetta on macOS 27.0.1 (2026-10-09); see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td rowspan="4"><strong>Android</strong></td>
<td><code>aarch64-linux-android</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cargo-ndk on an emulator (arm64 binary under NDK translation; see [running tests on a device/emulator](#running-tests-on-a-device-or-emulator))</td>
</tr>
<tr>
<td><code>armv7-linux-androideabi</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cargo-ndk (outside cross-rs; not re-verified in this round)</td>
</tr>
<tr>
<td><code>x86_64-linux-android</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cargo-ndk on an emulator (native)</td>
</tr>
<tr>
<td><code>i686-linux-android</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cargo-ndk on an x86 emulator (native)</td>
</tr>
<tr>
<td rowspan="3"><strong>iOS</strong></td>
<td><code>aarch64-apple-ios</code></td>
<td>✅</td>
<td>🟡</td>
<td>Build and link verified; device tests not yet run</td>
</tr>
<tr>
<td><code>aarch64-apple-ios-sim</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed on an ARM64 iOS 27.0 simulator; see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td><code>x86_64-apple-ios</code></td>
<td>✅</td>
<td>🟡</td>
<td>Test binaries linked; simulator tests unverified</td>
</tr>
<tr>
<td rowspan="2"><strong>tvOS</strong></td>
<td><code>aarch64-apple-tvos</code></td>
<td>✅</td>
<td>🟡</td>
<td>Build and link verified; device tests not yet run</td>
</tr>
<tr>
<td><code>aarch64-apple-tvos-sim</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed on an ARM64 tvOS 26.5 simulator; see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td rowspan="2"><strong>watchOS</strong></td>
<td><code>aarch64-apple-watchos</code></td>
<td>✅</td>
<td>🟡</td>
<td>Build and link verified (minimum watchOS 26.0); device tests not yet run</td>
</tr>
<tr>
<td><code>aarch64-apple-watchos-sim</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed on an ARM64 watchOS 27.0 simulator; see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td rowspan="2"><strong>visionOS</strong></td>
<td><code>aarch64-apple-visionos</code></td>
<td>✅</td>
<td>🟡</td>
<td>Build and link verified; device tests not yet run</td>
</tr>
<tr>
<td><code>aarch64-apple-visionos-sim</code></td>
<td>✅</td>
<td>✅</td>
<td>322 tests passed on an ARM64 visionOS 26.5 simulator; see <a href="#apple-verification">Apple verification</a></td>
</tr>
<tr>
<td><strong>WebAssembly</strong></td>
<td><code>wasm32-unknown-emscripten</code></td>
<td>✅</td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
</tbody>
</table>

**Legend:**

- ✅: verified — **Build** compiles, **Test** passes the test suite
- 🟡: runtime tests have not been verified for this target

### Timestamp Timezone Resolution

Timestamp accessors that take a timezone argument accept numeric offsets
(`'-04:00'`, resolved on every host) and named IANA zones
(`'America/New_York'`). Named zones are resolved from the zone database
available to the runtime:

- Linux and macOS read the TZif tree used by Abseil/cctz (`$TZDIR`, by
  default `/usr/share/zoneinfo`). Minimal container images may need `tzdata`
  installed.
- Android devices resolve zones from the system-packed tzdata that cctz
  falls back to (`/apex/com.android.tzdata/etc/tz/tzdata` and legacy
  locations); host-side Android builds without it behave like hosts with no
  database.
- Windows ships no TZif tree, so named zones resolve through the OS-managed
  IANA database via MSVC's C++20 `<chrono>` (Windows 10 1903+); an explicit
  `TZDIR` tree takes precedence. This fallback is MSVC-only: on
  `x86_64-pc-windows-gnu` (libstdc++ has no IANA `<chrono>` backend), named
  zones resolve only when `TZDIR` points at a TZif tree — MSYS2 provides one
  at `/usr/share/zoneinfo` (set
  `TZDIR=C:/msys64/usr/share/zoneinfo` for a process). Numeric offsets
  (`'-04:00'`) never need a database.
- WebAssembly and other hosts without any source return an
  invalid-argument error for named zones.

Results follow the host's database version: a zone with recently changed
rules can resolve differently on hosts with different data versions. See
[docs/timezone-support-design.md](docs/timezone-support-design.md) for the
full design.

### Cross-Compilation Support

cel-cxx includes built-in support for cross-compilation via [cross-rs](https://github.com/cross-rs/cross). The build system automatically detects cross-compilation environments and configures the appropriate toolchains.

**Usage with cross-rs:**
```bash
# Install cross-rs
cargo install cross --git https://github.com/cross-rs/cross

# Build for aarch64
cross build --target aarch64-unknown-linux-gnu
```

**Note**: Not all cross-rs targets are supported due to CEL-CPP's build requirements. musl
targets work out of the box through this repo's `Cross.toml`: cross builds the images
(GCC 12.4, based on [rust-musl-cross](https://github.com/rust-cross/rust-musl-cross))
on first use, since the stock cross-rs musl images ship a GCC 9.2 toolchain that is too
old for cel-cxx's Abseil dependency (GCC ≥ 10 required). Non-docker alternatives — a
host-installed musl toolchain, or plain `cargo build` on native musl hosts such as
Alpine — are described in [docs/musl-support.md](docs/musl-support.md).

### Apple Builds and Simulator Tests

Apple cross-builds require a macOS host with full Xcode selected (Command Line
Tools alone do not provide the iOS, tvOS, watchOS, or visionOS SDKs). Check the
selected installation and available SDKs/runtimes with:

```bash
xcode-select -p
xcodebuild -version
xcodebuild -showsdks
xcrun simctl list runtimes
xcrun simctl list devices available
```

Install any missing simulator runtimes in Xcode's settings. Physical devices
are unnecessary for simulator tests or device cross-builds; executing a
device binary still requires matching hardware and an appropriate app/signing
setup. Simulator tests below run standalone Rust test executables through
`simctl`, without an Xcode app project.

**Building:** install the target's Rust standard library, then build a linked
example (this verifies the Rust/C++ link, as well as library compilation):

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin \
    aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios \
    aarch64-apple-ios-macabi x86_64-apple-ios-macabi \
    aarch64-apple-tvos aarch64-apple-tvos-sim \
    aarch64-apple-watchos aarch64-apple-watchos-sim \
    aarch64-apple-visionos aarch64-apple-visionos-sim
cargo build --target aarch64-apple-ios --example basic
```

**Running macOS and Mac Catalyst tests:** these targets run directly on macOS.
The x86_64 binaries require Rosetta on an Apple Silicon host. Mac Catalyst
uses the macOS SDK's `System/iOSSupport` headers and libraries, with iOS
versions for `IPHONEOS_DEPLOYMENT_TARGET`; see the
[Rust target requirements](https://doc.rust-lang.org/rustc/platform-support/apple-ios-macabi.html).

```bash
cargo test --target aarch64-apple-darwin --tests
cargo test --target x86_64-apple-darwin --tests
cargo test --target aarch64-apple-ios-macabi --tests
cargo test --target x86_64-apple-ios-macabi --tests
```

**Running simulator tests:** from the repository root, select a simulator UUID
whose platform matches the target. For example, for an ARM64 iOS simulator:

```bash
export CEL_CXX_SIMULATOR_UDID=<simulator-uuid>
xcrun simctl boot "$CEL_CXX_SIMULATOR_UDID"  # omit if already booted
xcrun simctl bootstatus "$CEL_CXX_SIMULATOR_UDID" -b
export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_RUNNER="$PWD/scripts/apple-simulator-runner.sh"
cargo test --target aarch64-apple-ios-sim --tests

# Broader feature coverage; requires protoc and its standard .proto includes
# on the host (protoc must be on PATH for protobuf-legacy).
cargo test --target aarch64-apple-ios-sim --tests \
    --features tokio,smol,prost,protobuf-legacy
```

Use `CARGO_TARGET_AARCH64_APPLE_TVOS_SIM_RUNNER`,
`CARGO_TARGET_AARCH64_APPLE_WATCHOS_SIM_RUNNER`, or
`CARGO_TARGET_AARCH64_APPLE_VISIONOS_SIM_RUNNER` with the same
[runner](scripts/apple-simulator-runner.sh) for the other ARM64 simulators.
The runner copies `tests/fixtures` to the simulator's working directory so
runtime Protobuf descriptor compilation can find the inputs. `--tests`
covers unit/integration tests; it does not run documentation tests.

Set deployment targets consistently when overriding the defaults:
`MACOSX_DEPLOYMENT_TARGET`, `IPHONEOS_DEPLOYMENT_TARGET`,
`TVOS_DEPLOYMENT_TARGET`, `WATCHOS_DEPLOYMENT_TARGET`, and
`XROS_DEPLOYMENT_TARGET` are forwarded to the Bazel C++ build as well as
used by Rust/cc-rs. A build against a recent SDK is not evidence that every
older OS release supported by the deployment setting has been tested.

#### Apple Verification

All **13 Apple target triples** listed above built and linked successfully
during **2026-09-30–2026-10-09**, using an Apple M2 Mac with macOS 27.0.1,
Xcode 27.0 (27A266a), SDKs 27.0, and Rust 1.96.0.

Both architectures of macOS and Mac Catalyst, and each ARM64 simulator,
passed **322 tests across 26 unit/integration test executables**, with default
`derive` plus `tokio,smol,prost,protobuf-legacy`. Simulator runtimes were
iOS 27.0, tvOS 26.5, watchOS 27.0, and visionOS 26.5. The iOS simulator also
passed all 269 default-feature tests. No unit/integration tests were ignored or filtered out.
Both architectures of macOS and Mac Catalyst passed their documentation
tests: 317 passed, 4 explicitly ignored, 0 failed. The x86_64 macOS and
Mac Catalyst test suites ran through Rosetta. Mac Catalyst executables and sampled C++
objects were confirmed as `MACCATALYST`, with minimum iOS versions of 14.0
on ARM64 and 13.1 on x86_64. Named IANA timezones and `TZDIR` precedence
passed on macOS, Mac Catalyst, and in these simulator processes.

Device targets were verified with `cargo build --example basic` and the same
additional features, including the final Rust/C++ link. Runtime tests on iOS,
tvOS, watchOS, and visionOS devices have not yet been performed. Simulator
results do not establish device compatibility, timezone availability, or
app-signing compatibility. The `x86_64-apple-ios` test executables linked
successfully; runtime tests remain unverified and require an x86_64-compatible
iOS simulator runtime.

The build uses the Apple C++ toolchain for macOS execution tools as well as
for target code. Explicit tvOS/watchOS/visionOS deployment defaults avoid
Bazel's fallback to the installed SDK version. Rust may enforce a higher
minimum than an environment override: the verified ARM64 simulator
executables reported iOS/tvOS 14.0, watchOS 7.0, and visionOS 1.0. Older OS
versions were not tested. The ARM64 watchOS **device** example reported
watchOS 26.0; its deployment requirement should not be inferred from the
simulator result.

### Android Build Instructions

Android builds require additional setup beyond the standard Rust toolchain:

**Prerequisites:**
1. Install Android NDK and set `ANDROID_NDK_HOME`
2. Install `cargo-ndk` for simplified Android builds

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

**Building for Android:**
```bash
# Build for ARM64 (recommended)
cargo ndk --target aarch64-linux-android build

# Build for ARMv7
cargo ndk --target armv7-linux-androideabi build

# Build for x86_64 (emulator)
cargo ndk --target x86_64-linux-android build

# Build for i686 (emulator)
cargo ndk --target i686-linux-android build
```

**Why cargo-ndk is required:**
- `ANDROID_NDK_HOME` configures Bazel for CEL-CPP compilation
- `cargo-ndk` automatically sets up `CC_{target}` and `AR_{target}` environment variables needed for the Rust FFI layer
- This ensures both the C++ (CEL-CPP) and Rust (cel-cxx-ffi) components use compatible toolchains

**Host platforms:** Android cross-builds work from Linux, macOS, and Windows hosts. On Windows,
`cargo ndk --target aarch64-linux-android build` needs no extra setup beyond `cargo-ndk`, the NDK,
and MSYS2 (for the `bash`/`patch` tools Bazel uses); the build system neutralizes the Android SDK
repository environment because libcel only uses the NDK toolchain.

#### Running tests on a device or emulator

The test binaries link `libc++_shared.so` dynamically, so push the NDK's copy alongside the
binary and give the executable a runpath — `LD_LIBRARY_PATH` does **not** work for arm64
binaries launched through the NDK translation layer on x86 emulators (environment variables do
not reach processes started via the binfmt runner, but `DT_RUNPATH` does):

```bash
NDK=<ndk-dir>
RUSTFLAGS='-Clink-arg=-Wl,-rpath,/data/local/tmp' \
ANDROID_NDK_HOME=$NDK cargo ndk -t aarch64-linux-android -P 30 test --tests --no-run

adb push $NDK/toolchains/llvm/prebuilt/*/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so \
    /data/local/tmp/
adb push target/aarch64-linux-android/debug/deps/<test-binary> /data/local/tmp/t
adb shell chmod 755 /data/local/tmp/t
adb shell /data/local/tmp/t
```

The `protobuf`/`prost_derive`/`protobuf_legacy_derive` suites read `tests/fixtures/*.proto`
relative to the working directory, so push that directory to `/data/local/tmp/tests/fixtures`
and run with `cd /data/local/tmp`.

### Windows GNU Builds

The `x86_64-pc-windows-gnu` target needs a MinGW-w64 C++ toolchain on `PATH` —
Bazel compiles all of libcel (cel-cpp, Abseil, Protobuf, the ANTLR runtime)
with the same g++ that compiles the cxx bridge, so the ABI matches the GNU
linker. The native path requires no docker and no cross-rs:

**Requirements:**

1. [MSYS2](https://www.msys2.org/) with the UCRT64 environment installed
   (`gcc`, `g++`, `binutils` in `/ucrt64/bin`; GCC 10 or newer).
2. The Rust target: `rustup target add x86_64-pc-windows-gnu`.

```bash
# From an MSYS2 UCRT64 shell (bash)
cargo build --target x86_64-pc-windows-gnu --example basic
cargo test --target x86_64-pc-windows-gnu
```

The Bazel build resolves the gcc toolchain through a conditional extension in
`cel/MODULE.bazel` that activates whenever the target triple being built is
`x86_64-pc-windows-gnu` (see [mingw_conditional.bzl](crates/cel-build-utils/cel/platforms/mingw_conditional.bzl)); every other target —
including `x86_64-pc-windows-msvc` on the same host — is unaffected. Any host
with a mingw-w64 toolchain on `PATH` works the same way, and the docker +
cross-rs route is also available from any host, including Linux:

```bash
# Any host with docker + cross-rs; builds with the stock cross image
cross build --target x86_64-pc-windows-gnu --example basic
```

Binaries built **inside** the cross container link the Ubuntu mingw-w64
(msvcrt-flavor) C++ runtime dynamically: copy `libstdc++-6.dll`,
`libgcc_s_seh-1.dll`, and `libwinpthread-1.dll` from the image (under
`/usr/lib/gcc/x86_64-w64-mingw32/` and `/usr/x86_64-w64-mingw32/lib/`) next to
the binary to run it — the MSYS2 `libstdc++-6.dll` is a UCRT flavor and does
not substitute for it.

**Runtime:** binaries link the MSYS2 UCRT64 C++ runtime dynamically
(`libstdc++-6.dll`), which is found through `PATH` inside an MSYS2 shell;
copy it next to the binary (or keep `/ucrt64/bin` on `PATH`) to run it
elsewhere. Named IANA timezones need
`TZDIR` to point at a TZif tree (see
[Timestamp Timezone Resolution](#timestamp-timezone-resolution)).

## CEL Feature Support

### Core Language Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Basic Types** | ✅ | `null`, `bool`, `int`, `uint`, `double`, `string`, `bytes` |
| **Collections** | ✅ | `list<T>`, `map<K,V>` with full indexing and comprehensions |
| **Time Types** | ✅ | `duration`, `timestamp` with full arithmetic support |
| **Operators** | ✅ | Arithmetic, logical, comparison, and membership operators |
| **Variables** | ✅ | Variable binding and scoping |
| **Conditionals** | ✅ | Ternary operator and logical short-circuiting |
| **Comprehensions** | ✅ | List and map comprehensions with filtering |
| **Custom Types** | ✅ | Opaque types via `#[derive(Opaque)]` |
| **Protobuf Message Type** | ✅ | Native protobuf messages and enums as first-class CEL types with field access, message construction, and round-trip serialization |
| **Macros** | ✅ | CEL macro expansion support |
| **Function Overloads** | ✅ | Multiple function signatures with automatic resolution |
| **Type Checking** | ✅ | Compile-time type validation |

### Standard Library

| Feature | Status | Description |
|---------|--------|-------------|
| **Built-in Functions** | ✅ | Core CEL functions: `size()`, `type()`, `has()`, etc. |
| **String Functions** | ✅ | `contains()`, `startsWith()`, `endsWith()`, `matches()` |
| **List Functions** | ✅ | `all()`, `exists()`, `exists_one()`, `filter()`, `map()` |
| **Map Functions** | ✅ | Key/value iteration and manipulation |
| **Type Conversion** | ✅ | `int()`, `double()`, `string()`, `bytes()`, `duration()`, `timestamp()` |
| **Math Functions** | ✅ | Basic arithmetic and comparison operations |

### Optional Value Support

| Feature | Status | Description |
|---------|--------|-------------|
| **Optional Types** | ✅ | `optional<T>` with safe navigation and null handling |
| **Safe Navigation** | ✅ | `?.` operator for safe member access |
| **Optional Chaining** | ✅ | Chain optional operations without explicit null checks |
| **Value Extraction** | ✅ | `value()` and `hasValue()` functions for optional handling |
| **Optional Macros** | ✅ | `optional.of()`, `optional.ofNonZeroValue()` macros |

### Extension Libraries

| Extension | Status | Description |
|-----------|--------|-------------|
| **Strings Extension** | ✅ | Advanced string operations: `split()`, `join()`, `replace()`, `format()` |
| **Math Extension** | ✅ | Mathematical functions: `math.greatest()`, `math.least()`, `math.abs()`, `math.sqrt()`, bitwise ops |
| **Lists Extension** | ✅ | Enhanced list operations: `flatten()`, `reverse()`, `slice()`, `unique()` |
| **Sets Extension** | ✅ | Set operations: `sets.contains()`, `sets.equivalent()`, `sets.intersects()` |
| **Regex Extension** | ✅ | Regular expression support: `matches()`, `findAll()`, `split()` |
| **Encoders Extension** | ✅ | Encoding/decoding: `base64.encode()`, `base64.decode()`, URL encoding |
| **Bindings Extension** | ✅ | Variable binding and scoping enhancements |

### Runtime Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Custom Functions** | ✅ | Register custom Rust functions with automatic type conversion |
| **Async Support** | ✅ | Async function calls and evaluation with Tokio or smol integration |
| **Custom Extensions** | ✅ | Build and register custom CEL extensions |
| **Performance Optimization** | ✅ | Optimized evaluation with caching and short-circuiting |

## Protobuf Integration

cel-cxx supports native Protocol Buffer messages as first-class CEL types. You can bind serialized protobuf messages as variables, access their fields in CEL expressions, construct new messages, and extract results back to Rust.

### Compiling Proto Descriptors

CEL needs a `FileDescriptorSet` (a binary descriptor file) to understand your `.proto` types.
The recommended approach is [`protox`](https://crates.io/crates/protox), a pure-Rust protobuf
compiler that requires no external binary:

```rust,no_run
use prost::Message;

let fds = protox::compile(["proto/my_service.proto"], ["proto/"]).unwrap();
let descriptor_bytes = fds.encode_to_vec();
```

Alternatively, you can use `protoc` directly:

```bash
protoc --descriptor_set_out=descriptors.bin \
       --include_imports \
       --proto_path=proto \
       proto/my_service.proto
```

### Setting Up the Environment

```rust,no_run
use cel_cxx::*;

# let descriptor_bytes: Vec<u8> = vec![];
let env = Env::builder()
    .with_file_descriptor_set(&descriptor_bytes)
    .declare_variable_with_type("msg", ValueType::Struct(StructType::new("my.package.MyMessage")))?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

### Binding Protobuf Input

Serialize your message to bytes (e.g., via `prost`) and bind it:

```rust,no_run
# use cel_cxx::*;
# let serialized_bytes: Vec<u8> = vec![];
let activation = Activation::new()
    .bind_variable_dynamic("msg", StructValue::from_bytes("my.package.MyMessage", serialized_bytes))?;
# Ok::<(), cel_cxx::Error>(())
```

### Accessing Fields in CEL

CEL expressions can access all protobuf field types:

```cel
msg.name                              // scalar fields
msg.address.city                      // nested message fields
msg.tags[0]                           // repeated fields
msg.labels["env"]                     // map fields
msg.status == my.package.Status.ACTIVE // enum constants
has(msg.optional_field)               // field presence
```

### Message Construction

Construct new protobuf messages directly in CEL:

```cel
my.package.MyMessage{name: "Alice", id: 42}
```

### Extracting Results

```rust,no_run
# use cel_cxx::*;
# fn example(result: Value, env: Env) -> Result<(), Error> {
// Borrow via as_struct()
let sv = result.as_struct().unwrap();
let type_name = sv.type_name();
let bytes = sv.to_bytes();

// Or extract an owned StructValue
let sv = StructValue::from_value(&result)?;

// Read individual fields from Rust
let name = env.get_struct_field(&sv, "name")?;
let has_name = env.has_struct_field(&sv, "name")?;
# Ok(())
# }
```

### Typed API with Derive Macros

If you have compile-time protobuf types (via `prost-build` or `protobuf-codegen`), derive
macros let you skip the manual `StructValue::from_bytes` plumbing and use the standard
typed API instead:

```rust,ignore
// With the `prost` or `protobuf-legacy` feature enabled:
let env = Env::builder()
    .with_file_descriptor_set(&descriptors)
    .declare_variable::<MyMessage>("msg")?  // instead of declare_variable_with_type(...)
    .build()?;

let activation = Activation::new()
    .bind_variable("msg", my_message)?;  // instead of bind_variable_dynamic(...)

let result = program.evaluate(&activation)?;
let recovered = MyMessage::from_value(&result)?;  // instead of StructValue::from_value(...)
```

The derives are injected during code generation in your `build.rs` -- one line per type
for prost, or a small `CustomizeCallback` for protobuf-codegen.

See the **[Protobuf Derive Macros Guide](docs/protobuf-derive.md)** for full setup
instructions, feature flags, and examples.

### Well-Known Type Handling

cel-cpp automatically converts well-known types to their CEL equivalents:

| Protobuf Type | CEL Type | Notes |
|--------------|----------|-------|
| `google.protobuf.Duration` | `duration` | Full arithmetic and comparison support |
| `google.protobuf.Timestamp` | `timestamp` | Full arithmetic and comparison support |
| `google.protobuf.Int64Value`, `StringValue`, `BoolValue`, ... | Primitives | Auto-unboxed to `int`, `string`, `bool`, etc. |
| `google.protobuf.Struct` | `map` | Dynamic map with dot-notation access |
| `google.protobuf.Any` | — | `has()` works; full unpacking depends on cel-cpp support |

### Performance Note

Protobuf messages cross the Rust/C++ FFI boundary via serialization: messages are serialized to bytes on the Rust side and deserialized into arena-allocated C++ messages for evaluation, then serialized back when extracting results. This adds overhead proportional to message size.

Zero-copy is not currently implemented for two reasons. First, the architectural change to keep C++ arena-allocated messages alive across the FFI boundary would be significant. Second, true zero-copy would require both cel-cxx and the Rust protobuf library to link against the *exact same* C++ protobuf library instance so that message pointers can be passed directly across the boundary. The official Google Rust protobuf crate ([`protobuf`](https://crates.io/crates/protobuf)) supports a C++ kernel backend that would make this possible in theory, but it currently requires Bazel (not cargo), and there is no shared `protobuf-cpp-sys` crate that both libraries could depend on. cel-cxx also compiles its own copy of C++ protobuf as part of cel-cpp via cmake, so integrating a shared dependency would require reworking the build. Until the ecosystem converges, serialization/deserialization is the only correct approach.

## License

Licensed under the Apache License 2.0. See [LICENSE](https://github.com/xjasonli/cel-cxx/blob/master/LICENSE) for details.

## Acknowledgements

- [`google/cel-cpp`](https://github.com/google/cel-cpp) - The foundational C++ CEL implementation
- [`dtolnay/cxx`](https://github.com/dtolnay/cxx) - Safe and efficient Rust-C++ interop
- [`rmanoka/async-scoped`](https://github.com/rmanoka/async-scoped) - Scoped async execution for safe lifetime management
- The CEL community and other Rust CEL implementations for inspiration and ecosystem growth
