#![forbid(unsafe_code)]
//! Orchestrator binding (ADR-014) with structured errors (ADR-011).
//! Stub types and trait for compile-only M1 stage.

#[derive(Debug, Clone)]
pub struct Capabilities {
    pub ctx_max: u32,
    pub max_tokens_out: Option<u32>,
    pub supported_workloads: Vec<String>,
    pub engine: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskId(pub String);

#[derive(Debug, Clone)]
pub struct TaskRequest {
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct TaskAccepted {
    pub task_id: TaskId,
    pub queue_position: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum StreamEvent {
    Started,
    Token(String),
    Metrics(String),
    End,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Cancelled;

#[derive(Debug, Clone)]
pub struct NrvServerError {
    pub code: String,
    pub message: String,
    pub retriable: Option<bool>,
    pub retry_after_ms: Option<u64>,
}

pub trait OrchestratorClient {
    type Stream: Iterator<Item = StreamEvent> + Send;

    fn capabilities(&self) -> Result<Capabilities, NrvServerError>;
    fn enqueue(&self, req: TaskRequest) -> Result<TaskAccepted, NrvServerError>;
    fn stream(&self, task_id: &TaskId) -> Result<Self::Stream, NrvServerError>;
    fn cancel(&self, task_id: &TaskId) -> Result<Cancelled, NrvServerError>;
}
