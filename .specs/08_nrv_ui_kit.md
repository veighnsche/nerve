# @nrv/ui-kit (ts/packages/@nrv/ui-kit)

## Purpose
- Optional higher-level UI helpers built on top of `@nrv/core` primitives (ADR 013).
- Currently exposes a story-style logger stub to unblock downstream experimentation.

## Public API
- `type Logger` — structured logging surface with `section`, `log`, `ok`, `fail` callbacks.
- `logger(): Logger` — returns a recursive noop logger; future versions will bridge to
  terminal widgets and web renderers.

## Usage
```ts
import { logger } from "@nrv/ui-kit";

const story = logger();
story.section("bootstrap").log("cloning repo");
story.ok("ready");
```

## Implementation Notes
- Pure functions only; no process-wide state or hidden I/O.
- Build command: `npm run -w @nrv/ui-kit build`.
- Keep exports composable; richer widgets should remain opt-in modules.
