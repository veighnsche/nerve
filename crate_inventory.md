# Crate Inventory

Quick reference for all Rust crates currently referenced in the workspace manifests. Use this list to cross-check upstream versions or documentation with an internet-connected assistant.

## Runtime dependencies
- `thiserror` (`nrv-rs`) — error derivations for the guardrail primitives.
- `diffy` (`nrv-rs`) — unified diff parsing and application for `nrv.apply`.
- `hex` (`nrv-rs`) — hex encoding helpers for checksum reporting.
- `sha2` (`nrv-rs`) — SHA-256 hashing for diff checksum enforcement.
- `nrv-ui` (`nrv-rs`) — local path dependency that exposes narration applets.

## Dev-only dependencies
- `cucumber` (`apps/nrv`, `crates/nrv-orch-client`, `crates/nrv-rs`, `crates/nrv-ui`, `xtask`) — BDD scaffolding with the `macros` feature enabled.
- `tokio` (`apps/nrv`, `crates/nrv-orch-client`, `crates/nrv-rs`, `crates/nrv-ui`, `xtask`) — async runtime used by cucumber step definitions (features: `macros`, `rt`, `rt-multi-thread`).
- `tempfile` (`nrv-rs`) — temporary directory utilities for primitive tests.

> Note: Version numbers are taken directly from each crate’s `Cargo.toml`; update here if the manifests change.
