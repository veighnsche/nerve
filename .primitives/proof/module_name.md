# nrv.proof::module_name

## Purpose
Identifies the proof bundle namespace responsible for JSONL capture and attachment helpers (ADR-012).

## Current Behaviour
Returns `"proof"` as a sentinel. No proof emission primitives are available yet.

## Open Questions
- What APIs are needed to append narration and attachments deterministically?
- How do we surface schema/version information alongside emitted proofs?

## Next Steps
- Define specific proof helpers (`bundle`, `append`, `attach`) in individual files under this folder
  so their contracts can be designed and BDD-tested independently.
