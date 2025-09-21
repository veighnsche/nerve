# WayLaunch (M1) — Project interpreted by GPT‑5 (plan → apply)

This example shows the M1‑style, minimal and explicit workflow:

- You (or a model like “GPT‑5”) produce a JSON plan describing a list of single‑file unified diffs.
- You run a tiny tool that parses the plan and applies the diffs deterministically using `nrv_rs::toolkit`.
- You can dry‑run first, then persist, optionally with backups.

No magic global `nrv` object, no injected functions. Everything is explicit and user‑authored.

## Contents

- `plan.json` — creates a small output folder (`output/`) with two files.
- `plan_update.json` — updates `output/README.md` (apply after `plan.json`).
- `plan_checksum_fail.json` — demonstrates checksum protection failing on purpose.

## How to run

From repo root:

```bash
# 1) Dry run (default)
cargo run -p nrv-rs --example gpt5_waylaunch --

# 2) Persist with backup files (.bak)
cargo run -p nrv-rs --example gpt5_waylaunch -- --write-backup .bak

# 3) Apply a specific plan file
cargo run -p nrv-rs --example gpt5_waylaunch -- --plan .examples/20_WayLaunch_project_interpreted_by_GPT-5/plan_update.json --write

# 4) Demonstrate checksum failure (will error)
cargo run -p nrv-rs --example gpt5_waylaunch -- --plan .examples/20_WayLaunch_project_interpreted_by_GPT-5/plan_checksum_fail.json --write
```

## Plan shape

The plan matches the recommended shape in `.specs/15_llama_orch_toolkit.md`:

```jsonc
{
  "diffs": [
    {
      "path": "relative/path/to/file",
      "checksum": "<optional pre-image sha256>",
      "unified_diff": "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-hello\n+world\n"
    }
  ]
}
```

Notes:
- `path` is what gets written; `---/+++` headers are ignored for file lookup but must be valid unified diff.
- Zero‑context diffs may work in M1 but reduce robustness; prefer context when possible.
- Use `ApplyStrategy::WriteBackup` to keep a safety copy of existing files.
