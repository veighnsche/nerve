# Step::info

## Signature
`fn info(&self, msg: impl Into<String>) -> Step`

## Purpose
Records an informational narration event while returning a new `Step` instance that includes the
additional `StepEvent::Info` entry.

## Behaviour
- Does not mutate the existing `Step`; instead, clones existing events, appends the new info event,
  and returns a new `Step` with the expanded history.
- The message is stored verbatim; no formatting or enrichment occurs.

## Open Questions
- Should empty messages be allowed or normalised?
- Will we need structured metadata (e.g., severity) beyond the plain info event?
