# Step::events

## Signature
`fn events(&self) -> &[StepEvent]`

## Purpose
Exposes the collected narration events for downstream consumers (e.g., proof bundles, TTY renderers).

## Behaviour
- Returns an immutable slice of the events captured so far.
- Order reflects the sequence of method calls (`info`, `ok`, `fail`) that created each event.

## Notes
- Because `Step` is currently immutable-on-write, each call to an event method returns a new `Step`
  containing an extended event slice.
- Future extensions may add richer event types; they will be documented in this folder.
