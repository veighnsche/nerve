# Project Structure Plan: \<Project / Suite Name>

Status: Draft
Owner: @\<team/maintainer>
Date: YYYY-MM-DD
Version: v0.x

---

## 0) Purpose & Scope

* Why this structure exists (mono-repo, multi-repo, or hybrid) and what lifecycle it covers (MVP → v1 → maintenance).
* MUST link to relevant Specs/ADRs (naming, versioning, release policy).
* SHOULD define target ecosystems (CLI binaries, libraries, services, plugins) within a Rust workspace.

## 1) Repository Topology

* Decision: **Mono-repo | Multi-repo | Hybrid**.
* MUST justify choice (coordination, CI fan-out, atomic changes, release train).
* High-level map:

```
/docs/           — user + dev docs
/.specs/         — specifications (RFC-2119) + proofs
/.adr/           — architecture decision records
/crates/         — Rust libraries (workspace members)
/apps/           — binaries (workspace members)
/xtask/          — dev/maintenance tasks (workspace member)
/tools/          — devtools, generators, scripts
/.github/        — workflows, issue templates
/scripts/        — CLI helpers (format, test, release)
```

## 2) Rust Workspace Profile

* Layout: **Cargo workspace** with `resolver = "2"`.
* Structure:

  * `/crates/<name>/` (libraries)
  * `/apps/<bin>/` (binaries)
  * `/xtask/` (automation tasks)
* Build/Test:

  * `cargo check`
  * `cargo test` (unit/integration)
  * `cargo clippy -- -D warnings`
  * `cargo deny check`, `cargo audit`, `cargo outdated` (as applicable)
  * Feature flags documented and tested (off-by-default for risky features).
* Release:

  * SemVer; tag per crate or lockstep (document policy).
  * Optionally `cargo-release` / `cargo-dist`.
  * Docs published on docs.rs (public crates).
* Lint/Format:

  * `rustfmt` (workspace-level), `clippy` (deny warnings).
  * Enforce `#![forbid(unsafe_code)]` where feasible.
* Dependencies:

  * Minimize public deps; avoid unnecessary proc-macros.
  * MSRV documented (toolchain pinned via `rust-toolchain.toml`).
* CI:

  * Matrix {stable, beta} if needed.
  * Caching for `target/`; split jobs: lint → check → test → package.
* Artifacts:

  * `target/` outputs, release bundles; optional SBOM/signatures.

### Workspace Manifest Skeleton

```toml
[workspace]
resolver = "2"
members = ["crates/*", "apps/*", "xtask"]

[workspace.package]
edition = "2021"

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.metadata]
# optional: msrv, dist settings, release tooling config
```

## 3) Module Boundaries & Responsibilities

* Define **domains** (e.g., CLI Surface, Core Engine, Adapters, Integrations).
* Each crate MUST document: purpose, public API, dependency policy, tests, ownership.
* Cross-crate rules:

  * Allowed import graph (e.g., CLI → Core, Adapters → Core; no reverse).
  * Feature flags for optional deps/integrations.
  * Experimental APIs behind `experimental` feature gate.

## 4) Versioning & Releases

* Policy: **SemVer** across crates.
* Mono-repo options: **independent versioning** vs **lockstep** (choose one).
* MUST define changelog automation and provenance/attestations.
* Artifact matrix (OS/arch), signing, optional SBOM.

## 5) Build, Test, CI/CD

* Build graph: workspace-aware (avoid rebuilding unchanged members).
* Test pyramid:

  * **Unit** (per-crate) — MUST.
  * **Integration** (cross-crate) — MUST (e.g., under `/apps` or tests/).
  * **e2e** (binary-level/system) — SHOULD.
  * **Property/Fuzz** — MAY for parsers/critical logic.
* CI stages: lint → build → test → package → release (guarded by tags).
* Flakiness policy and retry budget.

## 6) Dev Experience & Tooling

* Bootstrap: `make bootstrap` or `just bootstrap` (installs rustup toolchain, linters).
* Common scripts: `format`, `lint`, `test`, `build`, `release`, `docs`.
* Editor configs: `.editorconfig`, `rust-toolchain.toml`, VSCode settings.
* Pre-commit hooks; Conventional Commits (optional but encouraged).

## 7) Configuration & Environments

* Config precedence: env vars → config files → CLI flags.
* Profiles: `dev`, `ci`, `release`, `debug`.
* Secrets: templates in `.env.example`; NEVER commit real secrets.

## 8) Observability & QA

* Logging: policy (levels, JSON vs pretty) for binaries; structured logs where applicable.
* Test artifacts: junit xml, coverage, golden fixtures stored in repo CI artifacts.
* Reproducibility: pinned toolchains; hermetic builds when feasible.

## 9) Security & Compliance

* Supply-chain:

  * Lockfiles committed.
  * `cargo-audit`, `cargo-deny` in CI; periodic `cargo update -p <crate>`.
* Release security:

  * Code signing (optional), reproducible builds (documented).
* Licensing:

  * Root `LICENSE`, SPDX headers per crate; third-party notices.

## 10) Documentation

* `/docs` generated with mdBook (or alternative).
* MUST document crate APIs (READMEs), CLI surfaces, configs, examples.
* Diagrams (Mermaid) co-located with docs and kept in git.

## 11) Roadmap & Milestones

* M0 Bootstrap (workspace scaffolding, CI green).
* M1 Core crates compile & pass unit tests.
* M2 Cross-crate integration + basic e2e.
* M3 Release packaging + docs published.
* M4 Maintenance cadence + deprecation policy.

## 12) Risks & Mitigations

* Monorepo contention, feature-flag complexity, toolchain churn.
* Mitigations: clear ownership/CODEOWNERS, API stability windows, ADRs for breaking changes.

## 13) Appendices

* **Make/Justfile** snippets for common tasks.
* **Crate template** for new members.

---

### Appendix A: Crate Template (library)

```
/crates/example/
├─ Cargo.toml
└─ src/
   ├─ lib.rs
   └─ <modules>.rs
```

**`Cargo.toml`**

```toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[lib]
path = "src/lib.rs"

[features]
default = []
experimental = []

[lints.rust]
unsafe_code = "forbid"
```

### Appendix B: Binary Template

```
/apps/example-cli/
├─ Cargo.toml
└─ src/main.rs
```

**`src/main.rs`**

```rust
fn main() {
    // init logging/config here
    println!("example-cli");
}
```

### Appendix C: `xtask` Skeleton

```
/xtask/
├─ Cargo.toml
└─ src/main.rs
```

**`src/main.rs`**

```rust
fn main() -> anyhow::Result<()> {
    // e.g., tasks: fmt, lint, dist
    Ok(())
}
```
