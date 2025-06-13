pub mod bazel;
//mod ar;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::anyhow;
use anyhow::Context;
use bazel::Bazel;
use anyhow::Result;

const BAZEL_MINIMAL_VERSION: &str = "8.0.0";
const BAZEL_DOWNLOAD_VERSION: Option<&str> = Some("8.2.1");

pub fn cel_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("build")
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Build {
    out_dir: Option<PathBuf>,
    target: Option<String>,
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
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("cel-build")),
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
                println!("cargo:warning=libcel: failed to build cel-cpp from source");
                eprintln!("\n\n\n{e}\n\n\n");
                std::process::exit(1)
            }
        }
    }

    pub fn try_build(&mut self) -> Result<Artifacts> {
        let target = self.target.as_ref()
            .context("TARGET dir not set")?;
        let out_dir = self.out_dir.as_ref()
            .context("OUT_DIR not set")?;
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)
                .context(format!("failed to create out_dir: {}", out_dir.display()))?;
        }
        let cel_dir = cel_dir();
        let install_dir = out_dir.join("install");

        let install_library_dir = install_dir.join("lib");
        let install_include_dir = install_dir.join("include");
        let libs = vec![
            "cel".to_owned()
        ];

        let install_library_file = install_library_dir.join("libcel.a");

        match install_library_file.try_exists() {
            Ok(true) => {
                return Ok(Artifacts {
                    lib_dir: install_library_dir,
                    include_dir: install_include_dir,
                    libs,
                    target: target.to_owned(),
                })
            }
            _ => (),
        }

        let bazel = Bazel::new(BAZEL_MINIMAL_VERSION, &out_dir, BAZEL_DOWNLOAD_VERSION)?
            .with_work_dir(&cel_dir);


        let _ = self.run_command(
            bazel.build([":cel"]),
            "building cel",
        ).map_err(|e| anyhow!(e))?;

        if install_dir.exists() {
            fs::remove_dir_all(&install_dir)
                .context(format!("failed to remove install_dir: {}", install_dir.display()))?;
        }

        // Create install directory
        fs::create_dir_all(&install_dir)
            .context(format!("failed to create install_dir: {}", install_dir.display()))?;

        // Create include directory
        fs::create_dir(&install_include_dir)
            .context(format!("failed to create install_include_dir: {}", install_include_dir.display()))?;
        // Create library directory
        fs::create_dir(&install_library_dir)
            .context(format!("failed to create install_library_dir: {}", install_library_dir.display()))?;

        // Copy include files
        let include_mapping = vec![
            (
                "bazel-out/../../../external/cel-cpp+",
                "."
            ),
            (
                "bazel-out/../../../external/abseil-cpp+/absl",
                "absl"
            ),
            (
                "bazel-out/../../../external/protobuf+/src/google",
                "google"
            ),
            (
                "bazel-bin/external/cel-spec+/proto/cel/expr/_virtual_includes/checked_proto/cel",
                "cel"
            ),
            (
                "bazel-bin/external/cel-spec+/proto/cel/expr/_virtual_includes/value_proto/cel",
                "cel"
            ),
            (
                "bazel-bin/external/cel-spec+/proto/cel/expr/_virtual_includes/syntax_proto/cel",
                "cel"
            ),
        ];

        for (f, t) in include_mapping {
            let f = cel_dir.join(f);
            let t = install_include_dir.join(t);
            cp_r(&f, &t)
                .context(format!("failed to copy include file: {}", t.display()))?;
        }

        std::fs::copy(
            &cel_dir.join("bazel-bin").join("libcel.a"),
            &install_library_dir.join("libcel.a"),
        ).context("failed to copy libcel.a")?;

        if std::env::var("PROFILE").unwrap() == "debug" {
            let compile_commands = cel_dir.join("compile_commands.json");
            match compile_commands.try_exists() {
                Ok(true) => (),
                _ => {
                    let _ = self.run_command(
                        bazel.run([":refresh_compile_commands"]),
                        "running cel",
                    ).map_err(|e| anyhow!(e))?;
                }
            }
        }

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
                format!(
                    "'{exe}' reported failure with {status}",
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
    //println!("copying {:?} -> {:?}", src, dst);
    for f in fs::read_dir(src).map_err(|e| anyhow!("{}: {e}", src.display()))? {
        let f = match f {
            Ok(f) => f,
            _ => continue,
        };
        fs::create_dir_all(dst)?;

        let file_name = f.file_name();
        let mut path = f.path();

        // Skip git metadata as it's been known to cause issues (#26) and
        // otherwise shouldn't be required
        if file_name.to_str() == Some(".git") {
            continue;
        }

        let dst = dst.join(file_name);
        let mut ty = f.file_type()?;
        while ty.is_symlink() {
            path = fs::read_link(&f.path())?;
            ty = fs::metadata(&path)?.file_type();
        }

        if ty.is_dir() {
            //fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
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
            println!("cargo:rustc-link-lib=static={}", lib);
        }
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}
