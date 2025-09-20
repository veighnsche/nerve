# StepEvent enum

## Variants
- `StepEvent::Info(String)`
- `StepEvent::Ok(Option<String>)`
- `StepEvent::Fail(Option<String>)`

## Purpose
Captures narration events emitted by `Step` applets. Each variant preserves caller-authored
messages without modification.

## Behaviour
- Events are stored in the order emitted.
- Optional `String` fields allow callers to omit additional context when not needed.

## Open Questions
- Do we need additional event variants (e.g., `Warn`, structured metrics) as the UI surface grows?
- Should events carry timestamps or actor metadata, or should that remain a caller concern?
