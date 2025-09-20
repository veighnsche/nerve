# ADR 018: Human Narration & Story-Style Logging

## Status

Accepted

## Context

The llama-orch team has adopted human-readable narration in logs. These narrations let a human “read the story” of a workflow alongside structured metrics and IDs.

For Nerve, the question is:
Should the libraries auto-pretty the console, or should they just expose **UI building blocks** so userland scripts can decide?

Decision: **We do not hijack the console.** Instead, Nerve exposes UI primitives that make it possible for repos to build their own “story logger” — colored, wrapped, sectioned — while still keeping logs machine-readable.

---

## Decision

### 1. Narration Source

* Narration strings (`human`) come from llama-orch SSE/logs.
* Nerve surfaces them in structured events (e.g. `stream.on("narration", {...})`).

### 2. Userland Control

* **Nerve core does not take over the console.**
* Instead, `nrv.ui` provides minimal **logging applets**:

  * `nrv.ui.log.human(text, { section?, color?, wrap? })`
  * `nrv.ui.section.start(label, { color })` / `.end()`
  * `nrv.ui.log.json(obj)` for structured mirror
* By default, these degrade to bare console lines (TTY-safe).

### 3. Story-Style UX

* With these applets, repos can implement:

  * Line wrapping at terminal width.
  * Section coloring (e.g. pipeline stages).
  * Narration concatenation into readable paragraphs.
* Default = barebones; pretty experiences live in userland.

### 4. Proof Bundles

* Narration is recorded alongside structured events.
* Bundles include both `human` and structured fields.
* No colors or wrapping in bundles — those are UI concerns.

---

## Consequences

* **Fun debugging** possible: a repo can build a chatbot-like logger with colors & narration.
* **No hidden behavior**: core doesn’t alter output unless asked.
* **Portable**: logs remain machine-readable for CI.
* **Composable**: repos can mix human narration with their own UI flows (e.g. approvals).

---

## References

* Constitution (explicit > implicit, proof bundles)
* ADR-013: UI / Interaction Primitives
* llama-orch Human Narration spec