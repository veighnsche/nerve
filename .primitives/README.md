# Nerve primitives plan

Terminology
- **Primitives** are the first-class capabilities under the `nrv` namespace (Rust + TS).
- **Applets** are the higher-level UI helpers exposed via `nrv.ui`. They are not being expanded here.
- Our goal: enumerate which primitives ship by default, what batteries we include, and what remains
  caller-defined or orchestrator-defined.

## Core LLM Flow (`nrv.llm`)
- ✅ `client(config)` — explicit transport configuration, proof sinks, tool registry.
- ✅ `client.capabilities()` — fetch orchestrator snapshot (ctx size, models, tools, hardware).
- ✅ `client.enqueue(request)` — issue LLM request with prompt, schema, tool bindings.
- ✅ `client.stream(handle)` — iterate `LlmStreamEvent` (tokens, tool calls, metrics, completion).
- ✅ `client.cancel(handle)` — stop in-flight jobs.
- ✅ `tools::registry()` — register tool descriptors/handlers (see `.specs/13_llm_tool_calls.md`).
- ✅ `request::builder()` — compose prompts, guardrails, tool bindings.
- ✅ `response::validate()` — enforce schema/contract on responses.
- ✅ `stream::adapter()` — bridge raw streams into narrations/proofs.
- ⏳ Tool attachment handling (binary payloads) — future ADR.

## File System (`nrv.file`, `nrv.dir`, `nrv.apply`)
- ✅ `nrv.file.read/write/stat/exists/remove` — with write strategies, encoding hints.
- ✅ `nrv.dir.list/ensure/remove_empty/walk` — deterministic ordering, guardrails.
- ✅ `nrv.apply.diff` — apply unified diffs with checksum, strategies.
- ⏳ Binary file support — optional flag later.
- ⏳ Recursive delete, glob writes — intentionally **out** unless explicitly enabled.

## Command Execution (`nrv.exec`)
- ✅ `nrv.exec.run` — run external commands with timeout/env/cwd options.
- ✅ Captured stdout/stderr, exit code, duration in outcome.
- ⏳ Streaming output support — optional future addition.
- ⏳ Sandboxing hooks — documented but caller-managed.

## Context & Matchers (`nrv.ctx`, `nrv.match`)
- ✅ `nrv.ctx` — placeholder for context storage (spec to be expanded with explicit API soon).
- ✅ `nrv.match` — semantic matcher helpers remain TODO; keep minimal contract (OneOf/ManyOf).
- ⏳ Actual matcher implementation (regex/semantic) — future milestone.

## Proofs (`nrv.proof`)
- ✅ Placeholder for proof bundle capture.
- ⏳ Concrete proof writer/JSONL schema — to be filled when proofs integrate with LLM events.

## UI (`nrv.ui`)
- ✅ Existing applets: `step`, `prompt.text`, `prompt.confirm`, `diff`, `progress`.
- ⛔ No new applets proposed right now; focus stays on primitives.

## CLI Batteries
- ✅ `nrv sync-capabilities` (spec). Implementation pending.
- ✅ Future CLI commands may wrap primitives (e.g., `nrv apply diff`) but remain optional.

## Guardrail Philosophy
- Defaults fail closed (e.g., atomic writes, command timeouts), but every primitive exposes options
  to relax guardrails when the user chooses.
- No hidden policy: LLM workflows orchestrate primitives; we provide batteries, not decisions.

## Open Questions
- Do we expose higher-level project scaffolding helpers (e.g., template generators)? Currently **no**.
- Where do we integrate telemetry/metrics? Pending ADR.
- Tool attachments / binary streaming — needs separate design.

Use this checklist to approve or reject primitive scopes before implementation begins.
