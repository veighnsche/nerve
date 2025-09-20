# nrv.match::module_name

## Purpose
Marks the semantic matcher namespace (`nrv.match`) targeted by ADR-009.

## Current Behaviour
Returns `"match"` to confirm the namespace is wired into the `nrv` object. No compile/validate/route
helpers are present yet.

## Open Questions
- How should matcher definitions be represented (OneOf/ManyOf/etc.)?
- What invariants and error surfaces are required for deterministic routing?

## Next Steps
- Decompose planned matcher helpers into individual primitives and document them here before
  implementation.
