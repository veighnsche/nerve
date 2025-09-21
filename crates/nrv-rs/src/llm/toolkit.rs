#![allow(clippy::module_name_repetitions)]

use std::path::PathBuf;

use crate::apply::{ApplyError, ApplyOptions, ApplyOutcome, ApplyStrategy};

/// Minimal representation of a single-file unified diff to apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldDiff {
    pub path: PathBuf,
    pub checksum: Option<String>,
    /// Unified diff text; MUST represent a single-file patch.
    pub diff: String,
}

/// A plan produced by an LLM that the caller wishes to apply deterministically.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApplyPlan {
    pub diffs: Vec<ScaffoldDiff>,
}

/// Applies all diffs in the provided plan using the chosen strategy.
///
/// - Order is deterministic (plan order).
/// - Each diff is applied independently; the first failure aborts the operation.
/// - Use `ApplyStrategy::DryRun` to validate without touching disk.
pub fn apply_plan(plan: &ApplyPlan, strategy: ApplyStrategy) -> Result<Vec<ApplyOutcome>, ApplyError> {
    let mut outcomes = Vec::with_capacity(plan.diffs.len());
    for entry in &plan.diffs {
        let opts = ApplyOptions {
            path: entry.path.clone(),
            diff: entry.diff.clone(),
            strategy,
            checksum: entry.checksum.clone(),
        };
        let outcome = crate::apply::diff(opts)?;
        outcomes.push(outcome);
    }
    Ok(outcomes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn apply_plan_new_file() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        let patch_text = r"--- /dev/null
+++ b/note.txt
@@ -0,0 +1 @@
+hello world
";

        let plan = ApplyPlan {
            diffs: vec![ScaffoldDiff {
                path: file_path.clone(),
                checksum: None,
                diff: patch_text.into(),
            }],
        };

        let outcomes = apply_plan(&plan, ApplyStrategy::Write).expect("apply");
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].hunks_applied, 1);

        let contents = fs::read_to_string(file_path).expect("read");
        assert_eq!(contents, "hello world\n");
    }

    #[test]
    fn apply_plan_checksum_mismatch_errors() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        fs::write(&file_path, b"hello\n").expect("write");

        // Compute actual checksum to ensure mismatch with a different value
        let actual = hex::encode(Sha256::digest(b"hello\n"));
        assert_eq!(actual.len(), 64);

        let patch_text = r"--- a/note.txt
+++ b/note.txt
@@ -1 +1 @@
-hello
+world
";

        let plan = ApplyPlan {
            diffs: vec![ScaffoldDiff {
                path: file_path.clone(),
                checksum: Some("deadbeef".into()), // wrong checksum on purpose
                diff: patch_text.into(),
            }],
        };

        let result = apply_plan(&plan, ApplyStrategy::Write);
        assert!(matches!(result, Err(ApplyError::ChecksumMismatch { .. })));
    }

    #[test]
    fn apply_plan_dry_run_does_not_write() {
        let dir = tempdir().expect("tempdir");
        let file_path = dir.path().join("note.txt");
        let patch_text = r"--- /dev/null
+++ b/note.txt
@@ -0,0 +1 @@
+hello world
";

        let plan = ApplyPlan {
            diffs: vec![ScaffoldDiff {
                path: file_path.clone(),
                checksum: None,
                diff: patch_text.into(),
            }],
        };

        let outcomes = apply_plan(&plan, ApplyStrategy::DryRun).expect("apply");
        assert_eq!(outcomes.len(), 1);
        assert!(!file_path.exists());
    }
}
