#![allow(clippy::module_name_repetitions)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Name of the `nrv.dir` module for scaffolding checks.
#[must_use]
pub const fn module_name() -> &'static str {
    "dir"
}

/// Options for listing directory contents.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub include_files: bool,
    pub include_directories: bool,
    pub follow_symlinks: bool,
}

/// Directory entry returned by listings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_symlink: bool,
}

/// Result metadata for ensure operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirOutcome {
    pub created: bool,
}

/// Errors emitted by directory helpers.
#[derive(Debug, thiserror::Error)]
pub enum DirError {
    #[error("nrv.dir: io failure during {operation} on {path}: {source}")]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("nrv.dir: directory not empty: {path}")]
    NotEmpty { path: PathBuf },
    #[error("nrv.dir: expected directory but found different file type: {path}")]
    NotDirectory { path: PathBuf },
}

pub fn list(path: PathBuf, options: ListOptions) -> Result<Vec<DirEntry>, DirError> {
    let include_files = options.include_files || (!options.include_files && !options.include_directories);
    let include_dirs = options.include_directories || (!options.include_files && !options.include_directories);

    let entries = fs::read_dir(&path).map_err(|source| DirError::Io {
        operation: "read_dir",
        path: path.clone(),
        source,
    })?;

    let mut results = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| DirError::Io {
            operation: "read_dir_entry",
            path: path.clone(),
            source,
        })?;

        let file_type = if options.follow_symlinks {
            entry.metadata().map(|meta| meta.file_type())
        } else {
            entry.file_type()
        }
        .map_err(|source| DirError::Io {
            operation: "metadata",
            path: entry.path(),
            source,
        })?;

        let is_dir = file_type.is_dir();
        let is_file = file_type.is_file();
        let is_symlink = file_type.is_symlink();

        if (is_dir && !include_dirs) || (is_file && !include_files) {
            continue;
        }

        results.push(DirEntry {
            path: entry.path(),
            is_dir,
            is_symlink,
        });
    }

    Ok(results)
}

pub fn ensure(path: PathBuf) -> Result<DirOutcome, DirError> {
    if path.exists() {
        if !Path::new(&path).is_dir() {
            return Err(DirError::NotDirectory { path });
        }
        return Ok(DirOutcome { created: false });
    }

    fs::create_dir_all(&path).map_err(|source| DirError::Io {
        operation: "create_dir_all",
        path: path.clone(),
        source,
    })?;

    Ok(DirOutcome { created: true })
}

pub fn remove_empty(path: PathBuf) -> Result<(), DirError> {
    match fs::remove_dir(&path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::DirectoryNotEmpty => {
            Err(DirError::NotEmpty { path })
        }
        Err(source) => Err(DirError::Io {
            operation: "remove_dir",
            path,
            source,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn ensure_and_list_directory() {
        let dir = tempdir().expect("tempdir");
        let nested = dir.path().join("nested");

        let outcome = ensure(nested.clone()).expect("ensure");
        assert!(outcome.created);

        let file_path = nested.join("file.txt");
        fs::write(&file_path, b"data").expect("write");

        let entries = list(nested.clone(), ListOptions::default()).expect("list");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, file_path);
        assert!(!entries[0].is_dir);

        remove_empty(nested.clone()).expect_err("not empty");
        fs::remove_file(file_path).expect("remove");
        remove_empty(nested).expect("remove_empty");
    }
}
