# nrv.llm::request

## Purpose

Canonical entry point for issuing an LLM call. Expected to accept an explicit configuration—model id,
transport handle, inputs, guardrails—and return a handle for awaiting responses.

## Behaviour Expectations (to be designed)

- Deterministic: no hidden prompts, retries, or schema injection.
- Pure with respect to inputs: any defaults must be visible in code.
- Returns an object exposing typed accessors (e.g., `awaitJson()`, `awaitText()`, `stream()`).

## Decisions

- Rust shape: use a builder for validation against capabilities.
  - `LlmRequest::builder(capabilities)` → `.model(..)` → optional `.workload(..)` / `.max_tokens(..)` → `.build()`.
  - `nrv_rs::llm::client(LlmClientConfig::new(orchestrator))` constructs the client; cheap clones.
  - JS/TS parity is planned with a similar builder and explicit transport injection.
- Transport adapters: depend on `nrv-orch-client::OrchestratorClient` trait.
  - The `client()` is generic over `O: OrchestratorClient` and accepts an `Arc<O>`.
  - Concrete HTTP/SSE implementations live outside core and are injected by the user.
- Defaults & guardrails: only explicit numeric and capability validations are enforced.
  - Validations: known model, workload ↔ supported_models, non-zero `max_tokens`, respect per-model/global token ceilings.
  - No hidden prompts, retries, schema injection, or policy; callers own guardrails and parsing.
