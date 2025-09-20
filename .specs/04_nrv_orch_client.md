# nrv-orch-client (crates/nrv-orch-client)

## Purpose
- Defines the orchestrator binding shape required by ADR 014 with structured errors from ADR 011.
- Serves as a compile-time contract until a concrete transport implementation lands.

## Data Model
- `Capabilities` — complete orchestrator snapshot used by the CLI and runtime surfaces.
  - `metadata` (`OrchestratorMetadata`) — engine name, semantic version, optional build/commit identifiers.
  - `limits` (`Limits`) — global ceilings (`ctx_max`, `max_tokens_out`, optional `max_concurrent_requests`, optional `queue_depth_limit`).
  - `workloads` (`Vec<WorkloadCapability>`) — enumerates supported workload kinds (chat, tool, completion, embeddings, audio).
  - `models` (`Vec<ModelCapability>`) — catalogue of concrete model IDs with modality, context/token limits, and guardrail support flags.
  - `hardware` (`HardwareInventory`) — GPU (and optional CPU) inventory describing placement hints for generated code.
  - `tools` (`Option<Vec<ToolCapability>>`) — optional list of server-hosted tools callable by workloads.
  - `captured_at` (`Timestamp`) — ISO 8601 timestamp representing when the snapshot was produced.
- `OrchestratorMetadata`
  - `engine: String`
  - `version: String`
  - `build: Option<String>`
  - `commit: Option<String>`
- `Limits`
  - `ctx_max: u32`
  - `max_tokens_out: Option<u32>`
  - `max_concurrent_requests: Option<u32>`
  - `queue_depth_limit: Option<u32>`
- `WorkloadCapability`
  - `workload: WorkloadKind` (`"chat" | "completion" | "tool" | "embedding" | "audio"`) — encoded as Rust enum + TS string union.
  - `supported_models: Vec<String>`
  - `default_model: Option<String>`
  - `supports_guardrails: bool`
- `ModelCapability`
  - `id: String` (TS generator emits literal union via `typeof capabilities.models[number]['id']`)
  - `display_name: Option<String>`
  - `family: Option<String>`
  - `modality: ModelModality` (`"text" | "multimodal" | "audio" | "vision"`)
  - `ctx_max: u32`
  - `max_tokens_out: Option<u32>`
  - `supports_tool_calls: bool`
  - `supports_parallel_functions: bool`
  - `inference_units_per_ms: Option<f32>`
- `HardwareInventory`
  - `gpus: Vec<GpuInfo>` (TS generator exposes `type GpuId = ...` and literal unions).
  - `cpus: Option<Vec<CpuInfo>>`
- `GpuInfo`
  - `id: String`
  - `vendor: String`
  - `name: String`
  - `memory_gb: u32`
  - `driver: Option<String>`
  - `arch: Option<String>`
- `CpuInfo`
  - `model: String`
  - `cores: u16`
  - `threads: u16`
- `ToolCapability` (see [13_llm_tool_calls.md](./13_llm_tool_calls.md))
  - `name: String`
  - `description: Option<String>`
  - `input_schema: Option<String>` (validated JSON Schema as string)
  - `returns_schema: Option<String>`
  - `timeout_ms: Option<u32>`
- `TaskId` — opaque identifier for a queued task.
- `TaskRequest` — submission payload for a new task; currently only `model` is required.
- `TaskAccepted` — acknowledgement with assigned `TaskId` and optional queue position.
- `StreamEvent` — streaming lifecycle (`Started`, `Token`, `Metrics`, `End`, `Error`).
- `Cancelled` — marker type for successful cancellation responses.
- `NrvServerError` — structured error envelope with retry hints (`code`, `message`, optional `retriable`, optional `retry_after_ms`).

## Trait Contract: `OrchestratorClient`
- `type Stream` MUST be an iterator over `StreamEvent` and implement `Send`.
- `capabilities()` returns the latest `Capabilities` snapshot or `NrvServerError`.
- `enqueue(req)` submits a `TaskRequest`, returning `TaskAccepted` or `NrvServerError`.
- `stream(task_id)` yields the streaming iterator used by UI and proof bundles.
- `cancel(task_id)` resolves to `Cancelled` when the orchestrator confirms the stop.

## Error Semantics
- All methods return `Result<_, NrvServerError>` and MUST NOT panic on transport issues.
- Implementations SHOULD surface HTTP/gRPC/SSE details via `code` + `message` for auditability.
- Retries remain a caller concern; `retriable`/`retry_after_ms` are hints only.

## Next Steps
- Provide an HTTP SSE client once orchestrator endpoints stabilize.
- Extend `TaskRequest` with prompt payloads and guardrail options as ADR 006/007 primitives ship.
