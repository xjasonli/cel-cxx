use std::{ffi::OsStr, io::BufRead as _, path::{Path, PathBuf}, process::Command};
use anyhow::Result;
use anyhow::Context as _;

pub struct Bazel {
    pub path: PathBuf,
    pub arch: String,
    pub os: String,
    pub mode: String,
    pub wdir: Option<PathBuf>,
}

impl Default for Bazel {
    fn default() -> Self {
        Self {
            path: PathBuf::from("bazelisk"),
            arch: std::env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            os: std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
            mode: mode_from_profile(),
            wdir: None,
        }
    }
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
        minimal_version: &str,
        download_dir: &Path,
        download_version: Option<&str>,
    ) -> Result<Self> {
        let path = bazel_path(minimal_version, download_dir, download_version)?;
        Ok(Self {
            path: path,
            ..Default::default()
        })
    }

    pub fn with_work_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.wdir = Some(dir.as_ref().to_owned());
        self
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
        cmd
            //.arg(format!("--cpu={}", self.arch))
            .arg(format!("--compilation_mode={}", self.mode))
    }

    pub fn bin_dir(&self) -> Result<String> {
        let cpu = match (self.os.as_str(), self.arch.as_str()) {
            ("linux", "x86_64") => "k8",
            ("windows", "x86_64") => "x64_windows",
            ("macos", "x86_64") => "darwin",
            _ => return Err(anyhow::anyhow!("Unsupported architecture")),
        };
        Ok(format!("{}-{}", cpu, self.mode))
    }

    pub fn parse_output_libraries(&self, stdout: &[u8]) -> Result<Vec<PathBuf>> {
        let prefix = format!("bazel-out/{}", self.bin_dir()?);

        Ok(
            stdout.lines()
                .filter_map(|line| line.ok())
                .filter_map(|line| {
                    let s = Path::new(line.trim());
                    if !s.starts_with(&prefix) {
                        return None;
                    }
                    if s.extension() != Some(OsStr::new("a")) {
                        return None;
                    }
                    Some(s.to_owned())
                })
                .collect::<Vec<_>>()
        )
    }
    //pub fn parse_output_pb_headers(&self, stdout: &[u8]) -> Vec<PathBuf> {
    //    let prefix = format!("bazel-out/{}", self.bin_dir());
    //    stdout.lines()
    //        .filter_map(|line| line.ok())
    //        .filter_map(|line| {
    //            let s = Path::new(line.trim());
    //            if !s.starts_with(&prefix) {
    //                return None;
    //            }
    //            if !s.file_name().to_string_lossy().ends_with(".pb.h") {
    //                return None;
    //            }
    //            Some(s.to_owned())
    //        })
    //        .collect::<Vec<_>>()
    //}

    pub fn parse_targets(&self, stdout: &[u8]) -> Vec<String> {
        stdout.lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| line.rsplitn(2, " ").last().map(|s| s.to_owned()))
            .collect::<Vec<_>>()
    }
}

fn bazel_path(minimal_version: &str, download_dir: &Path, download_version: Option<&str>) -> Result<PathBuf, anyhow::Error> {
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
fn bazel_command_version<S: AsRef<std::ffi::OsStr>>(cmd: S) -> Result<semver::Version, anyhow::Error> {
    let output = std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .context("Failed to run bazel")?;
    let s = String::from_utf8(output.stdout).context("Failed to parse bazel version")?;
    let re = regex::Regex::new(r"bazel ([^\s]+)")?;
    let caps = re.captures(&s).ok_or(anyhow::anyhow!("Invalid bazel version"))?;
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
        Ok(format!("bazel.exe"))
    } else {
        Ok(format!("bazel"))
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
        .verify(
            downloader::verify::with_digest::<sha2::Sha256>(decode_hex(&sha256)?)
        );

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
        Ok(Self { tmp: Some(tmp), dst: Some(dst) })
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

/*
#[tokio::main(flavor = "current_thread")]
async fn install_bazelisk<P: AsRef<Path>>(dir: P) -> Result<PathBuf, anyhow::Error> {
    let releases = octocrab::instance()
        .repos("bazelbuild", "bazelisk")
        .releases()
        .list()
        .per_page(10)
        .page(1u32)
        .send()
        .await
        .context("Failed to get releases")?
        .items;
    let release = releases.first().ok_or(anyhow::anyhow!("No release found"))?;

    let pattern = {
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "windows",
            _ => return Err(anyhow::anyhow!("Unsupported OS")),
        };
        let arch = match std::env::consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" => "arm64",
            _ => return Err(anyhow::anyhow!("Unsupported architecture")),
        };
        format!("bazelisk-{}-{}", os, arch)
    };
    let asset = release.assets
        .iter()
        .find(|asset| asset.name.starts_with(&pattern))
        .ok_or(anyhow::anyhow!("No asset found"))?;

    let mut stream = octocrab::instance()
        .repos("bazelbuild", "bazelisk")
        .release_assets()
        .stream(asset.id.0)
        .await
        .context("Failed to stream asset")?
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
    //let mut reader = stream.into_async_read();

    let (dst_path, tmp_path) = {
        let mut dst = dir.as_ref().join("bazelisk");
        if std::env::consts::OS == "windows" {
            dst = dst.with_extension("exe");
        }
        let tmp = dst.with_extension("tmp");
        (dst, tmp)
    };
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&tmp_path)
        .context("Failed to open file")?;

    while let Some(chunk) = stream.try_next().await.context("Failed to read asset")? {
        use std::io::Write as _;
        file.write_all(&chunk)?;
    }

    std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))
        .context("Failed to set permissions")?;
    std::fs::rename(&tmp_path, &dst_path)
        .context("Failed to rename file")?;

    Ok(dst_path)
}

*/