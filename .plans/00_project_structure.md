# Project Structure Plan: Nerve

Status: Plan
Owner: @vince
Date: 2025-09-20
Version: v0.1

---

## 0) Purpose & Scope

Define and execute a concrete, minimal project structure for this repository using the template as a guide, not as output. This plan is actionable and aligned with:

- Constitution v1/v2 in `/.adr/900_consitution.md` and `/.adr/901_constitution.md` (summarized in `/.specs/00_nerve.md`).
- ADRs 002, 005–010, 011–018 (error model, proofs, UI applets, orchestrator binding, versioning, testing).

Lifecycle target: MVP (docs-first) → v1 (artifact-stable) → v2 (back-compat contract) per ADR-015.

## 1) Repository Topology (Decision: Hybrid Monorepo)

We keep governance (specs/ADRs) and implementations in one repo to enable atomic changes and tight alignment.

Target structure:

```
/docs/                      — user + dev docs (later)
/.specs/                    — specifications (RFC-2119 normalized)
/.adr/                      — architecture decision records
/.plans/                    — planning docs (this file)
/crates/                    — Rust workspace members
  nrv-rs/                   — core Rust library (ADR-008, 006, 007, 009)
  nrv-orch-client/          — orchestrator binding (ADR-014, ADR-011)
/apps/
  nrv/                      — micro-CLI (codegen/sync; NOT a runtime) (ADR-010)
/ts/packages/               — TypeScript packages (optional for M1)
  @nrv/core/                — JS/TS core surface (ADR-010, 008)
  @nrv/ui-kit/              — optional UI Kit (ADR-013)
/proofs/                    — optional, committed Proof Bundles for tests/examples (ADR-012)
/xtask/                     — dev/maintenance tasks (workspace member)
/.github/workflows/         — CI pipelines (lint, build, test)
/scripts/                   — helper scripts (optional)
```

## 2) Rust Workspace Profile

- Root `Cargo.toml` defines a workspace with members: `crates/*`, `apps/*`, `xtask`.
- Toolchain: edition 2021; `rust-toolchain.toml` already present.
- Lints: `#![forbid(unsafe_code)]` where feasible; clippy deny warnings in CI.

## 3) Modules & Responsibilities

- CLI Surface — `apps/nrv` (Rust)
  - MUST implement only codegen/init and `sync-capabilities` (no runtime) per ADR-010.

- Core Libraries — `crates/nrv-rs` (Rust)
  - MUST expose primitives: file/dir/apply (ADR-006), ctx helpers (ADR-007), matcher helpers (ADR-009), proof helpers (ADR-012), UI applets (ADR-013 stubs).
  - MUST follow Anti-Insanity (ADR-008): minimal, explicit, no hidden behavior.

- Orchestrator Binding — `crates/nrv-orch-client` (Rust)
  - MUST define `capabilities/enqueue/stream/cancel` contract and structured errors (ADR-014, ADR-011). Network impl can be deferred; compile with stubs in M1.

- Optional JS/TS — `ts/packages/@nrv/core`, `ts/packages/@nrv/ui-kit`
  - MAY mirror surfaces for Node/Bun/Deno; start as stubs in M2/M3.

### Responsibility Matrix (MUST / MUST NOT)

- apps/nrv (micro-CLI)
  - MUST: generate files, init templates, snapshot capabilities to typed artifacts.
  - MUST NOT: execute LLM calls, implement retries/policies, manage secrets, run user workflows.
  - IO: file writes are explicit and deterministic; guardrails enforced by core primitives.

- crates/nrv-rs (core primitives)
  - MUST: implement file/dir/apply with guardrails; expose ctx mechanics without policy; provide pure matchers; minimal proof capture helpers; tiny UI applets with JSON mode fallback.
  - MUST NOT: perform network calls, manage secrets, inject prompts/policies, auto-capture proofs, perform implicit retries.
  - Errors: return structured error objects with stable codes (ADR-011); panics only for invariants.

- crates/nrv-orch-client (orchestrator binding)
  - MUST: define typed client trait and error taxonomy; support `capabilities/enqueue/stream/cancel` semantics; stream events as data suitable for Proof Bundles.
  - MUST NOT: apply business policy; MUST not hide SSE/network errors.

- ts/packages/@nrv/*
  - MUST: mirror the same boundaries as Rust; UI Kit is opt-in and built atop core applets.
  - MUST NOT: add hidden behaviors or policy.

### Allowed Dependencies (Boundaries)

- `apps/nrv` → may depend on `nrv-rs` for primitives and on `nrv-orch-client` only for capability typing during codegen.
- `crates/nrv-rs` → MUST have zero dependency on orchestrator/network crates.
- `crates/nrv-orch-client` → MUST be independent from `nrv-rs` primitives to keep surfaces decoupled.
- `@nrv/ui-kit` → MAY depend on `@nrv/core`; NEVER vice versa.

### Separation Verification (Acceptance)

- Grep checks show `apps/nrv` contains no network code and no LLM calls.
- `nrv-rs` builds with `--offline` and no net features; no references to HTTP/SSE clients.
- `nrv-orch-client` exposes traits/types only; no runtime network requirement for compile.
- CI denies warnings; clippy confirms no `unwrap()` in library code paths that handle errors.

## 4) Versioning & Releases (ADR-015)

- Independent versioning per crate/package.
- Pre-v2: no compatibility guarantees; v1 is artifact-stable; v2 starts the back-compat contract.
- Proof Bundle schema changes must be called out in release notes.

## 5) Build, Test, CI/CD

- CI (M1): rustfmt check, clippy (deny warnings), cargo check, unit tests (if any).
- CI (M2+): add TS build (tsc) when TS packages exist.
- No hidden retries or heuristics in CI logic (ADR-011, ADR-008).

Testing responsibilities (ADR-016):

- Unit/property tests live next to each crate; contract/BDD tests at workspace level.
- Determinism gates and snapshot tests validate emitted Proof Bundles where applicable.
- CI fails on flakiness; proofs may be stored under `/proofs/` for reproducibility.

## 6) Dev Experience

- Provide `scripts/` helpers for common tasks (format, lint, build). Keep optional and documented.
- Conventional Commits for changelogs.

## 7) Configuration & Security (ADR-017)

- Secrets MUST NOT be managed by core; use env/dotenv. Redact in logs/proofs.
 - Large sensitive blobs MUST NOT be inlined in JSON; use attachments in proofs.

## 8) Documentation

- Root README: link to `/.specs/00_nerve.md` and this plan; state micro-CLI scope (ADR-010).
- Later: /docs with mdBook or alternative.
 - Keep specs authoritative; examples and Proof Bundles are separate from specs.

## 9) Roadmap & Milestones

- M0 (done): Normalize ADRs into `/.specs/00_nerve.md`.
- M1: Scaffold Rust workspace + micro-CLI; compile green; minimal CI.
- M2: Implement minimal primitives (`file/dir/apply`, ctx, matcher) and orch client stubs.
- M3: Proof Bundles helpers and UI applets; optional TS package stubs.
- M4: Docs site and initial releases.

---

## 10) Implementation Checklist (Do This Now)

Perform the following steps in order. Each step has clear acceptance criteria.

1) Create directories

- /.github/workflows/
- /crates/nrv-rs/
- /crates/nrv-orch-client/
- /apps/nrv/
- /xtask/
- /ts/packages/@nrv/core/ (optional, M2)
- /ts/packages/@nrv/ui-kit/ (optional, M3)

Acceptance:

- Directories exist exactly as listed.

2) Add root Cargo workspace

- Create `Cargo.toml` at repo root with `[workspace]` and members: `crates/*`, `apps/*`, `xtask`.

Acceptance:

- `cargo metadata` runs successfully.

3) Scaffold crates (compilable stubs)

- `crates/nrv-rs`: `src/lib.rs` with placeholder modules for `file`, `dir`, `apply`, `ctx`, `match`, `proof`, `ui`.
- `crates/nrv-orch-client`: `src/lib.rs` with types/traits for `capabilities/enqueue/stream/cancel` and error type matching ADR-011 taxonomy.

Acceptance:

- `cargo check` succeeds for both crates.

4) Scaffold micro-CLI

- `apps/nrv`: `src/main.rs` with `--version` and `sync-capabilities` (no-op) following ADR-010 scope.

Acceptance:

- `cargo build -p nrv` succeeds; running `nrv --version` prints a version; `nrv sync-capabilities` exits 0.

5) Minimal CI

- Add `/.github/workflows/ci.yml` with jobs: rustfmt check, clippy (deny warnings), cargo check. TS build can be added later.

Acceptance:

- CI runs on push/PR and passes on a clean checkout.

6) Optional TS skeleton (defer to M2)

- `ts/packages/@nrv/core`: minimal `package.json`, `tsconfig.json`, `src/index.ts` with stub exports.

Acceptance:

- `tsc -p ts/packages/@nrv/core` succeeds (when added).

7) Docs touch-ups (M1)

- Update README to point to specs and this plan and state CLI scope.

Acceptance:

- README present and accurate.

---

## 11) Risks & Mitigations

- Monorepo contention and CI time
  - Mitigation: path-based CI filters, split jobs (lint → check → build).
- Scope creep into CLI runtime
  - Mitigation: enforce ADR-010; CLI only codegen/sync; no runtime behavior.
- Hidden complexity
  - Mitigation: ADR-008 Anti-Insanity; prefer deletion and smaller surfaces.
