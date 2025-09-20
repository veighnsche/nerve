# @nrv/core (ts/packages/@nrv/core)

## Purpose
- Canonical JS/TS primitive surface for UI applets and workflow narration (ADR 013).
- Mirrors the Rust `nrv-rs` intent while staying tree-shakeable and explicit.

## Public API
- `type Step` — describes the logging callbacks (`info`, `ok`, `fail`).
- `ui.step(label: string): Step` — returns a noop logger stub for now; later will emit
  structured narration compatible with Proof Bundles.
- `version(): string` — returns the package semantic version (`0.1.0`).

## Usage
```ts
import { ui, version } from "@nrv/core";

const step = ui.step("scaffold repo");
step.info("starting");
step.ok();

console.log(version());
```

## Implementation Notes
- Module currently exports pure functions; no side effects or hidden singletons (ADR 008).
- Build command: `npm run -w @nrv/core build` (emits `dist/` via `tsc`).
- Keep APIs tiny and data-oriented; any orchestration belongs in userland scripts.
- Rust parity lives in `crates/nrv-ui`; ensure cross-language applet behaviour remains aligned.
