pub mod bazel;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use bazel::Bazel;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Match cel-cpp's Bazel baseline, including module compatibility resolution
// for its current Abseil and protobuf dependencies.
const BAZEL_MINIMAL_VERSION: &str = "8.7.0";
const BAZEL_MAXIMAL_VERSION: &str = "8.99.99";
const BAZEL_DOWNLOAD_VERSION: Option<&str> = Some("8.7.0");

/// Supported target triples - must match platform definitions in cel/BUILD.bazel
const SUPPORTED_TARGETS: &[&str] = &[
    // Linux (glibc)
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "armv7-unknown-linux-gnueabi",
    "armv7-unknown-linux-gnueabihf",
    "i686-unknown-linux-gnu",
    "i586-unknown-linux-gnu",
    "powerpc64le-unknown-linux-gnu",
    "riscv64gc-unknown-linux-gnu",
    "loongarch64-unknown-linux-gnu",
    // Linux (musl). Built with the same cpu/os platform constraints as the
    // gnu targets: cross-rs selects its toolchain through root-module
    // registration priority (see cel/MODULE.bazel), while native musl hosts
    // (e.g. Alpine) resolve their own auto-configured toolchain.
    // Cross-rs' musl images ship GCC with C++ support; glibc hosts without
    // a musl C++ compiler cannot build these targets. The loongarch64 and
    // riscv64gc musl targets use the stock cross-rs images (crosstool-NG
    // 14.2); the rest go through the docker/cross rust-musl-cross images
    // referenced by Cross.toml.
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "armv7-unknown-linux-musleabi",
    "armv7-unknown-linux-musleabihf",
    "i686-unknown-linux-musl",
    "i586-unknown-linux-musl",
    "loongarch64-unknown-linux-musl",
    "riscv64gc-unknown-linux-musl",
    // Android
    "aarch64-linux-android",
    "armv7-linux-androideabi",
    "x86_64-linux-android",
    "i686-linux-android",
    // Apple macOS
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    // Apple iOS
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "x86_64-apple-ios",
    // Apple Mac Catalyst
    "aarch64-apple-ios-macabi",
    "x86_64-apple-ios-macabi",
    // Apple tvOS
    "aarch64-apple-tvos",
    "aarch64-apple-tvos-sim",
    // Apple watchOS
    "aarch64-apple-watchos",
    "aarch64-apple-watchos-sim",
    // Apple visionOS
    "aarch64-apple-visionos",
    "aarch64-apple-visionos-sim",
    // Windows (MSVC)
    "x86_64-pc-windows-msvc",
    // Windows (MinGW-w64). Requires a gnu-targeting gcc on PATH (e.g. an
    // MSYS2 UCRT64 shell); the Bazel build resolves it through the
    // conditional mingw extension registered in cel/MODULE.bazel. The
    // docker + cross-rs route also works via rules_cross_rs.
    "x86_64-pc-windows-gnu",
    // WebAssembly
    "wasm32-unknown-emscripten",
];

/// Check if target triple is supported for non-cross builds
fn is_supported_target(target: &str) -> bool {
    SUPPORTED_TARGETS.contains(&target)
}

fn is_cross_rs() -> bool {
    env::var("CROSS_SYSROOT").is_ok() && env::var("CROSS_TOOLCHAIN_PREFIX").is_ok()
}

/// Bazel --config section for the target triple (see cel/.bazelrc). The
/// Apple sections expand "gcc" and set platform-specific deployment
/// defaults; MinGW uses its own "mingw" section.
fn target_config(target: &str) -> &'static str {
    if target.contains("apple") {
        if target.contains("darwin") {
            return "macos";
        } else if target.ends_with("ios-macabi") {
            return "catalyst";
        } else if target.contains("ios") {
            return "ios";
        } else if target.contains("tvos") {
            return "tvos";
        } else if target.contains("watchos") {
            return "watchos";
        } else if target.contains("visionos") {
            return "visionos";
        }
    }
    if target.contains("windows") {
        if target.contains("gnu") {
            return "mingw";
        }
        return "msvc";
    }
    "gcc"
}

fn work_dir(target: &str) -> Result<PathBuf> {
    if !is_supported_target(target) {
        return Err(anyhow::anyhow!(
            "Unsupported target for cel-build-utils: {}. See SUPPORTED_TARGETS in cel-build-utils/src/lib.rs",
            target
        ));
    }

    Ok(Path::new(env!("CARGO_MANIFEST_DIR")).join("cel"))
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Build {
    out_dir: Option<PathBuf>,
    target: Option<String>,
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    libs: Vec<String>,
    #[allow(dead_code)]
    target: String,
}

impl Build {
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("cel")),
            target: env::var("TARGET").ok(),
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn target(&mut self, target: &str) -> &mut Build {
        self.target = Some(target.to_string());
        self
    }

    /// Exits the process on failure. Use `try_build` to handle the error.
    pub fn build(&mut self) -> Artifacts {
        match self.try_build() {
            Ok(a) => a,
            Err(e) => {
                println!("cargo:warning=libcel: failed to build cel-cpp from source\n{e}");
                std::process::exit(1)
            }
        }
    }

    pub fn try_build(&mut self) -> Result<Artifacts> {
        // Short-circuit: use pre-built artifacts if CEL_PREBUILT_DIR is set.
        // This skips the entire Bazel build, useful for environments where
        // Bazel cannot download dependencies (e.g. corporate networks).
        //
        // Expected layout:
        //   $CEL_PREBUILT_DIR/lib/libcel.a
        //   $CEL_PREBUILT_DIR/include/  (cel-cpp + abseil + protobuf headers)
        if let Ok(prebuilt) = env::var("CEL_PREBUILT_DIR") {
            return self.use_prebuilt(&prebuilt);
        }

        let target = self.target.as_ref().context("TARGET dir not set")?;
        let out_dir = self.out_dir.as_ref().context("OUT_DIR not set")?;
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)
                .context(format!("failed_to create out_dir: {}", out_dir.display()))?;
        }
        let work_dir = work_dir(target)?;
        let install_dir = out_dir.join("install");

        let install_library_dir = install_dir.join("lib");
        let install_include_dir = install_dir.join("include");
        let libs = vec!["cel".to_owned()];

        let mut bazel = Bazel::new(
            target.clone(),
            BAZEL_MINIMAL_VERSION,
            BAZEL_MAXIMAL_VERSION,
            out_dir,
            BAZEL_DOWNLOAD_VERSION,
        )?
        .with_work_dir(&work_dir);

        if is_cross_rs() {
            bazel = bazel.with_option("--output_user_root=/tmp/bazel");
        }

        let mut build_command = bazel.build(["//:cel"]);

        build_command.arg(format!("--platforms=//:{target}"));

        build_command.arg(format!("--config={}", target_config(target)));

        // Match deployment overrides used by rustc and cc-rs. Without
        // these, libcel can require a newer OS than the Rust executable.
        if target.contains("apple") {
            let (variable, flag) = match target_config(target) {
                "macos" => ("MACOSX_DEPLOYMENT_TARGET", "macos_minimum_os"),
                "ios" | "catalyst" => ("IPHONEOS_DEPLOYMENT_TARGET", "ios_minimum_os"),
                "tvos" => ("TVOS_DEPLOYMENT_TARGET", "tvos_minimum_os"),
                "watchos" => ("WATCHOS_DEPLOYMENT_TARGET", "watchos_minimum_os"),
                "visionos" => ("XROS_DEPLOYMENT_TARGET", "minimum_os_version"),
                _ => unreachable!(),
            };
            println!("cargo:rerun-if-env-changed={variable}");
            if let Ok(version) = env::var(variable) {
                build_command.arg(format!("--{flag}={version}"));
            }
        }

        // Android environment vars leak into Bzlmod repository setup and break
        // builds on hosts with an Android SDK installed:
        // - rules_android (auto-injected into every Bzlmod build) reads
        //   ANDROID_HOME and fails analysis on hosts whose SDK only has a
        //   minor-version platform dir like "android-37.0". The SDK is not
        //   needed either way: libcel compiles C++ through the NDK toolchain
        //   only, so neutralize the SDK vars for every target.
        // - cel/MODULE.bazel's conditional_android_ndk extension reads
        //   ANDROID_NDK_HOME, so Android targets keep it; the Windows
        //   path-separator bug in rules_android_ndk's symlink setup is
        //   normalized in android_conditional.bzl.
        // Neutralize for repository rules only (--repo_env does not touch
        // the process env, so cargo-ndk still sees the NDK for linking).
        build_command.arg("--repo_env=ANDROID_HOME=");
        build_command.arg("--repo_env=ANDROID_SDK_ROOT=");
        if !target.contains("android") {
            build_command.arg("--repo_env=ANDROID_NDK_HOME=");
        }

        // The Debian/Ubuntu arm-linux-gnueabi cross compilers default to
        // -march=armv5t (armel) while the Rust target is armv7: 4-byte
        // atomics then go through __sync_* runtime calls, which collide
        // with Rust's compiler_builtins at final-link time. Match the
        // target arch (the same -march cross-rs itself passes to CMake
        // projects). Hard-float variants add +fp: their compilers default
        // to armv7-a+fp, and a plain -march=armv7-a drops the FPU, which
        // -mfloat-abi=hard then rejects. --copt applies to C and C++
        // compiles, target side only, so the x86_64 exec tools are
        // unaffected.
        if target.starts_with("armv7") {
            let suffix = if target.contains("gnueabihf") || target.contains("musleabihf") {
                "+fp"
            } else {
                ""
            };
            build_command.arg(format!("--copt=-march=armv7-a{suffix}"));
        }

        self.run_command(build_command, "building cel")
            .map_err(|e| anyhow!(e))?;

        if install_dir.exists() {
            fs::remove_dir_all(&install_dir).context(format!(
                "failed to remove install_dir: {}",
                install_dir.display()
            ))?;
        }

        // Create install directory
        fs::create_dir_all(&install_dir).context(format!(
            "failed to create install_dir: {}",
            install_dir.display()
        ))?;

        // Create include directory
        fs::create_dir(&install_include_dir).context(format!(
            "failed to create install_include_dir: {}",
            install_include_dir.display()
        ))?;
        // Create library directory
        fs::create_dir(&install_library_dir).context(format!(
            "failed to create install_library_dir: {}",
            install_library_dir.display()
        ))?;

        // Resolve this invocation's real output locations from Bazel. The
        // bazel-* convenience symlinks in the workspace cannot be trusted
        // here: cross-rs containers cannot refresh them through the docker
        // mount (they would keep pointing at the host's output base, and
        // the copies below would read a foreign tree), and concurrent
        // host builds rewrite them underneath.
        let info_cmd = bazel.command("info");
        let info_out = self
            .run_command(info_cmd, "querying bazel output locations")
            .map_err(|e| anyhow!(e))?;
        let info = String::from_utf8_lossy(&info_out);
        let execution_root = info
            .lines()
            .find_map(|l| l.strip_prefix("execution_root: "))
            .context("bazel info did not report execution_root")?;
        let bazel_bin = info
            .lines()
            .find_map(|l| l.strip_prefix("bazel-bin: "))
            .context("bazel info did not report bazel-bin")?;
        let execution_root = PathBuf::from(execution_root);
        let bazel_bin = PathBuf::from(bazel_bin);

        // Copy include files
        // NOTE: cel-spec gencode comes from _virtual_imports -- protobuf 35.x's
        // proto_library emits .pb.h/.pb.cc there. The legacy _virtual_includes
        // tree only holds stale gencode from older protobuf builds, which fails
        // compilation with gencode/runtime version mismatch errors.
        let include_mapping = vec![
            (execution_root.join("external/cel-cpp+"), "."),
            (execution_root.join("external/abseil-cpp+/absl"), "absl"),
            (execution_root.join("external/protobuf+/src/google"), "google"),
            // Protobuf's public parse_context.h includes utf8_validity.h.
            (
                execution_root.join("external/protobuf+/third_party/utf8_range/utf8_validity.h"),
                "utf8_validity.h",
            ),
            (
                execution_root.join("external/protobuf+/third_party/utf8_range/utf8_range.h"),
                "utf8_range.h",
            ),
            (
                bazel_bin.join("external/cel-spec+/proto/cel/expr/_virtual_imports/checked_proto/cel"),
                "cel",
            ),
            (
                bazel_bin.join("external/cel-spec+/proto/cel/expr/_virtual_imports/value_proto/cel"),
                "cel",
            ),
            (
                bazel_bin.join("external/cel-spec+/proto/cel/expr/_virtual_imports/syntax_proto/cel"),
                "cel",
            ),
        ];

        for (f, t) in include_mapping {
            // Forward slashes are fine on Windows: std::path/std::fs accept
            // them, and Bazel prints forward-slash paths on every platform.
            let t = install_include_dir.join(t);
            cp_r(&f, &t)?;
        }

        // Static library naming follows the resolved toolchain, not the
        // target OS: MSVC's auto-configured toolchain renames archives to
        // name.lib, while every gcc-family toolchain (mingw for
        // windows-gnu, the NDK for android) keeps the default
        // lib<name>.a naming, which the GNU linker's -lcel resolves.
        let libcel_name = if target.contains("msvc") {
            "cel.lib"
        } else {
            "libcel.a"
        };
        std::fs::copy(
            bazel_bin.join(libcel_name),
            install_library_dir.join(libcel_name),
        )
        .context(format!("failed to copy {libcel_name}"))?;

        Ok(Artifacts {
            lib_dir: install_library_dir,
            include_dir: install_include_dir,
            libs,
            target: target.to_owned(),
        })
    }

    /// Use pre-built CEL artifacts instead of building from source.
    ///
    /// Copies the pre-built `lib/` and `include/` trees into `$OUT_DIR/cel/install/`
    /// so the rest of the build (cxx bridge compilation) finds them in the
    /// same location as a normal Bazel build.
    fn use_prebuilt(&self, prebuilt_dir: &str) -> Result<Artifacts> {
        let dir = PathBuf::from(prebuilt_dir);
        let src_lib = dir.join("lib");
        let src_include = dir.join("include");

        let libcel = if cfg!(windows) { "cel.lib" } else { "libcel.a" };
        anyhow::ensure!(
            src_lib.join(libcel).exists(),
            "CEL_PREBUILT_DIR={prebuilt_dir}: {libcel} not found in {}",
            src_lib.display()
        );
        anyhow::ensure!(
            src_include.exists(),
            "CEL_PREBUILT_DIR={prebuilt_dir}: include/ directory not found"
        );

        // Copy into $OUT_DIR/cel/install/ so cxx_build finds the headers
        let out_dir = self.out_dir.as_ref().context("OUT_DIR not set")?;
        let install_dir = out_dir.join("install");
        if install_dir.exists() {
            fs::remove_dir_all(&install_dir)?;
        }
        let install_lib = install_dir.join("lib");
        let install_inc = install_dir.join("include");
        fs::create_dir_all(&install_lib)?;
        cp_r(&src_include, &install_inc)?;
        fs::copy(src_lib.join(libcel), install_lib.join(libcel))?;

        println!(
            "cargo:warning=Using pre-built CEL from {} (skip Bazel)",
            dir.display()
        );

        Ok(Artifacts {
            lib_dir: install_lib,
            include_dir: install_inc,
            libs: vec!["cel".to_owned()],
            target: self
                .target
                .clone()
                .unwrap_or_else(|| "unknown".to_owned()),
        })
    }

    #[track_caller]
    fn run_command(&self, mut command: Command, desc: &str) -> Result<Vec<u8>, String> {
        let output = command.output();

        let verbose_error = match output {
            Ok(output) => {
                let status = output.status;
                if status.success() {
                    return Ok(output.stdout);
                }
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!(
                    "'{exe}' reported failure with {status}\nstdout: {stdout}\nstderr: {stderr}",
                    exe = command.get_program().to_string_lossy()
                )
            }
            Err(failed) => match failed.kind() {
                std::io::ErrorKind::NotFound => format!(
                    "Command '{exe}' not found. Is {exe} installed?",
                    exe = command.get_program().to_string_lossy()
                ),
                _ => format!(
                    "Could not run '{exe}', because {failed}",
                    exe = command.get_program().to_string_lossy()
                ),
            },
        };

        println!("cargo:warning={desc}: {verbose_error}");
        Err(format!(
            "Error {desc}:
    {verbose_error}
    Command failed: {command:?}"
        ))
    }
}

fn cp_r(src: &Path, dst: &Path) -> Result<()> {
    if src.is_file() {
        fs::copy(src, dst).with_context(|| {
            format!("failed to copy '{}' to '{}'", src.display(), dst.display())
        })?;
        return Ok(());
    }
    for f in fs::read_dir(src).map_err(|e| anyhow!("{}: {e}", src.display()))? {
        let f = match f {
            Ok(f) => f,
            _ => continue,
        };
        fs::create_dir_all(dst).map_err(|e| anyhow!("failed to create dir {dst:?}: {e}"))?;

        let file_name = f.file_name();
        let mut path = f.path();

        // Git metadata is not needed in installed build artifacts.
        if file_name.to_str() == Some(".git") {
            continue;
        }

        let dst = dst.join(file_name);
        let mut ty = f.file_type().map_err(|e| anyhow!("failed to read file type {f:?}: {e}"))?;
        while ty.is_symlink() {
            let link_path = fs::read_link(f.path()).map_err(|e| anyhow!("failed to read link {f:?}: {e}"))?;
            if link_path.is_relative() {
                path = f.path().parent().unwrap().join(link_path);
            } else {
                path = link_path;
            }
            ty = fs::metadata(&path).map_err(|e| anyhow!("failed to read metadata {path:?}: {e}"))?.file_type();
        }

        if ty.is_dir() {
            cp_r(&f.path(), &dst)?;
        } else {
            let _ = fs::remove_file(&dst);
            if let Err(e) = fs::copy(&path, &dst) {
                return Err(anyhow!(
                    "failed to copy '{}' to '{}': {e}",
                    path.display(),
                    dst.display()
                ));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn sanitize_sh(path: &Path) -> String {
    if !cfg!(windows) {
        return path.to_string_lossy().into_owned();
    }
    let path = path.to_string_lossy().replace("\\", "/");
    return change_drive(&path).unwrap_or(path);

    fn change_drive(s: &str) -> Option<String> {
        let mut ch = s.chars();
        let drive = ch.next().unwrap_or('C');
        if ch.next() != Some(':') {
            return None;
        }
        if ch.next() != Some('/') {
            return None;
        }
        Some(format!("/{}/{}", drive, &s[drive.len_utf8() + 2..]))
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn libs(&self) -> &[String] {
        &self.libs
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        for lib in self.libs.iter() {
            println!("cargo:rustc-link-lib=static={lib}");
        }
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}
