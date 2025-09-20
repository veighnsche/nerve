# Nerve

This repository contains the specifications and implementation scaffolding for Nerve.

- Specs (RFC 2119 normalized): `./.specs/00_nerve.md`
- nrv object surface (exhaustive): `./.specs/02_nrv_object.md`
- Core UI applets (exhaustive): `./.specs/01_ui_applets.md`
- ADRs and Constitutions: `./.adr/`
- Project structure plan (actionable): `./.plans/00_project_structure.md`

## Layout

```
/.specs/                    — specifications (RFC-2119) + proofs
/.adr/                      — architecture decision records
/.plans/                    — planning documents
/crates/                    — Rust workspace members
  nrv-rs/                   — core Rust library (stubs)
  nrv-orch-client/          — orchestrator binding (stubs)
/apps/
  nrv/                      — micro-CLI (codegen/sync-capabilities stub)
/ts/packages/
  @nrv/core/                — JS/TS core library (stubs)
  @nrv/ui-kit/              — optional UI Kit (stubs)
```

## Building

Rust workspace:

```bash
cargo check --workspace
cargo run -p nrv -- --version
cargo run -p nrv -- sync-capabilities
```

TypeScript packages (optional, requires Node + TypeScript):

```bash
# from repo root
npm install --workspaces --include-workspace-root --silent --no-fund
npm run -w @nrv/core build
npm run -w @nrv/ui-kit build
```

## Scope (per ADR-010)

The `nrv` CLI is a micro-CLI used for code generation and capability sync. It is not a runtime. All
workflows and LLM interactions are userland and explicit (see ADR-002, ADR-008).
