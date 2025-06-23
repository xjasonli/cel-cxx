use std::path::Path;
use anyhow::Result;
use cel_build_utils::{Artifacts, Build};

fn main() -> Result<()> {
    if skip_building() {
        // Skip C++ compilation when building docs
        println!("cargo:warning=Skipping C++ compilation for documentation build");
        return Ok(());
    }

    let artifacts = build_cpp()?;
    build_ffi(&artifacts)?;

    Ok(())
}

/// Check if we're building for documentation
fn skip_building() -> bool {
    // Check for docs.rs environment
    if std::env::var("DOCS_RS").is_ok() {
        return true;
    }
    false
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
