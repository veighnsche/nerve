# nrv CLI (apps/nrv)

## Purpose
- Provides the micro-CLI described in ADR 010; intentionally thin wrapper over the library surfaces.
- Ships only user-facing entry points that can be explained quickly and audited in version control.
- Emits usage guidance to STDERR and prints machine-friendly responses to STDOUT.

## Commands

### `nrv --version`
- Prints the crate version defined by `CARGO_PKG_VERSION`.
- Output format is `nrv <semver>` with a trailing newline.
- Returns exit code `0`.

### `nrv sync-capabilities`
- Calls the orchestrator `capabilities` endpoint once and materialises the snapshot into generated modules.
- Fails closed if the orchestrator responds with an error, malformed payload, or mismatched schema.
- Writes the following files (all deterministic, newline-normalised, UTF-8):
  - `.nrv/generated/capabilities.ts` — `export const capabilities = { … } as const;`
  - `.nrv/generated/types.d.ts` — ambient types (`type ModelId = …`, `interface CapabilitySnapshot { … }`).
  - `apps/nrv/src/nrv_capabilities.rs` — `pub const CAPABILITIES: CapabilitySnapshot = …;` (Rust mirror for libraries/tests).
- Uses temporary files + atomic rename to avoid partial snapshots.
- Returns exit code `0` when files are written successfully; non-zero for transport or IO failures.

#### Generated TypeScript surface
- `.nrv/generated/types.d.ts` MUST export the following:
  - `type WorkloadKind = "chat" | "completion" | "tool" | "embedding" | "audio";`
  - `type ModelModality = "text" | "multimodal" | "audio" | "vision";`
  - `type ModelId = typeof capabilities.models[number]['id'];`
  - `type GpuId = typeof capabilities.hardware.gpus[number]['id'];`
  - `interface CapabilitySnapshot` mirroring `.specs/04_nrv_orch_client.md`; optional fields use `?`, never `null`.
  - `interface WorkloadCapability`, `interface ModelCapability`, `interface HardwareInventory`, `interface GpuInfo`, `interface CpuInfo`, `interface ToolCapability` matching the Rust types.
  - `export { capabilities } from './capabilities';`
- `.nrv/generated/capabilities.ts` MUST export `const capabilities: CapabilitySnapshot` with `as const` to retain literal unions.
- Consumers import from `@nrv/generated/types` (document alias in `README.md`).

#### Generated Rust surface
- `apps/nrv/src/nrv_capabilities.rs` MUST expose:
  - `pub use nrv_rs::orch::{CapabilitySnapshot, WorkloadKind, ModelModality, ModelCapability, HardwareInventory};` (or equivalent re-export).
  - `pub const CAPABILITIES: CapabilitySnapshot` filled from the snapshot.
  - `pub static MODEL_IDS: &[&str]` and `pub static GPU_IDS: &[&str]` sorted lexicographically.

#### Determinism requirements
- Sort `workloads` by `workload`, `models` by `id`, and `hardware.gpus` by `id` before serialising.
- Emit UTF-8 with `\n` line endings and a trailing newline.
- Omit optional fields entirely instead of outputting `null`.

### `nrv --help`
- Displays usage summary and available commands.
- `nrv -h` is short-hand; both return exit code `0`.

### Unknown command
- Emits `Unknown command: <arg>` followed by usage text.
- Exits with status code `2`.

## Usage
```bash
cargo run -p nrv -- --help
```

## Extension Plan
- Wire `sync-capabilities` to orchestrator bindings when ADR 014 client matures.
- Document any new subcommands here; keep behavior minimal and explicit.
