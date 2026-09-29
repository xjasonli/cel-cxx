use anyhow::Result;
use cel_build_utils::{Artifacts, Build};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    if skip_building() {
        return Ok(());
    }

    track_bazel_workspace();

    let artifacts = build_cpp()?;
    build_ffi(&artifacts)?;

    Ok(())
}

/// The Bazel workspace under `cel-build-utils/cel` builds libcel, but cargo
/// cannot see its inputs. Without these directives a MODULE.bazel or patch
/// edit would keep linking a stale libcel from the previous build.
///
/// Patch files are tracked through the `*.patch` glob below rather than an
/// explicit list: a new patch takes effect only once MODULE.bazel's
/// `git_override(patches = [...])` references it, and MODULE.bazel is
/// tracked here, so adding a patch reruns this build script and re-expands
/// the glob. Tracking the whole `cel/` directory instead would recurse
/// through the `bazel-*` output symlinks that Bazel refreshes on every run
/// and reintroduce an unconditional rebuild loop.
fn track_bazel_workspace() {
    // Paths are relative to this crate's manifest directory. Only true
    // inputs belong here: MODULE.bazel.lock is derived state that Bazel
    // itself rewrites during the build, so tracking it would make cargo
    // rebuild libcel on every invocation (and it is gitignored anyway).
    let files = [
        "../cel-build-utils/cel/MODULE.bazel",
        "../cel-build-utils/cel/BUILD.bazel",
        "../cel-build-utils/cel/.bazelrc",
        "../cel-build-utils/cel/android_conditional.bzl",
        "../cel-build-utils/cel/mingw_conditional.bzl",
    ];
    for file in files {
        println!("cargo:rerun-if-changed={file}");
    }
    // Patches applied to the fetched cel-cpp dependency (for example the
    // Windows timezone fallback) must also invalidate the build.
    if let Ok(patches) = glob::glob("../cel-build-utils/cel/*.patch") {
        for patch in patches.flatten() {
            println!("cargo:rerun-if-changed={}", patch.display());
        }
    }
}

fn build_cpp() -> Result<Artifacts> {
    let artifacts = Build::new().build();
    artifacts.print_cargo_metadata();
    Ok(artifacts)
}

fn build_ffi(artifacts: &Artifacts) -> Result<()> {
    let hh_pattern = "include/**/*.h";
    let _hdrs = glob::glob(hh_pattern)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .inspect(|path| {
            println!("cargo:rerun-if-changed={}", path.display());
        })
        .collect::<Vec<_>>();

    // Implementation fragments included from the .cc files (the 32-bit ARM
    // __sync shim); neither the header nor the cc glob above covers them,
    // and an untracked edit compiles into a stale runtime object.
    for inc in glob::glob("src/**/*.inc")?.flatten() {
        println!("cargo:rerun-if-changed={}", inc.display());
    }

    let cc_pattern = "src/**/*.cc";
    let cc_srcs = glob::glob(cc_pattern)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .inspect(|path| {
            println!("cargo:rerun-if-changed={}", path.display());
        })
        .collect::<Vec<_>>();

    let rs_excludes = ["src/lib.rs", "src/common/mod.rs", "src/cxx.rs"]
        .into_iter()
        .map(Path::new)
        .collect::<Vec<_>>();

    let rs_pattern = "src/**/*.rs";
    let rs_srcs = glob::glob(rs_pattern)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .inspect(|path| {
            println!("cargo:rerun-if-changed={}", path.display());
        })
        .filter(|path| !rs_excludes.contains(&path.as_path()))
        .collect::<Vec<_>>();

    let mut cxx_std = "c++17".to_string();
    let target = std::env::var("TARGET")?;
    if target.contains("msvc") {
        cxx_std = "c++20".to_string();
    }

    let mut build = cxx_build::bridges(rs_srcs);

    build
        .include(artifacts.include_dir())
        .flag_if_supported("-Wno-attributes")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-class-memaccess")
        .flag_if_supported("-Wno-return-type")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-missing-requires")
        .flag_if_supported("-Wno-missing-field-initializers")
        .flag_if_supported("/nologo")
        .flag_if_supported("/EHsc")
        .flag_if_supported("/D_CRT_SECURE_NO_DEPRECATE")
        .flag_if_supported("/D_CRT_SECURE_NO_WARNINGS")
        .flag_if_supported("/Zc:sizedDealloc")
        .files(cc_srcs)
        .std(cxx_std.as_str());

    // armv7: the Debian arm-linux-gnueabi compiler defaults to
    // -march=armv5t (armel), which routes 4-byte atomics through __sync_*
    // runtime helpers that collide with Rust's compiler_builtins at
    // final-link time; match the target arch (same -march cel-build-utils
    // passes to the Bazel-built libcel). Hard-float variants add +fp:
    // a plain -march=armv7-a drops the FPU, which -mfloat-abi=hard
    // then rejects.
    if target.starts_with("armv7") {
        let suffix = if target.contains("gnueabihf") || target.contains("musleabihf") {
            "+fp"
        } else {
            ""
        };
        build.flag(format!("-march=armv7-a{suffix}"));
    }

    #[cfg(windows)]
    build.warnings(false);

    build.compile("cel-cxx");

    println!("cargo:rustc-link-lib=static=cel-cxx");

    // 32-bit ARM: abseil/protobuf use 64-bit atomics, which are libatomic
    // calls there rather than inline instructions; the final Rust link
    // needs the library. Static on musl (the toolchains ship no .so).
    if target.starts_with("arm") && !target.contains("aarch64") && !target.contains("arm64") {
        let kind = if target.contains("musl") { "static" } else { "dylib" };
        // rustc verifies static libraries on its own search path when the
        // crate is compiled, but cross toolchains keep libatomic in a
        // target lib dir only the gcc driver searches. Resolve it the way
        // the driver would and expose the directory.
        if let Some(dir) = toolchain_lib_dir(build.get_compiler().path(), "libatomic.a") {
            println!("cargo:rustc-link-search=native={}", dir.display());
        }
        println!("cargo:rustc-link-lib={kind}=atomic");
    }

    // Windows system libraries libcel uses through _WIN32 code paths.
    // MSVC links them via #pragma comment(lib, ...) in the sources; gcc
    // ignores those pragmas (-Wunknown-pragmas), so the MinGW link needs
    // them on the command line. kernel32 is in the mingw driver's default
    // set, these three are not.
    if target.contains("windows-gnu") {
        println!("cargo:rustc-link-lib=dylib=advapi32"); // Reg* (absl sysinfo)
        println!("cargo:rustc-link-lib=dylib=dbghelp"); // Sym* (absl symbolize)
        println!("cargo:rustc-link-lib=dylib=bcrypt"); // RNG seeding (absl random)
    }
    Ok(())
}

/// Directory holding a C++ runtime library, resolved through the compiler
/// driver the same way the final link resolves it (`-print-file-name`).
/// gcc echoes the file name back unchanged when it is not installed, so
/// only absolute results count. Cross toolchains keep their runtime
/// libraries outside the paths rustc searches on its own.
fn toolchain_lib_dir(compiler: &Path, file: &str) -> Option<PathBuf> {
    let output = Command::new(compiler)
        .arg(format!("-print-file-name={file}"))
        .output()
        .ok()?;
    let found = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let found = Path::new(&found);
    if !found.is_absolute() {
        return None;
    }
    let found = found.canonicalize().unwrap_or_else(|_| found.to_path_buf());
    found.parent().map(Path::to_path_buf)
}

/// Check if we're building for documentation or analysis tools
fn skip_building() -> bool {
    // Check for docs.rs environment
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping C++ build for docs.rs");
        return true;
    }

    // Check for CEL_CXX_FFI_SKIP_BUILD environment variable
    // This is useful when c++ build is not necessary, e.g. when used with cargo check
    //
    // example:
    // CEL_CXX_FFI_SKIP_BUILD=1 cargo check
    if std::env::var("CEL_CXX_FFI_SKIP_BUILD").is_ok() {
        println!("cargo:warning=Skipping C++ build due to CEL_CXX_FFI_SKIP_BUILD");
        return true;
    }

    // Check for analysis tools environment variables
    if is_analysis_tools() {
        println!("cargo:warning=Skipping C++ build for analysis tools");
        return true;
    }

    false
}

/// Detect if the build is being triggered by rust-analyzer / clippy
fn is_analysis_tools() -> bool {
    // Check for rust-analyzer specific environment variables
    if std::env::var("RA_RUSTC_WRAPPER").is_ok() {
        return true;
    }

    // Check if RUSTC_WRAPPER contains rust-analyzer
    if let Ok(wrapper) = std::env::var("RUSTC_WRAPPER") {
        if wrapper.contains("rust-analyzer") {
            return true;
        }
        if wrapper.contains("clippy") {
            return true;
        }
    }

    if let Ok(wrapper) = std::env::var("RUSTC_WORKSPACE_WRAPPER") {
        if wrapper.contains("rust-analyzer") {
            return true;
        }
        if wrapper.contains("clippy") {
            return true;
        }
    }

    false
}
