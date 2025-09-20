# nrv.llm::request

## Purpose
Canonical entry point for issuing an LLM call. Expected to accept an explicit configuration—model id,
transport handle, inputs, guardrails—and return a handle for awaiting responses.

## Behaviour Expectations (to be designed)
- Deterministic: no hidden prompts, retries, or schema injection.
- Pure with respect to inputs: any defaults must be visible in code.
- Returns an object exposing typed accessors (e.g., `awaitJson()`, `awaitText()`, `stream()`).

## Open Questions
- Should `request` be a function, builder, or struct constructor in Rust? (Same question for TS.)
- How do we plug in transport adapters (direct HTTP vs orchestrator)?
- What guardrails (token limits, schema hints) are enforced by default?
