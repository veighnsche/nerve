# nrv.dir::module_name

## Purpose
Identifies the `dir` namespace responsible for directory helper primitives and guardrails.

## Current Behaviour
Returns the string literal `"dir"`. Acts as a placeholder until concrete directory operations are
implemented.

## Open Questions
- What directory scaffolding (create/list/apply) primitives are required to satisfy ADR-006?
- How should we represent safe traversal and exclusion patterns without implicit behaviour?

## Next Steps
- Enumerate planned primitives (e.g., `ensure`, `list`, `plan_changes`) and add one file per
  primitive under this folder.
