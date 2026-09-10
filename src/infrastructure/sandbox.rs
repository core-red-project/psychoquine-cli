use crate::domain::error::DomainError;
use crate::domain::language::Language;
use crate::infrastructure::detector::{RuntimeInfo, SystemDetector};
use std::fs;
use std::io::Read;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

fn run_with_timeout(
    mut cmd: Command,
    timeout: Duration,
    context_name: &str,
) -> Result<Output, DomainError> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(DomainError::Io)?;

    let mut stdout_handle = child.stdout.take();
    let mut stderr_handle = child.stderr.take();

    let stdout_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(ref mut handle) = stdout_handle {
            let _ = handle.read_to_end(&mut buf);
        }
        buf
    });

    let stderr_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(ref mut handle) = stderr_handle {
            let _ = handle.read_to_end(&mut buf);
        }
        buf
    });

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = stdout_thread.join().unwrap_or_default();
                let stderr = stderr_thread.join().unwrap_or_default();
                return Ok(Output {
                    status,
                    stdout,
                    stderr,
                });
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = stdout_thread.join();
                    let _ = stderr_thread.join();
                    return Err(DomainError::ExecutionTimeout {
                        language: context_name.to_string(),
                        timeout_secs: timeout.as_secs(),
                    });
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
                return Err(DomainError::Io(e));
            }
        }
    }
}

pub struct SandboxRunner;

impl SandboxRunner {
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

    pub fn execute(language: Language, code: &str) -> Result<(String, RuntimeInfo), DomainError> {
        Self::execute_with_timeout(language, code, Self::DEFAULT_TIMEOUT)
    }

    pub fn execute_with_timeout(
        language: Language,
        code: &str,
        timeout: Duration,
    ) -> Result<(String, RuntimeInfo), DomainError> {
        let runtime = SystemDetector::detect_for_language(language);
        if !runtime.available {
            return Err(DomainError::RuntimeUnavailable(language.to_string()));
        }

        let meta = language.metadata();
        let temp_dir = tempfile::tempdir().map_err(DomainError::Io)?;
        let file_path = temp_dir.path().join(format!("quine.{}", meta.extension));
        fs::write(&file_path, code).map_err(DomainError::Io)?;

        let stdout_bytes = match language {
            Language::Python
            | Language::Javascript
            | Language::Bash
            | Language::Perl
            | Language::Php
            | Language::Ruby
            | Language::Lua
            | Language::Ocaml => {
                let mut cmd = Command::new(&runtime.binary);
                cmd.arg(&file_path);
                let out = run_with_timeout(cmd, timeout, &language.to_string())?;

                if !out.status.success() {
                    let err = String::from_utf8_lossy(&out.stderr);
                    return Err(DomainError::ExecutionFailed {
                        language: language.to_string(),
                        details: err.to_string(),
                    });
                }
                out.stdout
            }
            Language::C => {
                let bin_name = if cfg!(windows) { "c_bin.exe" } else { "c_bin" };
                let bin_path = temp_dir.path().join(bin_name);
                let mut comp = Command::new(&runtime.binary);
                comp.arg(&file_path).arg("-o").arg(&bin_path);
                let comp_out = run_with_timeout(comp, timeout, "C (Compiler)")?;

                if !comp_out.status.success() {
                    let err = String::from_utf8_lossy(&comp_out.stderr);
                    return Err(DomainError::CompilationFailed {
                        language: "C".to_string(),
                        details: err.to_string(),
                    });
                }

                let run = Command::new(&bin_path);
                let run_out = run_with_timeout(run, timeout, "C")?;
                if !run_out.status.success() {
                    let err = String::from_utf8_lossy(&run_out.stderr);
                    return Err(DomainError::ExecutionFailed {
                        language: "C".to_string(),
                        details: err.to_string(),
                    });
                }
                run_out.stdout
            }
            Language::Rust => {
                let bin_name = if cfg!(windows) {
                    "rust_bin.exe"
                } else {
                    "rust_bin"
                };
                let bin_path = temp_dir.path().join(bin_name);
                let mut comp = Command::new(&runtime.binary);
                comp.arg(&file_path).arg("-o").arg(&bin_path);
                let comp_out = run_with_timeout(comp, timeout, "Rust (Compiler)")?;

                if !comp_out.status.success() {
                    let err = String::from_utf8_lossy(&comp_out.stderr);
                    return Err(DomainError::CompilationFailed {
                        language: "Rust".to_string(),
                        details: err.to_string(),
                    });
                }

                let run = Command::new(&bin_path);
                let run_out = run_with_timeout(run, timeout, "Rust")?;
                if !run_out.status.success() {
                    let err = String::from_utf8_lossy(&run_out.stderr);
                    return Err(DomainError::ExecutionFailed {
                        language: "Rust".to_string(),
                        details: err.to_string(),
                    });
                }
                run_out.stdout
            }
            Language::Go => {
                let mut cmd = Command::new(&runtime.binary);
                cmd.arg("run").arg(&file_path);
                let out = run_with_timeout(cmd, timeout, "Go")?;

                if !out.status.success() {
                    let err = String::from_utf8_lossy(&out.stderr);
                    return Err(DomainError::ExecutionFailed {
                        language: "Go".to_string(),
                        details: err.to_string(),
                    });
                }
                out.stdout
            }
            _ => {
                let mut cmd = Command::new(&runtime.binary);
                cmd.arg(&file_path);
                let out = run_with_timeout(cmd, timeout, &language.to_string())?;

                if !out.status.success() {
                    let err = String::from_utf8_lossy(&out.stderr);
                    return Err(DomainError::ExecutionFailed {
                        language: language.to_string(),
                        details: err.to_string(),
                    });
                }
                out.stdout
            }
        };

        let output_str = String::from_utf8_lossy(&stdout_bytes).to_string();
        Ok((output_str, runtime))
    }
}
