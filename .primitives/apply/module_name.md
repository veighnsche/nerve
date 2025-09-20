# nrv.apply::module_name

## Purpose
Signals the `apply` namespace that will hold higher-level workflows composed from file and directory
primitives.

## Current Behaviour
Returns the string literal `"apply"`. No workflow helpers are published yet.

## Open Questions
- Which apply strategies (dry-run, merge, overwrite) should be first-class API surfaces?
- How will proof bundle hooks integrate with apply operations?

## Next Steps
- Draft precise apply primitives and describe each in its own markdown file within this directory.
