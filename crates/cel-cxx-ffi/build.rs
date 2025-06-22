use std::path::Path;
use anyhow::Result;
use cel_build_utils::{Artifacts, Build};

fn main() -> Result<()> {
    // Skip C++ compilation when building docs
    if is_docs_build() {
        println!("cargo:warning=Skipping C++ compilation for documentation build");
        create_dummy_artifacts()?;
        return Ok(());
    }

    let artifacts = build_cpp()?;
    build_ffi(&artifacts)?;

    Ok(())
}

/// Check if we're building for documentation
fn is_docs_build() -> bool {
    // Check for docs-only feature
    if cfg!(feature = "docs-only") {
        return true;
    }
    
    // Check for docs.rs environment
    if std::env::var("DOCS_RS").is_ok() {
        return true;
    }
    
    // Check for rustdoc
    if std::env::var("RUSTDOC_RUNNING").is_ok() {
        return true;
    }
    
    // Check if we're in a cargo doc build
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "release" {
            if let Ok(_) = std::env::var("CARGO_CFG_DOC") {
                return true;
            }
        }
    }
    
    false
}

/// Create dummy artifacts for documentation builds
fn create_dummy_artifacts() -> Result<()> {
    // For documentation builds, we don't need to create any artifacts
    // Just add rerun-if-changed for source files so cargo knows about them
    let hh_pattern = format!("include/**/*.h");
    if let Ok(paths) = glob::glob(&hh_pattern) {
        for path in paths.flatten() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    let cc_pattern = format!("src/**/*.cc");
    if let Ok(paths) = glob::glob(&cc_pattern) {
        for path in paths.flatten() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    let rs_pattern = format!("src/**/*.rs");
    if let Ok(paths) = glob::glob(&rs_pattern) {
        for path in paths.flatten() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    
    Ok(())
}

fn build_cpp() -> Result<Artifacts> {
    let artifacts = Build::new()
        .build();
    artifacts.print_cargo_metadata();
    Ok(artifacts)
}

fn build_ffi(artifacts: &Artifacts) -> Result<()> {
    let hh_pattern = format!("include/**/*.h");
    let _hdrs = glob::glob(&hh_pattern)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|path| {println!("cargo:rerun-if-changed={}", path.display()); path})
        .collect::<Vec<_>>();

    let cc_pattern = format!("src/**/*.cc");
    let cc_srcs = glob::glob(&cc_pattern)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|path| {println!("cargo:rerun-if-changed={}", path.display()); path})
        .collect::<Vec<_>>();

    let rs_excludes = [
        "src/lib.rs",
        "src/common/mod.rs",
    ].into_iter().map(|s| Path::new(s)).collect::<Vec<_>>();

    let rs_pattern = format!("src/**/*.rs");
    let rs_srcs = glob::glob(&rs_pattern)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|path| {println!("cargo:rerun-if-changed={}", path.display()); path})
        .filter(|path| !rs_excludes.contains(&path.as_path()))
        .collect::<Vec<_>>();

    cxx_build::bridges(rs_srcs)
        .include(artifacts.include_dir())
        .flag("-Wno-missing-requires")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-class-memaccess")
        .flag("-Wno-return-type")
        .flag("-Wno-sign-compare")
        .files(cc_srcs)
        .std("c++17")
        .compile("cel-cxx");

    println!("cargo:rustc-link-lib=static=cel-cxx");
    Ok(())
}
