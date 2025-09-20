# nrv.llm::request::builder

## Purpose
Fluent helper for assembling LLM requests. Allows callers to compose model selection, prompt
fragments, attachments, and guardrail options before executing the call.

## Behaviour Expectations
- Each builder step MUST be explicit—e.g., `set_model`, `set_prompt`, `set_schema`.
- `build()` (or equivalent) should emit a deterministic request object consumable by
  `nrv.llm::request`.

## Open Questions
- How should we validate builder state (missing prompt, missing model)?
- Do we support attaching proof bundle sinks or rely on the caller to wire them separately?
- Preferred ergonomics for Rust vs TS—mirrored APIs or idiomatic builders per language?
