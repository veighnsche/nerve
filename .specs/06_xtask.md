# xtask (xtask)

## Purpose
- Provides a workspace automation binary for formatting, linting, and distribution chores.
- Currently a placeholder that prints `xtask: placeholder (fmt, lint, dist)`.

## Design Notes
- Intentionally minimal while the Rust workspace stabilizes.
- Should remain a thin orchestrator around `cargo`, `npm`, and project scripts.
- Add subcommands (`fmt`, `lint`, `dist`) once the underlying tooling is settled.

## Usage
```bash
cargo run -p xtask
```

## Roadmap
- Implement typed argument parsing (e.g., `clap` or `bpaf`).
- Centralize formatting and lint entry points to keep CI fast and explicit.
