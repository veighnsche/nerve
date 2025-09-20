# ADR 014: Networking & Orchestrator Binding

## Status

Accepted

## Context

Nerve requires a predictable, minimal client surface to talk to the orchestrator.
The orchestrator implementation must provide the contract defined here.
If the current code does not yet match, it must be brought into compliance.

---

## Decision

### 1) Client Surface

Nerve defines four mandatory operations:

```rust
fn capabilities(&self) -> Result<Capabilities, NrvServerError>;
fn enqueue(&self, req: TaskRequest) -> Result<TaskAccepted, NrvServerError>;
fn stream(&self, task_id: TaskId) -> Stream<Item = StreamEvent>;
fn cancel(&self, task_id: TaskId) -> Result<Cancelled, NrvServerError>;
```

* **Capabilities** returns a `CapabilitySnapshot` with:

  * `metadata` → engine, semantic version, optional build/commit identifiers.
  * `limits` → global ceilings (`ctx_max`, `max_tokens_out`, optional concurrency + queue quantities).
  * `workloads` → supported workload kinds with model lists and guardrail flags.
  * `models` → catalogue entries describing modality, context ceilings, and tool-call support.
  * `hardware` → GPU inventory (and optional CPU data) for placement hints.
  * `tools` → optional server-hosted tool list (name/description/schema).
  * `captured_at` → ISO 8601 timestamp for the snapshot.
* **Stream** follows: `Started → Token* → Metrics* → End` OR `Error`.
* **Cancel** confirms job termination or returns a structured error.

### 2) Error Handling (see ADR-011)

* All failures map to `NrvServerError { code, message, retriable?, retry_after_ms?, data? }`.
* HTTP 4xx/5xx → `HTTP_ERROR` with `http_status`.
* SSE error frame → `STREAM_ERROR` with `code`/`message`.
* Disconnect without `End` → `STREAM_ABORTED`.
* `Retry-After` headers or SSE hints are surfaced as metadata only.

### 3) Authentication

* When configured, client passes `Authorization: Bearer <token>`.
* On loopback-only setups without auth, header is omitted.
* Always explicit, never guessed.

### 4) Proof Bundles & Narration

* All client events can be recorded into Proof Bundles (ADR-012).
* Narration strings from orchestrator streams are passed through unchanged.
* Core never generates narration itself.

### 5) Queue & Metrics Metadata

* Admission responses (`enqueue`) must include queue position and predicted start time when known.
* Streamed metrics may include queue depth, decode times, or other load signals.
* These are surfaced as data; no client-side policy is implied.

---

## Consequences

* Defines exactly what orchestrator must provide.
* Client surface remains minimal, deterministic, and auditable.
* No hidden retries, no fallbacks, no UI coupling.
* Proofs and narrations are recorded verbatim, ensuring replayability.

---

## Out of Scope

* Token minting or identity providers.
* Budget enforcement or model routing.
* Presentation/UI concerns.

---

## Acceptance Criteria

* Orchestrator implements `capabilities/enqueue/stream/cancel` as defined.
* SSE error frames and enriched capabilities are mandatory.
* Client returns structured errors per ADR-011.
* Proof Bundles can record full streams deterministically.

---

## References

* ADR-011: Error Handling & Recovery
* ADR-012: Proof Bundles & Auditing
* ADR-018: Human Narration & Story Logging
