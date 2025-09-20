# ADR 011: Error Handling & Recovery

## Status

Accepted

## Context

Nerve operates across two error domains:

1. **Client primitives** — local, deterministic failures from filesystem, context enforcement, or matchers.
2. **Server calls** — remote failures from llama-orch (`enqueue`, `stream`, `cancel`, `capabilities`).

To maintain the Constitution’s principles of **explicitness** and **explainability** and the **Anti-Insanity Clause**, all errors must be **structured, auditable, and reproducible**.
Retries, fallbacks, and escalation strategies belong in **userland**, not core.

---

## Decision

### 1. Error Model

* All primitives and server bindings return:

  * Rust: `Result<T, NrvError>`
  * JS/TS: `Promise<Result<T, NrvError>>`
* Panics are reserved for programmer bugs (invariants).
* Errors are always machine-readable objects:

```ts
{
  code: string,             // stable identifier
  message: string,          // human-readable summary
  retriable?: boolean,      // hint only
  retry_after_ms?: number,  // optional hint
  data?: any                // structured payload (e.g. offending path, http_status)
}
```

### 2. Error Taxonomy

#### Client Errors

* `FS_FORBIDDEN_PATH` — write blocked (e.g. `/etc`).
* `FS_PARENT_ESCAPE` — attempted `../` outside repo root.
* `FS_TOO_LARGE` — file write >100 MB (threshold configurable).
* `FS_INVALID_PATCH` — patch op matched 0 or >1 times.
* `CTX_OVER_LIMIT` — request tokens exceed model context.
* `MATCH_INVALID_LABEL` — LLM output not in closed set.
* `UI_ABORTED` — user rejected an approval/prompt.

#### Server Errors

* `HTTP_ERROR` — non-2xx response; `http_status` included.
* `STREAM_ABORTED` — SSE closed without `end`.
* `MODEL_UNAVAILABLE` — server returned 503 or equivalent.
* `RATE_LIMIT` — server 429; `retry_after_ms` populated if header present.
* `AUTH_REQUIRED` — Minimal Auth seam not satisfied.
* `CAPABILITY_MISMATCH` — attempted op not supported by server.

> Codes align with llama-orch logging/error taxonomy (ORCH-33xx series).

### 3. Server Integration

* HTTP errors map to `HTTP_ERROR`.
* SSE `error` frames → surfaced as `Err` with `code` and `message`.
* Disconnects without `end` → `STREAM_ABORTED`.
* Retry hints (`Retry-After`) are carried through as metadata; **never retried in core**.
* Human narration from server logs (see ADR-018) may accompany errors but does not alter them.

### 4. Proof Bundles

* Errors are logged exactly as structured objects: `{ code, message, data }`.
* Narrations may be appended, but bundles remain machine-readable (see ADR-012).
* Secrets must be redacted.

---

## Example Usage

### JS/TS

```ts
const res = await nrv.file.write("src/index.ts", code);
if (!res.ok) {
  if (res.err.code === "FS_FORBIDDEN_PATH") {
    console.error("Blocked write:", res.err.message);
  } else if (res.err.retriable) {
    await sleep(res.err.retry_after_ms ?? 1000);
    // userland retry
  }
}
```

### Rust

```rust
match nrv::file::write("src/index.rs", code) {
    Ok(_) => println!("Write succeeded"),
    Err(e) if e.code == "FS_FORBIDDEN_PATH" => {
        eprintln!("Blocked write: {}", e.message);
    }
    Err(e) if e.retriable.unwrap_or(false) => {
        std::thread::sleep(Duration::from_millis(e.retry_after_ms.unwrap_or(1000)));
        // userland retry logic
    }
    Err(e) => {
        eprintln!("Error: {} ({})", e.code, e.message);
    }
}
```

---

## Consequences

* Deterministic, auditable error handling.
* Same taxonomy across FS + orchestration.
* Proof bundles capture full error objects.
* Retries/fallbacks are explicit, never hidden.
* Slightly more boilerplate in userland, but preserves sanity and auditability.

---

## References

* Constitution (explicitness, explainability)
* ADR-008: Anti-Insanity Clause
* ADR-014: Networking & Orchestrator Binding (draft)
* ADR-018: Human Narration & Story Logging (companion)