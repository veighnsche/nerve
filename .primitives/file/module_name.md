# nrv.file::module_name

## Purpose
Marks the `file` guardrail namespace inside `nrv-rs`. Used by scaffolding and tests to confirm the
module is wired into the `nrv` surface.

## Current Behaviour
Returns the string literal `"file"`. No additional primitives are implemented yet.

## Open Questions
- What explicit read/write helpers will live under `nrv.file`?
- Which guardrails (no parent escapes, size limits, etc.) should be enforced by default vs opt-in?

## Next Steps
- Design individual primitives (`write`, `read`, `append`, `plan_write`, etc.) and add dedicated
  files in this directory as they materialise.
