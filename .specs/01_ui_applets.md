# Nerve Core UI Applets — Exhaustive List (nrv.ui)

This document enumerates the complete, current set of core UI applets exposed by the `nrv` object.
It is strictly scoped to `nrv.ui` (core). Higher‑level UX belongs to `@nrv/ui-kit` and is not listed
here. Sources: ADR‑013 (UI applets) and ADR‑018 (narration/logging constraints).

## Scope

- Core applets are minimal, deterministic, and degrade to JSON mode for CI.
- No console hijack, no hidden prompts, no synthesized narration.
- Non‑interactive defaults: prompts resolve to their `default` when not awaited.

## Output Modes (not applets)

- TTY mode: plain text; respects terminal width.
- JSON mode: structured events `{ kind, label, status, data? }`.
- Switching modes is explicit.

## Applets

The following list is exhaustive for `nrv.ui` at this time.

### 1) step(label)

Creates a scoped step with status methods.

TypeScript

```ts
const step = nrv.ui.step("Scaffolding");
step.info("Checking repo structure");
step.ok("Done");
step.fail("Could not write file");
```

Rust (parity target)

```rust
let step = nrv::ui::step("Scaffolding");
step.info("Checking repo structure");
step.ok(Some("Done"));
step.fail(Some("Could not write file"));
```

Semantics

- `info(msg: string)`: informational line within the step.
- `ok(msg?: string)`: marks step success; optional message.
- `fail(msg?: string)`: marks step failure; optional message.

### 2) prompt

Deterministic prompts. If not awaited, resolve to `default` without blocking.

Supported prompt types (exhaustive):

- `text({ label, default?: string }) -> Promise<string>`
- `confirm({ label, default?: boolean }) -> Promise<boolean>`

TypeScript

```ts
const name = await nrv.ui.prompt.text({ label: "Project name", default: "" });
const cont = await nrv.ui.prompt.confirm({ label: "Continue?", default: false });
```

Rust (parity target)

```rust
let name = nrv::ui::prompt::text("Project name", Some("".to_string()));
let cont = nrv::ui::prompt::confirm("Continue?", Some(false));
```

### 3) diff(renderable)

One‑shot diff approval.

TypeScript

```ts
const decision = await nrv.ui.diff(renderableDiff); // "accept" | "reject"
```

Semantics

- Input is a renderable diff payload (implementation‑defined by caller).
- Returns a decision: `"accept" | "reject"`.
- Non‑interactive default (if not awaited): `"reject"`.

### 4) progress

Begin/tick/end progress for long‑running tasks.

TypeScript

```ts
const bar = nrv.ui.progress.begin("Compiling");
bar.tick(5);
bar.end();
```

Rust (parity target)

```rust
let mut bar = nrv::ui::progress::begin("Compiling");
bar.tick(5);
bar.end();
```

Semantics

- `begin(label: string)` returns a progress handle.
- `tick(n: number)` advances progress by `n` units (units are caller‑defined).
- `end()` completes the progress scope.

## Out of Scope

- Any additional prompt types (select, multiselect, password), wizards, chat UI, themed progress,
  story loggers, or diff review loops live in `@nrv/ui-kit` and are not core applets.

## Exhaustiveness

As of this document, the `nrv.ui` applet surface consists only of:

- `step(label)` → `{ info, ok, fail }`
- `prompt.text`, `prompt.confirm`
- `diff(renderable)` → `"accept" | "reject"`
- `progress.begin` → `{ tick, end }`

No other core UI applets are provided. Any new core applet must be introduced via ADR and reflected
here to remain exhaustive.
