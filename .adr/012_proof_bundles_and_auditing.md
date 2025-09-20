# ADR 012: Proof Bundles & Auditing

## Status

Accepted

## Context

The Constitution commits us to transparency, auditability, and “explicit > implicit.” It also mentions proof bundles but doesn’t fix a format yet. llama-orch is introducing **human-readable narration** alongside structured logs; we want to capture that narration *as data* without forcing a specific console UI from core. Repos can build fun, colorful “story” views in userland; core just records the facts.&#x20;

---

## Decision

### 1) Bundle format (append-only, portable)

* **Primary:** newline-delimited JSON (`.jsonl`) for stream-friendly, grep-able artifacts.
* **Event schema (minimum fields per record):**

  ```json
  {
    "ts": "2025-09-20T14:52:10.123Z",     // RFC3339
    "actor": "nrv.file.write",            // component/action
    "phase": "admission|pack|enqueue|stream|apply|finalize|error",
    "level": "info|warn|error",
    "ids": { "job_id":"...", "task_id":"...", "req_id":"..." },
    "inputs": { "path":"...", "...": "..." },   // optional, structured
    "result": { "ok":true },                     // or { "ok":false, "error":{...} }
    "metrics": { "duration_ms": 42 },            // optional
    "human": "Opened plan; packing 3 chunks for llama3 8k", // narration, userland or orch
    "tags": ["proof","v1"]                       // optional
  }
  ```
* **Sidecar attachments (optional, referenced in JSONL):**

  * `prompt.txt`, `diff.patch`, `transcript.sse`, `stdout.txt`, `stderr.txt`.

### 2) File layout (deterministic)

```
.nrv/proofs/YYYY/MM/DD/<job_id>/
  bundle.jsonl                 # append-only
  attachments/
    000_prompt.txt
    001_diff.patch
    002_stream.transcript.sse
```

### 3) Capture API (mechanics, not policy)

Core provides minimal, explicit helpers — no hidden auto-capture:

* `nrv.proof.start(job_id, { seed?, run_meta? }) -> Proof`
* `proof.append(record)`  // append immediately
* `proof.attach(name, bytes|text) -> path`
* `proof.end({ outcome: "ok" | "error" })` → MUST flush buffered writes
* **No auto-prompt capture**: userland decides what to include.

### 4) Error recording (tie-in with ADR-011)

* Errors are recorded **exactly** as structured objects:

  ```json
  { "ok": false, "error": { "code": "RATE_LIMIT", "message": "...", "retriable": true, "retry_after_ms": 1200, "data": {"http_status":429} } }
  ```
* Never mutate or summarize the error in core. Narration may *add color* but not change content.&#x20;

### 5) Human narration (source & use)

* **Source:** narration strings come from userland code and/or llama-orch logs/SSE (“human” field).
* **Nerve never generates narrations itself.**
* **UI choice:** pretty/colored “story mode” is **not** in core; repos build it using ADR-013 UI applets. Bundles remain plain data.

### 6) Redaction & safety

* **MUST NOT** store secrets (API keys, tokens).
* Provide `nrv.proof.redact(str) -> str` and common token patterns; callers are responsible for using them.
* Large blobs (e.g., 50MB+) must be attached as files, not inlined JSON.

### 7) Performance & reliability

* Appends must be **immediate and durable**.
* Core MAY buffer for throughput, but **MUST flush synchronously** on every `proof.end()` call.
* If the proof path is unwritable, **fail soft** (warn + continue) to avoid masking primary workflows.

### 8) Testing hooks & gates

* Test adapters let BDD/unit tests read bundle events and assert:

  * presence of key records (`phase` coverage),
  * narration presence for critical spans,
  * determinism of prompts/contracts (via attachments/snapshots).
* **Coverage stats belong in userland CI**, not in core. Nerve only exposes data.

### 9) Versioning

* Include `bundle_version: "1"` in the first record of each file.
* Breaking changes bump to `"2"`. Migration tools are userland.
* Aligns with our “pre-v2: no BC guarantees” policy (ADR-015).&#x20;

### 10) Minimal examples

**Append an admission step with narration (JS/TS)**

```ts
const proof = nrv.proof.start(jobId, { run_meta: { model }});
proof.append({
  ts: new Date().toISOString(),
  actor: "nrv.ctx.enforce",
  phase: "admission",
  level: "info",
  ids: { job_id: jobId },
  result: { ok: true },
  human: `Admitted request; reserving ${reserveOut} tokens for output`
});
```

**Record a server error from SSE (Rust)**

```rust
proof.append(json!({
  "ts": now_rfc3339(),
  "actor": "nrv.orch.stream",
  "phase": "stream",
  "level": "error",
  "ids": { "job_id": job_id, "task_id": task_id },
  "result": { "ok": false, "error": err_struct }, // from ADR-011 taxonomy
  "human": "Stream aborted by server before end"
}));
```

---

## Consequences

* Reproducible, auditable runs with minimal overhead.
* Human-readable **story** available to UIs without coupling core to any console theme.
* Clean separation: orch narrates, Nerve records; repos render.
* Tests can assert narration presence and critical spans without brittle string matching.
* Fully compliant with Anti-Insanity Clause: minimal, explicit, explainable.&#x20;

---

## References

* Constitution: explicitness, proofs, explainability.&#x20;
* Anti-Insanity Clause: minimal API, explicit ownership.&#x20;
* Library-first & micro-CLI.&#x20;
* Human Narration in Logs (orch spec).&#x20;