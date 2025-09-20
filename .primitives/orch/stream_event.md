# nrv.orch::StreamEvent

## Variants
- `Started`
- `Token(String)`
- `Metrics(String)`
- `End`
- `Error(String)`

## Purpose
Represents the streaming lifecycle for orchestrator jobs. Consumers iterate over `StreamEvent`
instances to surface narration, capture proofs, and detect completion or errors.

## Open Questions
- Should `Metrics` expose structured data rather than a `String` payload?
- How will binary/tool outputs be represented—additional variants or attachments?
