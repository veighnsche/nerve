# nrv.ctx::module_name

## Purpose
Identifies the `ctx` namespace dedicated to context budgeting mechanics per ADR-007.

## Current Behaviour
Returns `"ctx"` as a sentinel. No budgeting helpers or token accounting primitives exist yet.

## Open Questions
- What minimal API should expose pack/truncate hooks without embedding policy?
- How will user-provided tokenizers integrate with these helpers?

## Next Steps
- Catalogue planned primitives (e.g., `budget`, `pack`, `truncate`) and capture their design in
  standalone files so they can be implemented and tested incrementally.
