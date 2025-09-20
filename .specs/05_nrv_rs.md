# nrv-rs (crates/nrv-rs)

## Purpose
- Hosts the Rust primitive surface described across ADR 006–013.
- Currently provides module stubs so dependents can compile while APIs coalesce.

## Module Plan
- `llm` — canonical LLM lifecycle primitives (capabilities → enqueue → stream/cancel) bridging user
  prompts to orchestrator transports (ADR 002/014).
- `file` — file guardrails: write/read helpers with overrideable safety rails (ADR 006).
- `dir` — directory scaffolding: create/list/apply operations with guardrails (ADR 006).
- `apply` — higher-level workflows that compose `file`/`dir` primitives.
- `ctx` — context budgeting mechanics (packing/truncation hooks) without policy (ADR 007).
- `match` — semantic matcher helpers (`compile`, `validate`, `route`) per ADR 009.
- `proof` — proof-bundle writer with JSONL core and attachment helpers (ADR 012).
- `ui` — re-export of `nrv-ui` crate so callers access applets via `nrv_rs::ui`.

## Implementation Notes
- `#![forbid(unsafe_code)]` ensures all primitives are safe Rust.
- Actual functions, types, and error envelopes remain TODO; add them incrementally with tests.
- Keep modules small and focused; export only deliberate public APIs.

## Next Steps
- Flesh out each module with typed APIs matching the ADR requirements.
- Share error types with `nrv-orch-client` once transport is wired to avoid duplication.
- Delegate UI-specific behaviour to the dedicated `nrv-ui` crate to keep core primitives focused.
- Prioritise `nrv.llm` implementation so matchers, proofs, and CLI flows have a single explicit
  bridge to orchestrator jobs.
