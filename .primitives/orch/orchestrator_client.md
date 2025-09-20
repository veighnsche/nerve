# nrv.orch::OrchestratorClient

## Signature
```
pub trait OrchestratorClient {
    type Stream: Iterator<Item = StreamEvent> + Send;

    fn capabilities(&self) -> Result<Capabilities, NrvServerError>;
    fn enqueue(&self, req: TaskRequest) -> Result<TaskAccepted, NrvServerError>;
    fn stream(&self, task_id: &TaskId) -> Result<Self::Stream, NrvServerError>;
    fn cancel(&self, task_id: &TaskId) -> Result<Cancelled, NrvServerError>;
}
```

## Purpose
Defines the contract between the CLI/libraries and the orchestrator transport. Implementations wrap
HTTP/gRPC/SSE clients while exposing deterministic data types and error envelopes.

## Behaviour
- `Stream` must iterate over `StreamEvent` and be `Send` so callers can process events off-thread.
- All methods return `Result` with `NrvServerError` to enforce structured error reporting.

## Open Questions
- Do we need async trait support (`async fn`) or can the iterator bridge suffice?
- How should backpressure and streaming cancellation be signalled beyond the `cancel` method?
