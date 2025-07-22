pub mod bazel;
//mod ar;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use bazel::Bazel;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BAZEL_MINIMAL_VERSION: &str = "8.0.0";
const BAZEL_MAXIMAL_VERSION: &str = "8.99.99";
const BAZEL_DOWNLOAD_VERSION: Option<&str> = Some("8.3.1");

/// Supported target triples - must match platform definitions in build/BUILD.bazel
const SUPPORTED_TARGETS: &[&str] = &[
    // Linux
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "armv7-unknown-linux-gnueabi",
    "i686-unknown-linux-gnu",
    //"x86_64-unknown-linux-musl",
    //"aarch64-unknown-linux-musl",
    //"armv7-unknown-linux-musleabihf",
    //"i686-unknown-linux-musl",
    // Android
    "aarch64-linux-android",
    "armv7-linux-androideabi",
    "x86_64-linux-android",
    "i686-linux-android",
    // Apple macOS
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "arm64e-apple-darwin",
    // Apple iOS
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "x86_64-apple-ios",
    "arm64e-apple-ios",
    // Apple tvOS
    "aarch64-apple-tvos",
    "aarch64-apple-tvos-sim",
    "x86_64-apple-tvos",
    // Apple watchOS
    "aarch64-apple-watchos",
    "aarch64-apple-watchos-sim",
    "x86_64-apple-watchos-sim",
    "arm64_32-apple-watchos",
    "armv7k-apple-watchos",
    // Apple visionOS
    "aarch64-apple-visionos",
    "aarch64-apple-visionos-sim",
    // Windows
    "x86_64-pc-windows-msvc",
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

fn is_windows(target: &str) -> bool {
    target.contains("windows")
}

fn target_config(target: &str) -> Option<&'static str> {
    if target.contains("apple") {
        if target.contains("darwin") {
            return Some("macos");
        } else if target.contains("ios") {
            return Some("ios");
        }
    }
    if target.contains("windows") {
        return Some("msvc");
    }
    None
}

fn work_dir(target: &str) -> Result<PathBuf> {
    if !is_supported_target(target) {
        return Err(anyhow::anyhow!(
            "Unsupported target for cel-build-utils: {}. See SUPPORTED_TARGETS in cel-build-utils/src/lib.rs",
            target
        ));
    }

    let dir = if is_windows(target) {
        "cel-windows"
    } else {
        "cel"
    };

    Ok(Path::new(env!("CARGO_MANIFEST_DIR")).join(dir))
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

        if let Some(config) = target_config(target) {
            build_command.arg(format!("--config={config}"));
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

        // Copy include files
        let include_mapping = vec![
            ("bazel-cel/external/cel-cpp+", "."),
            ("bazel-cel/external/abseil-cpp+/absl", "absl"),
            ("bazel-cel/external/protobuf+/src/google", "google"),
            (
                "bazel-bin/external/cel-spec+/proto/cel/expr/_virtual_includes/checked_proto/cel",
                "cel",
            ),
            (
                "bazel-bin/external/cel-spec+/proto/cel/expr/_virtual_includes/value_proto/cel",
                "cel",
            ),
            (
                "bazel-bin/external/cel-spec+/proto/cel/expr/_virtual_includes/syntax_proto/cel",
                "cel",
            ),
        ];

        for (f, t) in include_mapping {
            #[cfg(windows)]
            let f = f.replace("bazel-cel", "bazel-cel-windows");
            #[cfg(windows)]
            let f = f.replace("/", "\\");
            #[cfg(windows)]
            let t = t.replace("/", "\\");

            let f = work_dir.join(f);
            let t = install_include_dir.join(t);
            cp_r(&f, &t)?;
        }

        let libcel_name = if is_windows(target) {
            "cel.lib"
        } else {
            "libcel.a"
        };
        std::fs::copy(
            work_dir.join("bazel-bin").join(libcel_name),
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

    #[track_caller]
    fn run_command(&self, mut command: Command, desc: &str) -> Result<Vec<u8>, String> {
        //println!("running {:?}", command);
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
    //println!("copying dir {src:?} -> {dst:?}");
    for f in fs::read_dir(src).map_err(|e| anyhow!("{}: {e}", src.display()))? {
        let f = match f {
            Ok(f) => f,
            _ => continue,
        };
        fs::create_dir_all(dst).map_err(|e| anyhow!("failed to create dir {dst:?}: {e}"))?;

        let file_name = f.file_name();
        let mut path = f.path();

        // Skip git metadata as it's been known to cause issues (#26) and
        // otherwise shouldn't be required
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
            //fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
            cp_r(&f.path(), &dst)?;
        } else {
            //println!("copying file {path:?} -> {dst:?}");
            let _ = fs::remove_file(&dst);
            if let Err(e) = fs::copy(&path, &dst) {
                //println!("failed to copy {path:?} -> {dst:?}");
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
