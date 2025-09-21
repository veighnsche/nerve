#![allow(clippy::module_name_repetitions)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use diffy::{apply as apply_patch, Patch};
use sha2::{Digest, Sha256};

/// Name of the apply module (kept for parity with other primitives).
#[must_use]
pub const fn module_name() -> &'static str {
    "apply"
}

/// Strategy controlling how diffs are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyStrategy {
    /// Apply the diff and write the new contents to disk.
    Write,
    /// Validate the diff without writing.
    DryRun,
    /// Apply the diff while keeping a backup of the pre-image.
    WriteBackup { backup_suffix: &'static str },
}

impl Default for ApplyStrategy {
    fn default() -> Self {
        Self::Write
    }
}

/// Options for applying a diff to a file.
#[derive(Debug, Clone)]
pub struct ApplyOptions {
    pub path: PathBuf,
    pub diff: String,
    pub strategy: ApplyStrategy,
    pub checksum: Option<String>,
}

impl ApplyOptions {
    #[must_use]
    pub fn new(path: PathBuf, diff: String) -> Self {
        Self {
            path,
            diff,
            strategy: ApplyStrategy::Write,
            checksum: None,
        }
    }
}

/// Outcome of a diff application request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyOutcome {
    pub status: ApplyStatus,
    pub hunks_applied: usize,
    pub warnings: Vec<String>,
}

impl ApplyOutcome {
    #[must_use]
    pub fn new(status: ApplyStatus, hunks_applied: usize) -> Self {
        Self {
            status,
            hunks_applied,
            warnings: Vec::new(),
        }
    }
}

/// High-level status codes surfaced to callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyStatus {
    Applied,
    Noop,
}

/// Errors raised while attempting to apply a diff.
#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    #[error("nrv.apply: unimplemented: {message}")]
    Unimplemented { message: &'static str },
    #[error("nrv.apply: io failure during {operation} on {path}: {source}")]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("nrv.apply: invalid diff: {message}")]
    InvalidDiff { message: String },
    #[error("nrv.apply: checksum mismatch for {path}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },
    #[error("nrv.apply: file contains invalid utf-8: {path}")]
    InvalidUtf8 { path: PathBuf },
}

/// Applies a unified diff to the specified file.
pub fn diff(options: ApplyOptions) -> Result<ApplyOutcome, ApplyError> {
    let ApplyOptions {
        path,
        diff,
        strategy,
        checksum,
    } = options;

    let (original_bytes, existed) = match fs::read(&path) {
        Ok(bytes) => (bytes, true),
        Err(err) if err.kind() == io::ErrorKind::NotFound => (Vec::new(), false),
        Err(source) => {
            return Err(ApplyError::Io {
                operation: "read",
                path: path.clone(),
                source,
            });
        }
    };

    if let Some(expected) = checksum {
        let actual = hex::encode(Sha256::digest(&original_bytes));
        if actual != expected {
            return Err(ApplyError::ChecksumMismatch {
                path: path.clone(),
                expected,
                actual,
            });
        }
    }

    let original = String::from_utf8(original_bytes.clone())
        .map_err(|_| ApplyError::InvalidUtf8 { path: path.clone() })?;

    let patch = Patch::from_str(&diff).map_err(|err| ApplyError::InvalidDiff {
        message: err.to_string(),
    })?;

    let patched = apply_patch(&original, &patch).map_err(|err| ApplyError::InvalidDiff {
        message: err.to_string(),
    })?;

    let status = if patched == original {
        ApplyStatus::Noop
    } else {
        ApplyStatus::Applied
    };

    let hunks = patch.hunks().len();

    if matches!(strategy, ApplyStrategy::DryRun) {
        return Ok(ApplyOutcome {
            status,
            hunks_applied: hunks,
            warnings: Vec::new(),
        });
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ApplyError::Io {
            operation: "create_dir_all",
            path: parent.to_path_buf(),
            source,
        })?;
    }

    if let ApplyStrategy::WriteBackup { backup_suffix } = strategy {
        if existed {
            let backup = append_suffix(&path, backup_suffix);
            fs::write(&backup, &original_bytes).map_err(|source| ApplyError::Io {
                operation: "backup",
                path: backup,
                source,
            })?;
        }
    }

    fs::write(&path, patched.as_bytes()).map_err(|source| ApplyError::Io {
        operation: "write",
        path: path.clone(),
        source,
    })?;

    Ok(ApplyOutcome {
        status,
        hunks_applied: hunks,
        warnings: Vec::new(),
    })
}

fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path
        .file_name()
        .and_then(|n| n.to_str())
        .map_or_else(|| path.display().to_string(), str::to_owned);
    name.push_str(suffix);
    if let Some(parent) = path.parent() {
        parent.join(name)
    } else {
        PathBuf::from(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn apply_new_file() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        let patch_text = r"--- /dev/null
+++ b/note.txt
@@ -0,0 +1 @@
+hello world
";

        let outcome = super::diff(ApplyOptions {
            path: file_path.clone(),
            diff: patch_text.into(),
            strategy: ApplyStrategy::Write,
            checksum: None,
        })
        .expect("apply");

        assert_eq!(outcome.status, ApplyStatus::Applied);
        assert_eq!(outcome.hunks_applied, 1);

        let contents = fs::read_to_string(file_path).expect("read");
        assert_eq!(contents, "hello world\n");
    }

    #[test]
    fn apply_dry_run_does_not_write() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        let patch_text = r"--- /dev/null
+++ b/note.txt
@@ -0,0 +1 @@
+hello world
";

        let outcome = super::diff(ApplyOptions {
            path: file_path.clone(),
            diff: patch_text.into(),
            strategy: ApplyStrategy::DryRun,
            checksum: None,
        })
        .expect("apply");

        assert_eq!(outcome.status, ApplyStatus::Applied);
        assert!(!file_path.exists());
    }

    #[test]
    fn checksum_mismatch_errors() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        fs::write(&file_path, b"hello\n").expect("write");

        let result = super::diff(ApplyOptions {
            path: file_path.clone(),
            diff: r"--- a/note.txt
+++ b/note.txt
@@ -1 +1 @@
-hello
+world
"
            .into(),
            strategy: ApplyStrategy::Write,
            checksum: Some("deadbeef".into()),
        });

        assert!(matches!(result, Err(ApplyError::ChecksumMismatch { .. })));
    }

    #[test]
    fn write_backup_captures_previous_contents() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        fs::write(&file_path, b"hello\n").expect("write");

        let patch_text = r"--- a/note.txt
+++ b/note.txt
@@ -1 +1 @@
-hello
+world
";

        super::diff(ApplyOptions {
            path: file_path.clone(),
            diff: patch_text.into(),
            strategy: ApplyStrategy::WriteBackup {
                backup_suffix: ".bak",
            },
            checksum: None,
        })
        .expect("apply");

        let backup_path = file_path.with_file_name("note.txt.bak");
        let backup = fs::read_to_string(backup_path).expect("backup");
        assert_eq!(backup, "hello\n");

        let updated = fs::read_to_string(file_path).expect("updated");
        assert_eq!(updated, "world\n");
    }
}
