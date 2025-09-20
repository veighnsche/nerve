#![forbid(unsafe_code)]
//! Orchestrator binding (ADR-014) with structured errors (ADR-011).
//! Stub types and trait for compile-only M1 stage.

#[derive(Debug, Clone)]
pub struct Capabilities {
    pub metadata: OrchestratorMetadata,
    pub limits: Limits,
    pub workloads: Vec<WorkloadCapability>,
    pub models: Vec<ModelCapability>,
    pub hardware: HardwareInventory,
    pub tools: Option<Vec<ToolCapability>>,
    pub captured_at: String,
}

#[derive(Debug, Clone)]
pub struct OrchestratorMetadata {
    pub engine: String,
    pub version: String,
    pub build: Option<String>,
    pub commit: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub ctx_max: u32,
    pub max_tokens_out: Option<u32>,
    pub max_concurrent_requests: Option<u32>,
    pub queue_depth_limit: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct WorkloadCapability {
    pub workload: WorkloadKind,
    pub supported_models: Vec<String>,
    pub default_model: Option<String>,
    pub supports_guardrails: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadKind {
    Chat,
    Completion,
    Tool,
    Embedding,
    Audio,
}

#[derive(Debug, Clone)]
pub struct ModelCapability {
    pub id: String,
    pub display_name: Option<String>,
    pub family: Option<String>,
    pub modality: ModelModality,
    pub ctx_max: u32,
    pub max_tokens_out: Option<u32>,
    pub supports_tool_calls: bool,
    pub supports_parallel_functions: bool,
    pub inference_units_per_ms: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum ModelModality {
    Text,
    Multimodal,
    Audio,
    Vision,
}

#[derive(Debug, Clone)]
pub struct HardwareInventory {
    pub gpus: Vec<GpuInfo>,
    pub cpus: Option<Vec<CpuInfo>>,
}

#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub id: String,
    pub vendor: String,
    pub name: String,
    pub memory_gb: u32,
    pub driver: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u16,
    pub threads: u16,
}

#[derive(Debug, Clone)]
pub struct ToolCapability {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Option<String>,
    pub returns_schema: Option<String>,
    pub timeout_ms: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct TaskId(pub String);

#[derive(Debug, Clone)]
pub struct TaskRequest {
    pub model: String,
    pub workload: Option<WorkloadKind>,
    pub max_tokens: Option<u32>,
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
