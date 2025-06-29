use anyhow::Context as _;
use anyhow::Result;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

/// Supported target triples - must match BUILD.bazel platform definitions
const SUPPORTED_TARGETS: &[&str] = &[
    // Linux
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    // Windows
    "x86_64-pc-windows-msvc",
    // Android
    "aarch64-linux-android",
    "armv7-linux-androideabi",
    "x86_64-linux-android",
    "i686-linux-android",
    // macOS
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    // iOS
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "x86_64-apple-ios",
];

pub struct Bazel {
    pub path: PathBuf,
    pub mode: String,
    pub wdir: Option<PathBuf>,
    pub target: String,
}

/// Check if target triple is supported
fn is_supported_target(target: &str) -> bool {
    SUPPORTED_TARGETS.contains(&target)
}

fn mode_from_profile() -> String {
    let profile = std::env::var("PROFILE").unwrap();
    let mode = match profile.as_str() {
        "debug" => "fastbuild",
        "release" => "opt",
        _ => "opt",
    };
    mode.to_owned()
}

impl Bazel {
    pub fn new(
        target: String,
        minimal_version: &str,
        download_dir: &Path,
        download_version: Option<&str>,
    ) -> Result<Self> {
        let path = bazel_path(minimal_version, download_dir, download_version)?;

        // Check if target is supported
        if !is_supported_target(&target) {
            return Err(anyhow::anyhow!("Unsupported target: {}", target));
        }

        Ok(Self {
            path,
            mode: mode_from_profile(),
            wdir: None,
            target,
        })
    }

    pub fn with_work_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.wdir = Some(dir.as_ref().to_owned());
        self
    }

    /// Get Bazel platform for the target - must match BUILD.bazel platform definitions
    pub fn target_platform(&self) -> String {
        format!("//:{}", self.target)
    }

    pub fn config(&self) -> Option<String> {
        if self.target.contains("apple") {
            if self.target.contains("darwin") {
                return Some("macos".to_string());
            } else if self.target.contains("ios") {
                return Some("ios".to_string());
            }
            return None;
        }

        None
    }

    pub fn build<I, S>(&self, targets: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = self.command("build");
        cmd.args(targets);
        cmd
    }

    pub fn run<I, S>(&self, targets: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = self.command("run");
        cmd.args(targets);
        cmd
    }

    pub fn cquery<S1: AsRef<OsStr>, S2: AsRef<str>>(&self, expr: S1, output: S2) -> Command {
        let mut cmd = self.command("cquery");
        cmd.arg(expr);
        if !output.as_ref().is_empty() {
            cmd.arg(format!("--output={}", output.as_ref()));
        }
        cmd
    }

    fn command<S: AsRef<OsStr>>(&self, command: S) -> Command {
        let mut cmd = Command::new(&self.path);
        if let Some(work_dir) = &self.wdir {
            cmd.current_dir(work_dir);
        }
        cmd.arg(command);
        self.common_args(&mut cmd);
        cmd
    }

    fn common_args<'a>(&self, cmd: &'a mut Command) -> &'a mut Command {
        cmd.arg(format!("--compilation_mode={}", self.mode));
        cmd.arg(format!("--platforms={}", self.target_platform()));
        if let Some(config) = self.config() {
            cmd.arg(format!("--config={}", config));
        }
        cmd
    }
}

fn bazel_path(
    minimal_version: &str,
    download_dir: &Path,
    download_version: Option<&str>,
) -> Result<PathBuf, anyhow::Error> {
    // detect bazel or bazelisk version
    let minimal_ver = semver::Version::parse(minimal_version)?;
    if let Ok(ver) = bazel_command_version("bazel") {
        if ver >= minimal_ver {
            return Ok(PathBuf::from("bazel"));
        }
    }
    if let Ok(version) = bazel_command_version("bazelisk") {
        if version >= minimal_ver {
            return Ok(PathBuf::from("bazelisk"));
        }
    }
    let path = download_dir.join(bazel_filename()?);
    if let Ok(version) = bazel_command_version(&path) {
        if version >= minimal_ver {
            return Ok(path);
        }
    }

    let version = download_version.unwrap_or(minimal_version);
    download_bazel(download_dir, version)
}

// bazel --version output:
// bazel 8.2.1
fn bazel_command_version<S: AsRef<std::ffi::OsStr>>(
    cmd: S,
) -> Result<semver::Version, anyhow::Error> {
    let output = std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .context("Failed to run bazel")?;
    let s = String::from_utf8(output.stdout).context("Failed to parse bazel version")?;
    let re = regex::Regex::new(r"bazel ([^\s]+)")?;
    let caps = re
        .captures(&s)
        .ok_or(anyhow::anyhow!("Invalid bazel version"))?;
    let version = caps.get(1).unwrap().as_str();
    Ok(semver::Version::parse(version).unwrap())
}

fn bazel_url(version: &str) -> Result<(String, String), anyhow::Error> {
    let os = match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "darwin",
        "linux" => "linux",
        _ => return Err(anyhow::anyhow!("Unsupported host os")),
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "arm64",
        _ => return Err(anyhow::anyhow!("Unsupported host arch")),
    };

    // URL examples:
    // https://github.com/bazelbuild/bazel/releases/download/8.2.1/bazel-8.2.1-darwin-arm64
    // https://github.com/bazelbuild/bazel/releases/download/8.2.1/bazel-8.2.1-darwin-x86_64
    // https://github.com/bazelbuild/bazel/releases/download/8.2.1/bazel-8.2.1-linux-arm64
    // https://github.com/bazelbuild/bazel/releases/download/8.2.1/bazel-8.2.1-linux-x86_64
    // https://github.com/bazelbuild/bazel/releases/download/8.2.1/bazel-8.2.1-windows-arm64.exe
    // https://github.com/bazelbuild/bazel/releases/download/8.2.1/bazel-8.2.1-windows-x86_64.exe
    let mut url = format!("https://github.com/bazelbuild/bazel/releases/download/{version}/bazel-{version}-{os}-{arch}");
    if os == "windows" {
        url += ".exe";
    }
    let sha256_url = format!("{}.sha256", url);
    Ok((url, sha256_url))
}

fn bazel_filename() -> Result<String, anyhow::Error> {
    let os = match std::env::consts::OS {
        "windows" => "windows",
        "macos" => "darwin",
        "linux" => "linux",
        _ => return Err(anyhow::anyhow!("Unsupported host os")),
    };
    if os == "windows" {
        Ok("bazel.exe".to_string())
    } else {
        Ok("bazel".to_string())
    }
}

fn parse_sha256(s: &str) -> Result<String, anyhow::Error> {
    let mut parts = s.split_whitespace();
    let hash = parts.next().ok_or(anyhow::anyhow!("Invalid sha256"))?;
    let _ = parts.next().ok_or(anyhow::anyhow!("Invalid sha256"))?;
    Ok(hash.to_string().to_lowercase())
}

fn decode_hex(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn download_bazel<P: AsRef<Path>>(dir: P, version: &str) -> Result<PathBuf, anyhow::Error> {
    let filename_dst = bazel_filename()?;
    let filename_tmp = format!("{}.tmp", filename_dst);
    let guard = DownloadGuard::new(
        dir.as_ref().join(&filename_tmp),
        dir.as_ref().join(&filename_dst),
    )?;

    let (url, sha256) = {
        let (url, sha256_url) = bazel_url(version)?;
        let sha256 = parse_sha256(&reqwest::blocking::get(sha256_url)?.text()?)?;
        (url, sha256)
    };

    let dl = downloader::Download::new(&url)
        .file_name(Path::new(&filename_tmp))
        .verify(downloader::verify::with_digest::<sha2::Sha256>(decode_hex(
            &sha256,
        )?));

    let mut downloader = downloader::Downloader::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .download_folder(dir.as_ref())
        .parallel_requests(3)
        .retries(3)
        .timeout(std::time::Duration::from_secs(60))
        .build()?;
    let mut results = downloader.download(&[dl])?;
    if results.is_empty() {
        return Err(anyhow::anyhow!("No file found"));
    }
    let _ = results.remove(0)?;

    guard.complete()
}

struct DownloadGuard {
    tmp: Option<PathBuf>,
    dst: Option<PathBuf>,
}

impl DownloadGuard {
    pub fn new(tmp: PathBuf, dst: PathBuf) -> Result<Self, anyhow::Error> {
        if tmp.exists() {
            std::fs::remove_file(&tmp).context("Failed to remove tmp file")?;
        }
        if dst.exists() {
            std::fs::remove_file(&dst).context("Failed to remove dst file")?;
        }
        Ok(Self {
            tmp: Some(tmp),
            dst: Some(dst),
        })
    }
    pub fn complete(mut self) -> Result<PathBuf, anyhow::Error> {
        let Some(tmp) = self.tmp.take() else {
            return Err(anyhow::anyhow!("No tmp file found"));
        };
        let Some(dst) = self.dst.take() else {
            return Err(anyhow::anyhow!("No dst file found"));
        };

        #[cfg(unix)]
        std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o755))
            .context("Failed to set permissions")?;

        std::fs::rename(&tmp, &dst).context("Failed to rename file")?;
        Ok(dst)
    }
}

impl Drop for DownloadGuard {
    fn drop(&mut self) {
        if let Some(tmp) = self.tmp.take() {
            let _ = std::fs::remove_file(tmp);
        }
    }
}
