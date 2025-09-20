# nrv.orch::Cancelled

## Structure
```
pub struct Cancelled;
```

## Purpose
Marker type returned when the orchestrator confirms a cancellation request succeeded. The absence of
fields emphasises the lack of additional metadata at this stage.

## Open Questions
- Do we need to bubble up cancellation reasons or timestamps in the future?
