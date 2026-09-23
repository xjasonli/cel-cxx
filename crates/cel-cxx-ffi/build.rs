use anyhow::Result;
use cel_build_utils::{Artifacts, Build};
use std::path::Path;

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

    #[cfg(windows)]
    build.warnings(false);

    build.compile("cel-cxx");

    println!("cargo:rustc-link-lib=static=cel-cxx");
    Ok(())
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
