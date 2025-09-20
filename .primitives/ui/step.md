# nrv.ui::step

## Signature
`step(label: impl Into<Arc<str>>) -> Step`

## Purpose
Creates a narration applet representing a single workflow step. The returned `Step` captures events
without producing side effects so callers can render or record narration explicitly.

## Behaviour
- Stores the provided label for later inspection.
- Starts with no recorded events; successive method calls (`info`, `ok`, `fail`) return new `Step`
  instances with appended events.
- Intended to mirror the JS/TS `@nrv/core` `ui.step` helper.

## Open Questions
- Should steps carry timestamps or sequence numbers, or should that remain a caller concern?
- How will steps integrate with proof bundle emission once implemented?
