# ADR 010: Library-First Architecture with Micro-CLI

## Status

- Accepted  
- Supersedes ADR 003 (Single `nrv` Object Injection)

## Context

Earlier ADRs (notably ADR 003) assumed the CLI would inject a global `nrv` object into scripts.  
This provided “zero-ceremony” ergonomics but violated ADR 008’s Anti-Insanity Clause:

- Globals hide dependencies and complicate reasoning.
- Type safety depends on fragile ambient declarations.
- Version skew risks (CLI runtime vs. editor types).
- Mocking and testing become awkward.

We now adopt a **library-first approach**: Nerve is imported as a library in JS/TS/Rust, while a **micro-CLI** provides code generation and convenience only.

## Decision

### 1) Core Libraries

- **`@nrv/core` (JS/TS)**
  - Provides `createNrv(host)` and namespaces (`file`, `dir`, `apply`, `llm`, `ctx`, `match`, etc.).
  - No global injection. Scripts explicitly import the library.
  - Host adapters communicate with llama-orch (enqueue, stream, cancel, capabilities).
- **`nrv-rs` (Rust)**
  - Mirrors surface of `@nrv/core`.
  - Enables orchestrators written in Rust.

### 2) Micro-CLI

The CLI is a helper, not a runtime.

Commands:

- `nrv sync-capabilities`
  - Calls llama-orch for `GET /capabilities` (and related endpoints).
  - Generates:
    - `.nrv/generated/capabilities.ts` (`as const` arrays → literal unions).
    - `.nrv/generated/types.d.ts` (ambient types / module augmentation).
    - `src/nrv_capabilities.rs` (Rust module with constants).
- `nrv init` (optional)
  - Creates `.nrv/config.json` and minimal scaffolds (`index.ts`, `tsconfig.json`).
- No runner: users run scripts via `node`, `bun`, `deno`, or `cargo`.

### 3) Type Safety from Real Hardware

- CLI snapshots server/device info into generated files.
- JS/TS: unions for `GpuId`, `GpuVendor`, etc. → hoverable in editors.
- Rust: generated module with `GPU_IDS`/`GPUS`.
- Regenerate with `nrv sync-capabilities` when hardware/server changes.

### 4) Runtime Binding

- At runtime, scripts call `nrv.llm(…)` with placement hints referencing generated types.
- Snapshots may drift; regeneration keeps compile-time and runtime aligned.

## In Scope

- Libraries (`@nrv/core`, `nrv-rs`) as the primary interface.
- Micro-CLI limited to codegen/init.
- Generated modules for capabilities.

## Out of Scope

- Global `nrv` injection (ADR 003 is obsolete).
- Full runtime CLI (scripts are run by standard tools).
- Automatic dependency installation (npm/crates).

## Consequences

### Pros

- Explicit imports → clearer, testable, mockable.
- Strong type safety: hoverable GPU IDs, vendor names, VRAM sizes.
- No hidden globals → aligns with ADR 008 Anti-Insanity Clause.
- Works across runtimes (Node, Bun, Deno, Rust).
- Smaller, auditable CLI.

### Cons

- Slightly more ceremony (imports + `sync-capabilities`).
- Snapshots drift if not regenerated.
- Loses “instant workshop” style, but gains sanity.

### Neutral / Notes

- If “global feel” is desired for demos, a separate `@nrv/global` package MAY exist, but it is **not core** and must follow ADR 008 (explicit opt-in).

## Alternatives Considered

- **Keep global injection (ADR 003)** → rejected; superseded here.
- **Full runtime CLI** → rejected; too rigid.
- **No CLI at all** → rejected; codegen is essential for surfacing real hardware.

## References

- ADR 002: User-Defined LLM Interactions Only
- ADR 006: How the CLI Writes Files
- ADR 007: Context Size & Budgeting
- ADR 008: Anti-Insanity Clause
- ADR 009: Semantic Pattern Matcher
