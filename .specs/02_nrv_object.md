# The `nrv` Object — Exhaustive Categories and Applets

This document defines the exhaustive set of categories exposed by the `nrv` object in this repo,
with a brief explanation of each category’s responsibilities and current applets. It aligns with
the active ADRs and current scaffolding. If a category or applet is not listed here, it is not part
of `nrv` at this time.

Sources: ADR‑006, ADR‑007, ADR‑009, ADR‑010, ADR‑011, ADR‑012, ADR‑013, ADR‑014, ADR‑018.

## Output Modes (UI policy, not a category)

- TTY and JSON output modes apply to UI applets (ADR‑013/018). Mode switching is explicit.

## Categories (Exhaustive)

1) nrv.llm (language model bridge)

- Purpose: deterministic helpers for issuing LLM calls while keeping prompts, transports, and
  schemas explicit (ADR‑002/014).
- Notes: callers own every token; primitives expose capabilities, enqueue, stream, cancel, and
  response helpers without hidden policy. Tool wiring lives in [13_llm_tool_calls.md](./13_llm_tool_calls.md).

2) nrv.file (file primitives)

- Purpose: single‑file operations with apply guardrails (ADR‑006).
- Notes: deterministic writes; no hidden edits; respects user prompts/policies in caller space.

3) nrv.dir (directory primitives)

- Purpose: directory introspection and creation helpers (ADR‑006).
- Notes: no project shape assumptions; caller defines layout.

4) nrv.apply (write/apply helpers)

- Purpose: explicit, auditable write/apply mechanics (ADR‑006), suitable for proof capture.
- Notes: no auto‑writes; caller chooses strategy (overwrite, merge, skip).
- Surface: `apply.diff({ path, diff, strategy?, checksum? })` applies unified diffs deterministically (see [12_nrv_apply.md](./12_nrv_apply.md)).

5) nrv.ctx (context mechanics)

- Purpose: pass explicit context/state; do not embed policy (ADR‑007).
- Notes: avoids globals; portable across environments.

6) nrv.match (semantic matcher)

- Purpose: compile/validate/route using OneOf/ManyOf style helpers (ADR‑009).
- Notes: deterministic; no hidden network or filesystem.

7) nrv.proof (proof bundles)

- Purpose: capture JSONL proof bundles and attachments (ADR‑012) in deterministic layouts.
- Notes: explicit capture only; no auto‑capture.

8) nrv.ui (interaction applets)

- Purpose: tiny, deterministic applets that degrade to JSON for CI (ADR-013/018).
- Applets (exhaustive as of now):
  - step(label) → { info, ok, fail }
  - prompt.text({ label, default? }) → Promise<string>
  - prompt.confirm({ label, default? }) → Promise<boolean>
  - diff(renderable) → "accept" | "reject" (default reject if non-interactive)
  - progress.begin(label) → { tick, end }
- Implementation: Rust surface lives in `crates/nrv-ui` and is re-exported via `nrv-rs::ui` for
  the single `nrv` object; JS surface remains in `@nrv/core`.

9) nrv.orch (orchestrator client)

- Purpose: networking binding for capabilities and job/stream control (ADR‑014).
- Surface (high‑level, normative):
  - capabilities() → fetch server capabilities/metadata
  - enqueue(request) → job id
  - stream(jobId) → async event stream
  - cancel(jobId)
- Notes: errors map to structured types per ADR‑011. Presentation stays in `nrv.ui` or userland.

## Not Present (By Design)

- No additional categories are defined beyond those listed above. Any expansion MUST arrive via ADR
  and be documented in this index before implementation begins.

## Exhaustiveness Guarantee

The categories above (file, dir, apply, llm, ctx, match, proof, ui, orch) constitute the complete `nrv`
surface in this repository at this time. Any new category or applet must be added via ADR and then
listed here to maintain exhaustiveness.

## Minimal Examples

LLM lifecycle

```ts
const caps = await nrv.llm.capabilities();
const job = await nrv.llm.enqueue({ model: caps.models[0].id, prompt: userAuthoredPrompt });
for await (const ev of nrv.llm.stream(job.id)) {
  if (ev.type === "token") process.stdout.write(ev.text);
  if (ev.type === "error") throw new Error(`${ev.code}: ${ev.message}`);
}
```

UI step

```ts
const step = nrv.ui.step("Scaffolding");
step.info("Checking repo structure");
step.ok("Done");
```
