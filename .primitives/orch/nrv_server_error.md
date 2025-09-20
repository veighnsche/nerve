# nrv.orch::NrvServerError

## Structure
```
pub struct NrvServerError {
    pub code: String,
    pub message: String,
    pub retriable: Option<bool>,
    pub retry_after_ms: Option<u64>,
}
```

## Purpose
Structured error envelope returned by orchestrator client methods. Provides deterministic codes and
messages along with retry hints so callers can implement their own policies.

## Open Questions
- Should `code` follow a fixed schema/enumeration for easier matching?
- How should we convey additional context (e.g., HTTP status, request IDs) without violating the
  Anti-Insanity Clause?
