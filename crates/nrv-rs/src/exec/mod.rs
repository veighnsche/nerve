#![allow(clippy::module_name_repetitions)]

use std::collections::BTreeMap;
use std::io::{self, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Name of the `nrv.exec` module for scaffolding checks.
#[must_use]
pub const fn module_name() -> &'static str {
    "exec"
}

/// Options for running external commands.
#[derive(Debug, Clone)]
pub struct CommandOptions {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: BTreeMap<String, String>,
    pub timeout: Option<Duration>,
    pub use_shell: bool,
}

impl CommandOptions {
    #[must_use]
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            cwd: None,
            env: BTreeMap::new(),
            timeout: None,
            use_shell: false,
        }
    }
}

/// Result of a command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecOutcome {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
}

/// Errors emitted by command execution.
#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("nrv.exec: io failure during {operation}: {source}")]
    Io {
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("nrv.exec: failed to join {stream} reader thread")]
    StreamJoin { stream: &'static str },
    #[error("nrv.exec: command timed out after {duration:?}")]
    Timeout {
        duration: Duration,
        stdout: String,
        stderr: String,
    },
}

pub fn run(options: CommandOptions) -> Result<ExecOutcome, ExecError> {
    let start = Instant::now();
    let mut command = build_command(&options)?;

    if let Some(cwd) = &options.cwd {
        command.current_dir(cwd);
    }
    command.envs(&options.env);
    command.stdin(Stdio::null());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|source| ExecError::Io {
        operation: "spawn",
        source,
    })?;

    let stdout_handle = child.stdout.take().map(reader_thread).ok_or(ExecError::Io {
        operation: "capture_stdout",
        source: io::Error::new(io::ErrorKind::Other, "stdout handle missing"),
    })?;

    let stderr_handle = child.stderr.take().map(reader_thread).ok_or(ExecError::Io {
        operation: "capture_stderr",
        source: io::Error::new(io::ErrorKind::Other, "stderr handle missing"),
    })?;

    let mut timed_out = None;
    let exit_status = loop {
        match child.try_wait().map_err(|source| ExecError::Io {
            operation: "try_wait",
            source,
        })? {
            Some(status) => break status,
            None => {
                if let Some(timeout) = options.timeout {
                    if start.elapsed() >= timeout {
                        timed_out = Some(timeout);
                        child.kill().map_err(|source| ExecError::Io {
                            operation: "kill",
                            source,
                        })?;
                        let status = child.wait().map_err(|source| ExecError::Io {
                            operation: "wait",
                            source,
                        })?;
                        break status;
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        }
    };

    let stdout = stdout_handle
        .join()
        .map_err(|_| ExecError::StreamJoin { stream: "stdout" })?
        .map_err(|source| ExecError::Io {
            operation: "read_stdout",
            source,
        })?;
    let stderr = stderr_handle
        .join()
        .map_err(|_| ExecError::StreamJoin { stream: "stderr" })?
        .map_err(|source| ExecError::Io {
            operation: "read_stderr",
            source,
        })?;

    if let Some(duration) = timed_out {
        return Err(ExecError::Timeout {
            duration,
            stdout,
            stderr,
        });
    }

    Ok(ExecOutcome {
        status: exit_status.code(),
        stdout,
        stderr,
        duration_ms: start.elapsed().as_millis(),
    })
}

fn reader_thread(mut reader: impl Read + Send + 'static) -> thread::JoinHandle<io::Result<String>> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    })
}

#[cfg(unix)]
fn command_line_join(parts: &[String]) -> String {
    parts
        .iter()
        .map(|part| {
            if part.is_empty()
                || part
                    .chars()
                    .any(|c| c.is_whitespace() || matches!(c, '\'' | '"' | '$' | '`' | '\\'))
            {
                format!("'{}'", part.replace('\'', "'\\''"))
            } else {
                part.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(windows)]
fn command_line_join(parts: &[String]) -> String {
    parts
        .iter()
        .map(|part| {
            if part.chars().any(|c| c.is_whitespace() || matches!(c, '"' | '^' | '&' | '|' | '<' | '>')) {
                format!("\"{}\"", part.replace('"', "\\\""))
            } else {
                part.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_command(options: &CommandOptions) -> Result<Command, ExecError> {
    if options.use_shell {
        let mut parts = Vec::with_capacity(options.args.len() + 1);
        parts.push(options.program.clone());
        parts.extend(options.args.iter().cloned());
        let command_str = command_line_join(&parts);
        #[cfg(windows)]
        let mut command = {
            let mut cmd = Command::new("cmd");
            cmd.arg("/C").arg(&command_str);
            cmd
        };
        #[cfg(not(windows))]
        let mut command = {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(&command_str);
            cmd
        };
        Ok(command)
    } else {
        let mut command = Command::new(&options.program);
        command.args(&options.args);
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn runs_basic_command() {
        let mut options = CommandOptions::new("echo");
        options.args.push("nrv".into());
        options.use_shell = true;

        let outcome = run(options).expect("exec");
        assert_eq!(outcome.status, Some(0));
        assert!(outcome.stdout.to_lowercase().contains("nrv"));
    }

    #[cfg(unix)]
    #[test]
    fn times_out_long_running_command() {
        let mut options = CommandOptions::new("sleep 2");
        options.use_shell = true;
        options.timeout = Some(Duration::from_millis(200));

        let err = run(options).expect_err("timeout");
        match err {
            ExecError::Timeout { duration, .. } => {
                assert_eq!(duration, Duration::from_millis(200));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
