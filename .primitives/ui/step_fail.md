# Step::fail

## Signature
`fn fail(&self, msg: Option<impl Into<String>>) -> Step`

## Purpose
Records a failure marker so callers can capture and reason about errors explicitly in their workflows.
Returns a new `Step` containing the appended `StepEvent::Fail`.

## Behaviour
- Accepts an optional message; `None` indicates the caller has no additional context.
- Preserves the ordered event history, enabling deterministic replay in proofs or UI renders.

## Open Questions
- Should we enforce a non-empty message to avoid silent failures?
- How will failure events interact with proof bundle attachments or orchestrator telemetry?
