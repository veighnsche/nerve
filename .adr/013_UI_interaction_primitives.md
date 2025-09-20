# ADR 013: UI / Interaction Primitives & UI Kit

## Status

Accepted

## Context

Nerve’s philosophy is **explicit, minimal core** with **user-defined processes**. We still want users to build delightful, story-style CLI experiences (colors, sectioning, chatbot loops) without reinventing the wheel. The balance:

* Core exposes **tiny, deterministic I/O applets** (LEGO bricks).
* A **separate, opt-in UI Kit** provides batteries-included UX (LEGO sets) built *on* those bricks.

## Decision

### 1) Separation of Concerns

* **`@nrv/core` / `nrv-rs` (Core):** low-level UI applets; no console takeover; no policy; no theming defaults beyond TTY safety.
* **`@nrv/ui-kit` (Separate Package):** higher-level flows and pretty renderers (wizard/chat/story logger/diff reviewer/progress themes). Explicit import by repos. Not required to use Nerve.

### 2) Core Applet Surface (mechanics only)

Each does exactly one thing; all APIs must degrade to headless/JSON for CI.

* **Steps & status**

  ```ts
  const step = nrv.ui.step("Scaffolding");
  step.info("Checking repo structure");
  step.ok("Done");
  step.fail("Could not write file");
  ```

* **Prompts**

  ```ts
  const name = await nrv.ui.prompt.text({ label: "Project name", default: "" });
  const cont = await nrv.ui.prompt.confirm({ label: "Continue?", default: false });
  ```

* **Diff approval**

  ```ts
  const decision = await nrv.ui.diff(renderableDiff); // "accept" | "reject"
  ```

* **Progress**

  ```ts
  const bar = nrv.ui.progress.begin("Compiling");
  bar.tick(5);
  bar.end();
  ```

**Output modes**

* **TTY mode:** plain text; wraps at terminal width; no global console hijack.
* **JSON mode:** emits structured events (`{ kind, label, status, data? }`) suitable for CI logs.
* Switching modes is **explicit**, default is TTY.

**Non-interactive defaults**

* If a script does not `await` a prompt, it resolves with the provided `default` (or `""/false/"reject"`). This enables “auto-programming until ready-for-everything” without hidden pauses.

**Narration**

* Applets can *display* narration strings provided by userland or server streams, but **core never synthesizes narration**.

### 3) UI Kit Surface (opt-in, built on core)

Shipped as `@nrv/ui-kit` (JS/TS). Mirrors are allowed for Rust if desired (`nrv-ui-kit`).

* **Wizard**

  ```ts
  import { wizard } from "@nrv/ui-kit";
  const answers = await wizard([
    { type: "text", label: "Project name" },
    { type: "confirm", label: "Scaffold now?" }
  ]);
  ```

* **Story Logger (narration + sections + colors + wrapping)**

  ```ts
  import { logger } from "@nrv/ui-kit";
  const log = logger.storyMode({ theme: "pipeline" }); // colors per section
  log.section("Planning").log("Checking repo").ok("Repo OK");
  ```

* **Diff Review Loop**

  ```ts
  import { reviewDiffs } from "@nrv/ui-kit";
  await reviewDiffs([{ path, diff }]); // renders list; accept/reject each
  ```

* **Chat UI**

  ```ts
  import { chatUI } from "@nrv/ui-kit";
  await chatUI({ intro: "Hi—let’s shape your repo.", onMessage });
  ```

* **Progress Themes**

  ```ts
  import { themedProgress } from "@nrv/ui-kit";
  const p = themedProgress("Build", { style: "pulse" });
  p.tick(1); p.end();
  ```

**UI Kit rules**

* Built *entirely* on `@nrv/core` applets.
* No hidden network/FS ops; presentation only.
* All pretty features are optional and replaceable; users can ignore the kit and just use core.

### 4) Integration with Proofs & Orchestrator

* UI elements **must not** auto-write proof bundles; userland controls proof capture.
* When streaming from the orchestrator, the UI Kit can render **human narration** and sections, but does not transform structured content.
* Errors are displayed using data from ADR-011; no auto-retries.

## Consequences

* **Anti-Insanity compliant:** core is minimal, explicit, and snapshot-friendly; no console hijack or hidden prompts.
* **Batteries when wanted:** the UI Kit delivers a polished CLI/chat experience with zero custom wiring, but only when explicitly imported.
* **Composable:** teams can start with the Kit and later drop to core primitives for custom flows, or vice versa.
* **Portable:** CI uses JSON mode; local dev enjoys pretty TTY.

## Out of Scope

* GUI frameworks, web servers, or desktop apps. (The Kit is CLI-first; web shells can be separate, opt-in projects.)
* Secret management or policy decisions (auth, retries, fallbacks).

## Notes for Implementers

* Respect terminal width for wrapping; never truncate structured data.
* Provide a no-color fallback (e.g., when `NO_COLOR` is set).
* Ensure output is deterministic under `--json`.
* Keep Rust parity in mind (surface symmetry where feasible).

## Alternatives Considered

* Bake the UI Kit into core → rejected (hidden policy, theming, console takeover).
* Core with no Kit → rejected (too spartan; encourages forked, inconsistent UX).
