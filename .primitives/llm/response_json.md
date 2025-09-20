# nrv.llm::response::json

## Purpose
Deterministically parse the model response into JSON so helpers like `nrv.match.validateOneOf` can
operate on typed data.

## Behaviour Expectations
- Validates the raw response string and returns structured data or a typed error.
- Provides hooks for schema enforcement (caller-supplied) without modifying prompts.
- Must surface parsing errors with stable error codes for proof/logging.

## Open Questions
- Should we expose streaming JSON assembly for large payloads?
- How does this helper interact with proof bundles and replay logs?
