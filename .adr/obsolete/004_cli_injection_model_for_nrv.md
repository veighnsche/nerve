# ADR 004: CLI Injection Model for `nrv`

## Status

- Accepted

## Context

We are designing the Nerve CLI, whose primary responsibility is to inject the `nrv` object into user-defined orchestration scripts.  
The CLI itself is implemented in Rust for performance, safety, and ecosystem integration.  

Key requirements:

- Users must be able to write orchestration scripts in **JavaScript, TypeScript, or Rust**.  
- All scripts must interact with the same capability surface (`nrv`).  
- The CLI must mediate script execution (read files, run scripts, provide `nrv`) — **only the CLI can interpret these scripts**.  
- We want **typed bindings** (TypeScript definitions) for developer experience, and a **WebAssembly layer** to isolate untrusted user code.  

Questions raised:

- What runtime to use for JS/TS (e.g. QuickJS, Deno, Node bindings)?  
- How to expose `nrv` into Rust user scripts (since there is no WASM embedding layer for Rust itself)?  

## Decision

- The **CLI MUST be implemented in Rust.**
- The CLI MUST inject a single global `nrv` object into user scripts.  
- **JavaScript / TypeScript**:
  - The CLI MUST embed a JS engine (QuickJS, or WASM runtime with JS support).
  - The `nrv` object MUST be exposed into that engine as a host binding.
  - The CLI MUST provide a **TypeScript definition file** (`nrv.d.ts`) for editor support and type checking.
  - Distribution MUST include an npm package that re-exports the TS types for developers.
- **Rust scripts**:
  - Rust user scripts CANNOT be directly interpreted like JS/TS.
  - Instead, Rust support MUST be provided by a small SDK crate (`nrv-rs`) that exposes `nrv::*` as functions/macros.
  - These Rust scripts MUST be compiled and run via the CLI (not directly by `cargo run`), so that `nrv` functions are mediated by the CLI runtime.
- **WASM Layer**:
  - The CLI MUST run user JS/TS scripts inside a WASM sandbox to ensure safety.
  - All `nrv` calls MUST cross the WASM boundary into Rust implementations.
  - Rust user scripts MAY bypass WASM, but must still call into the same Rust `nrv` implementations.

### In Scope

- JS/TS runtime embedding (QuickJS or equivalent).
- TypeScript typings for `nrv`.
- Rust SDK crate (`nrv-rs`) to allow Rust scripts to call into the CLI-provided `nrv`.
- CLI mediation: only the CLI can interpret scripts and provide `nrv`.

### Out of Scope

- Direct Node.js/Deno support (may be future work).
- Arbitrary WASM plugin execution (this ADR focuses only on JS/TS via WASM).

## Consequences

### Pros

- Unified capability surface (`nrv`) across languages.
- Strong typing for JS/TS via `nrv.d.ts`.
- Rust implementation ensures performance, safety, and native ecosystem access.
- WASM sandboxing improves safety for untrusted scripts.

### Cons

- Requires embedding a JS runtime (QuickJS or similar), which adds maintenance.
- Rust user scripts require a wrapper crate and special execution path.
- Slightly higher complexity in CLI build/distribution (npm package + Rust crate + binary).

### Neutral / Notes

- Using QuickJS is pragmatic: small, embeddable, mature. Could be swapped later (e.g. Wasmtime, V8 bindings).
- TypeScript typing layer is critical for DX but adds maintenance overhead.
- Rust support may lag behind JS/TS in ergonomics since it cannot use WASM injection.

## Alternatives Considered

- **Direct Node.js embedding**: rejected for portability (harder to ship a self-contained binary).
- **Deno embedding**: rejected; heavier runtime, less embeddable.
- **Rust-only CLI (no JS/TS)**: rejected; too restrictive for an LLM-native language.
- **Globals injection (like ADR 001)**: rejected; `nrv` object provides safer namespace.

## References

- ADR 001: Standard Library in Global Scope (superseded by ADR 003)
- ADR 002: User-Defined LLM Interactions Only
- ADR 003: Single `nrv` Object Injection
- QuickJS project: <https://bellard.org/quickjs/>
- Wasmtime project: <https://wasmtime.dev/>
