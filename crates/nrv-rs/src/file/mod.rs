#![allow(clippy::module_name_repetitions)]

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Name of the `nrv.file` module for scaffolding checks.
#[must_use]
pub const fn module_name() -> &'static str {
    "file"
}

/// Supported write strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteStrategy {
    Overwrite,
    Append,
    Create,
    OverwriteWithBackup { backup_suffix: &'static str },
}

impl Default for WriteStrategy {
    fn default() -> Self {
        Self::Overwrite
    }
}

/// Encoding hints for the file engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileEncoding {
    Utf8,
    Binary,
}

impl Default for FileEncoding {
    fn default() -> Self {
        Self::Utf8
    }
}

/// Options supplied to `write`.
#[derive(Debug, Clone)]
pub struct WriteOptions {
    pub path: PathBuf,
    pub content: String,
    pub strategy: WriteStrategy,
    pub encoding: FileEncoding,
    pub create_dirs: bool,
}

impl WriteOptions {
    #[must_use]
    pub fn new(path: PathBuf, content: String) -> Self {
        Self {
            path,
            content,
            strategy: WriteStrategy::Overwrite,
            encoding: FileEncoding::Utf8,
            create_dirs: false,
        }
    }
}

/// Result of a write operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteOutcome {
    pub bytes_written: usize,
    pub created: bool,
    pub warnings: Vec<String>,
}

impl WriteOutcome {
    #[must_use]
    pub fn new(bytes_written: usize, created: bool) -> Self {
        Self {
            bytes_written,
            created,
            warnings: Vec::new(),
        }
    }
}

/// Metadata returned by `stat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStat {
    pub size: u64,
    pub is_file: bool,
    pub is_symlink: bool,
}

/// Content returned by `read`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileContent {
    pub text: String,
}

/// Errors raised by file operations.
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("nrv.file: io failure during {operation} on {path}: {source}")]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("nrv.file: operation requires utf8 content: {path}")]
    InvalidUtf8 { path: PathBuf },
}

pub fn read(path: PathBuf) -> Result<FileContent, FileError> {
    let bytes = fs::read(&path).map_err(|source| FileError::Io {
        operation: "read",
        path: path.clone(),
        source,
    })?;

    let text = String::from_utf8(bytes).map_err(|_| FileError::InvalidUtf8 { path: path.clone() })?;

    Ok(FileContent { text })
}

pub fn write(mut options: WriteOptions) -> Result<WriteOutcome, FileError> {
    let path = options.path.clone();
    if options.create_dirs {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| FileError::Io {
                operation: "create_dirs",
                path: parent.to_path_buf(),
                source,
            })?;
        }
    }

    if let WriteStrategy::OverwriteWithBackup { backup_suffix } = options.strategy {
        if Path::new(&path).exists() {
            let backup_path = append_suffix(&path, backup_suffix);
            fs::copy(&path, &backup_path).map_err(|source| FileError::Io {
                operation: "backup",
                path: backup_path,
                source,
            })?;
        }
    }

    let existed_before = path.exists();
    let mut open_options = OpenOptions::new();
    open_options.write(true);

    match options.strategy {
        WriteStrategy::Overwrite | WriteStrategy::OverwriteWithBackup { .. } => {
            open_options.create(true).truncate(true);
        }
        WriteStrategy::Append => {
            open_options.create(true).append(true);
        }
        WriteStrategy::Create => {
            open_options.create_new(true);
        }
    }

    let mut file = open_options.open(&path).map_err(|source| FileError::Io {
        operation: "open",
        path: path.clone(),
        source,
    })?;

    let bytes = options.content.into_bytes();
    file.write_all(&bytes).map_err(|source| FileError::Io {
        operation: "write",
        path: path.clone(),
        source,
    })?;
    file.flush().map_err(|source| FileError::Io {
        operation: "flush",
        path: path.clone(),
        source,
    })?;

    Ok(WriteOutcome {
        bytes_written: bytes.len(),
        created: !existed_before,
        warnings: Vec::new(),
    })
}

pub fn stat(path: PathBuf) -> Result<FileStat, FileError> {
    let metadata = fs::symlink_metadata(&path).map_err(|source| FileError::Io {
        operation: "stat",
        path: path.clone(),
        source,
    })?;

    Ok(FileStat {
        size: metadata.len(),
        is_file: metadata.file_type().is_file(),
        is_symlink: metadata.file_type().is_symlink(),
    })
}

pub fn remove(path: PathBuf) -> Result<(), FileError> {
    fs::remove_file(&path).map_err(|source| FileError::Io {
        operation: "remove",
        path,
        source,
    })
}

pub fn exists(path: PathBuf) -> Result<bool, FileError> {
    Ok(path.exists())
}

fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut stem = path
        .file_name()
        .and_then(|name| name.to_str())
        .map_or_else(|| path.display().to_string(), str::to_owned);
    stem.push_str(suffix);
    if let Some(parent) = path.parent() {
        parent.join(stem)
    } else {
        PathBuf::from(stem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn write_and_read_roundtrip() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("sample.txt");

        let options = WriteOptions {
            path: file_path.clone(),
            content: "hello world".to_string(),
            strategy: WriteStrategy::Overwrite,
            encoding: FileEncoding::Utf8,
            create_dirs: false,
        };

        let outcome = write(options).expect("write");
        assert!(outcome.created);
        assert_eq!(outcome.bytes_written, 11);

        let content = read(file_path.clone()).expect("read");
        assert_eq!(content.text, "hello world");

        let stats = stat(file_path.clone()).expect("stat");
        assert_eq!(stats.size, 11);
        assert!(stats.is_file);
        assert!(!stats.is_symlink);

        assert!(exists(file_path.clone()).expect("exists"));

        remove(file_path.clone()).expect("remove");
        assert!(!exists(file_path).expect("exists"));
    }

    #[test]
    fn backup_suffix_is_written_when_present() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("data.txt");

        write(WriteOptions {
            path: file_path.clone(),
            content: "alpha".into(),
            strategy: WriteStrategy::Overwrite,
            encoding: FileEncoding::Utf8,
            create_dirs: true,
        })
        .expect("initial write");

        write(WriteOptions {
            path: file_path.clone(),
            content: "beta".into(),
            strategy: WriteStrategy::OverwriteWithBackup {
                backup_suffix: ".bak",
            },
            encoding: FileEncoding::Utf8,
            create_dirs: false,
        })
        .expect("overwrite with backup");

        let backup_path = file_path.with_file_name("data.txt.bak");
        let backup_contents = read(backup_path).expect("backup");
        assert_eq!(backup_contents.text, "alpha");
    }
}
