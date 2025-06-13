use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};


pub fn create_from<N, D, I, S>(name: N, dir: D, srcs: I) -> Result<PathBuf, String>
where
    N: AsRef<str>,
    D: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<Path>,
{
    let filename = format!("lib{}.a", name.as_ref());

    let mut command = Command::new("ar");
    command.current_dir(dir.as_ref());
    command.arg("-M");
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    let verbose_error = match command.spawn() {
        Ok(mut child) => {
            if let Some(stdin) = child.stdin.take() {
                if let Err(e) = feed_ar_input(stdin, srcs, &filename) {
                    let _ = child.kill();
                    format!("prepare ar input failed: {:?}", e)
                } else {
                    match child.wait() {
                        Ok(status) => {
                            if status.success() {
                                return Ok(dir.as_ref().join(filename));
                            }
                            let mut stderr = String::new();
                            child.stderr.take().unwrap().read_to_string(&mut stderr).unwrap();
                            format!(
                                "'{exe}' reported failure with {status}: {stderr}",
                                exe = command.get_program().to_string_lossy(),
                                stderr = stderr,
                            )
                        }
                        Err(failed) => {
                            format!(
                                "Could not run '{exe}', because {failed}",
                                exe = command.get_program().to_string_lossy()
                            )
                        }
                    }
                }
            } else {
                let _ = child.kill();
                format!(
                    "child has no stdin"
                )
            }
        }
        Err(failed) => match failed.kind() {
            std::io::ErrorKind::NotFound => format!(
                "Command '{exe}' not found.",
                exe = command.get_program().to_string_lossy(),
            ),
            _ => format!(
                "Could not run '{exe}', because {failed}",
                exe = command.get_program().to_string_lossy(),
            )
        }
    };

    let desc = "ar -M";
    println!("cargo:warning={desc}: {verbose_error}");
    Err(format!(
        "Error {desc}:
{verbose_error}
Command failed: {command:?}"
    ))
}

fn feed_ar_input<W, I, S>(mut stdin: W, srcs: I, filename: &str) -> std::io::Result<()>
where
    W: std::io::Write,
    I: IntoIterator<Item = S>,
    S: AsRef<Path>,
{
    std::writeln!(stdin, "create {}", filename)?;
    for src in srcs.into_iter() {
        //println!("addlib {}", src.as_ref().as_os_str().to_string_lossy());
        std::writeln!(stdin, "addlib {}", src.as_ref().as_os_str().to_string_lossy())?;
    }
    std::writeln!(stdin, "save")?;
    std::writeln!(stdin, "end")?;
    stdin.flush()?;
    drop(stdin);
    Ok(())
}
