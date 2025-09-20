# nrv.orch::TaskAccepted

## Structure
```
pub struct TaskAccepted {
    pub task_id: TaskId,
    pub queue_position: Option<u32>,
}
```

## Purpose
Acknowledgement message returned by `enqueue`. Conveys the persisted `TaskId` and optional queue
position hints for UX feedback.

## Open Questions
- Should queue position be a separate struct with additional metadata (e.g., estimated wait time)?
- How will retries affect this response surface?
