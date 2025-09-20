# nrv.orch::TaskId

## Structure
```
pub struct TaskId(pub String);
```

## Purpose
Opaque identifier for queued tasks. Clients treat the inner string as an orchestrator-provided token
and should not rely on internal formatting.

## Open Questions
- Should `TaskId` expose helper methods (e.g., `as_str`) or remain a thin tuple struct?
- Will we need typed newtypes for different orchestrator backends?
