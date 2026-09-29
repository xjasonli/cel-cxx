//! `TZDIR` must take precedence over every other timezone source.
//!
//! On Windows a named IANA zone resolves through the OS-managed database
//! (cel-cpp's C++20 `<chrono>` fallback) only when cctz's file-based lookup
//! fails. Proving the precedence requires `TZDIR` to be visible to cctz
//! before the first zone is resolved, which in turn requires it to be part
//! of the process environment at startup: cctz on MSVC reads `TZDIR` with
//! `_dupenv_s`, which consults the CRT snapshot taken at process start, so
//! a later in-process `std::env::set_var` (a `SetEnvironmentVariableW`
//! call) is invisible to it. This test therefore re-executes its own test
//! binary as a child process with a fixture zoneinfo tree exported as
//! `TZDIR`, containing a deliberately wrong `America/New_York` (constant
//! UTC). A result matching the crafted data proves the file-based lookup
//! won; the OS database or a genuine zoneinfo tree would return the real
//! zone instead. Under cross-rs the child runs through cargo's runner
//! (see `child_command`), the same way the test binary itself is launched.

use cel_cxx::*;
use std::io::Write as _;

/// Signals the child re-entry of this test: the fixture tree is installed
/// and `TZDIR` was inherited at process start.
const FIXTURE_ENV: &str = "CEL_CXX_TZDIR_TEST_FIXTURE";

/// A minimal v1 TZif file describing a constant UTC zone, in the shape of
/// `/usr/share/zoneinfo/UTC`: one time type with a zero offset, no
/// transitions, and the "UTC" abbreviation.
fn write_constant_utc_tzif(path: &std::path::Path) -> std::io::Result<()> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(b"TZif");
    bytes.push(0); // version 1
    bytes.extend_from_slice(&[0u8; 15]); // reserved
    // isutcnt, isstdcnt, leapcnt, timecnt, typecnt, charcnt
    for count in [0u32, 0, 0, 0, 1, 4] {
        bytes.extend_from_slice(&count.to_be_bytes());
    }
    // One time type: zero offset, not DST, abbreviation index 0.
    bytes.extend_from_slice(&0i32.to_be_bytes());
    bytes.push(0);
    bytes.push(0);
    bytes.extend_from_slice(b"UTC\0");
    std::fs::File::create(path)?.write_all(&bytes)
}

fn new_york_hour() -> Result<Value, Error> {
    let env = Env::builder().declare_variable::<String>("ts")?.build()?;
    // 2023-07-04T15:04:05Z is a Tuesday.
    let activation = Activation::new()
        .bind_variable("ts", "2023-07-04T15:04:05Z".to_string())?;
    let program = env.compile("timestamp(ts).getHours('America/New_York')")?;
    program.evaluate(&activation)
}

#[test]
fn test_tzdir_precedes_os_timezone_database() -> Result<(), Error> {
    if std::env::var_os(FIXTURE_ENV).is_some() {
        // Child re-entry: the fixture tree is installed and `TZDIR` points
        // at it from process start, so cctz resolves the crafted zone.
        // The crafted file says America/New_York is constant UTC (hour 15).
        // The real zone is UTC-4 in July (hour 11) on the OS database and
        // on any genuine zoneinfo tree alike, so only the file-based
        // lookup yields 15.
        println!("child run with TZDIR={}", std::env::var("TZDIR").unwrap_or_default());
        assert_eq!(
            new_york_hour()?,
            Value::Int(15),
            "expected the crafted constant-UTC fixture, not the real zone"
        );
        return Ok(());
    }

    // Parent: install the fixture tree and re-run this test binary as a
    // fresh process with TZDIR set at startup. cctz caches every resolved
    // zone name for the process lifetime, so the child must be the first
    // process to resolve the zone — the parent never evaluates a zone.
    let dir = std::env::temp_dir().join(format!("cel-cxx-tzdir-{}", std::process::id()));
    std::fs::create_dir_all(dir.join("America"))
        .expect("create fixture zoneinfo directory");
    write_constant_utc_tzif(&dir.join("America").join("New_York"))
        .expect("write fixture TZif file");

    let output = child_command()
        .env(FIXTURE_ENV, &dir)
        .env("TZDIR", &dir)
        .output()
        .expect("spawn child test process");

    std::fs::remove_dir_all(&dir).ok();

    assert!(
        output.status.success(),
        "child TZDIR precedence run failed\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    Ok(())
}

/// Command that re-runs this test binary as a fresh process.
///
/// Under cross-rs the binary itself is launched through cargo's runner
/// (qemu-user for foreign architectures, wine for windows-gnu), and a bare
/// `exec` of `current_exe` would only work if the host registers a
/// `binfmt_misc` handler for the target — something stock docker hosts do
/// only for their own emulation set (loongarch64, for one, is missing).
/// Route the child through the same runner: when cargo exports a
/// `CARGO_TARGET_<triple>_RUNNER` environment variable, run the binary
/// under it, exactly as cargo itself does.
fn child_command() -> std::process::Command {
    let exe = std::env::current_exe().expect("resolve current test binary");
    let runner = std::env::vars()
        .find(|(name, _)| name.starts_with("CARGO_TARGET_") && name.ends_with("_RUNNER"))
        .map(|(_, value)| {
            value
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let mut command = if runner.is_empty() {
        std::process::Command::new(&exe)
    } else {
        let mut command = std::process::Command::new(&runner[0]);
        command.args(&runner[1..]).arg(&exe);
        command
    };
    command.args(["test_tzdir_precedes_os_timezone_database", "--exact", "--nocapture"]);
    command
}
