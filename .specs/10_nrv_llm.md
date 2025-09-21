# nrv.llm (language model primitives)

## Purpose
- Provide deterministic primitives for issuing LLM calls from Rust and JS with explicit prompts,
  schemas, tooling, and transports (ADR-002, ADR-013, ADR-014).
- Supply enough batteries for autonomous workflows while keeping guardrails caller-controlled and
  overridable.
- Keep auditability first-class: every request, stream event, and tool invocation is observable.

## Implementation Status (M1)
- Implemented: client construction, capabilities → enqueue → stream/cancel, request builder with
  model/workload/token validations, simplified streaming events.
- Not yet implemented: tool registry and tool call handling; response schema validation helpers;
  finish reasons on completion events; proof adapters.
- The streaming surface in M1 maps orchestrator events to `LlmStreamEvent::{Started, Token, Metrics, Completed}`;
  errors terminate the iterator with `LlmError::Stream`.

## Client Surface
- `client(config)` — constructs an `LlmClient` with explicit transport + optional policies.
- `client.capabilities()` — resolves orchestrator `CapabilitySnapshot` (see `.specs/04_nrv_orch_client.md`).
- `client.enqueue(request)` — submits a typed request and returns `TaskHandle`.
- `client.stream(handle)` — yields `LlmStreamEvent` until completion or cancellation.
- `client.cancel(handle)` — requests cancellation with structured result.
- `tools::registry()` — registries for tool descriptors / handlers (see `.specs/13_llm_tool_calls.md`).
- `request::builder()` — helper for composing prompts, inputs, guardrails, and tool requirements.
- `response::validate()` — validates responses against caller-provided schema/expectations.
- `stream::adapter()` — convenience for bridging orchestrator streams into structured narration + proofs.
  
See also: userland scaffolding helpers in `.specs/15_llama_orch_toolkit.md`.

## Request Model
`LlmRequest` fields:
- `model: ModelId` — explicit identifier from capabilities snapshot.
- `workload: WorkloadKind` — chat/completion/tool/etc.
- `prompt: Prompt` — structured prompt object (`role`, `content`, optional attachments).
- `max_tokens: Option<u32>` — caller-specified limit (<= capabilities).
- `temperature: Option<f32>` — optional sampler config.
- `metadata: Option<Map<String, String>>` — tags carried through audit logs.
- `tools: Option<Vec<ToolBinding>>` — registered tool descriptors the request may call.
- `input_schema: Option<JsonSchema>` — expected response schema.
- `guardrails: Option<GuardrailConfig>` — optional policies (retry, stop sequences) — purely caller-owned.

All numeric values MUST be validated against capability limits before dispatch.

## Streaming Events
`LlmStreamEvent` variants:
- `Started { task_id, accepted_at }`
- `Token { text, index }`
- `ToolCall { descriptor_name, invocation_id, arguments_json }`
- `ToolResult { invocation_id, payload_json }`
- `Metrics { latency_ms?, queue_depth? }`
- `Completed { finish_reason }`
- `Error { code, message, retriable?, retry_after_ms? }`

Streams MUST be ordered and terminate with `Completed` or `Error`.

Note (M1): ToolCall/ToolResult and `finish_reason` are planned. Current code emits `Started | Token | Metrics | Completed`
and surfaces errors via `LlmError::Stream` rather than a stream `Error` variant.

## Tool Integration
- Tool descriptors registered via `tools::registry()` are serialized into `ToolBinding` entries.
- When a `ToolCall` event arrives, the runtime validates JSON payload, executes handler, and emits
  `ToolResult` back to the orchestrator.
- Timeouts, validation failures, and handler errors map to `ToolError` events and propagate to the
  orchestrator without retries unless caller policy wraps it.

## Client Configuration
`LlmClientConfig` includes:
- `transport: TransportConfig` — base URL, auth token, TLS options.
- `proof_sink: Option<ProofCollector>` — callback for recording lifecycle events.
- `tool_registry: Option<RegisteredTools>` — frozen registry from the builder.
- `default_guardrails`, `retry_policy`, `clock` — optional pluggable components.

Clients MUST be cheap clones referencing shared state.

## Error Semantics
- All public methods return `Result<_, LlmError>`.
- `LlmError` covers transport issues, schema mismatches, tool validation errors, and orchestrator
  rejections.
- Errors MUST NOT panic or retry silently; callers own policy decisions.

## Examples

```rust
let tools = nrv_rs::llm::tools::registry()
    .register(descriptor, handler)?
    .freeze();

let client = nrv_rs::llm::client(LlmClientConfig {
    transport,
    proof_sink: None,
    tool_registry: Some(tools),
    ..Default::default()
})?;

let handle = client.enqueue(LlmRequest::builder()
    .model(model_id)
    .prompt(prompt)
    .max_tokens(512)
    .build()?)?;

let mut stream = client.stream(&handle)?;
while let Some(event) = stream.next().await {
    match event? {
        LlmStreamEvent::Token { text, .. } => print!("{text}"),
        LlmStreamEvent::ToolCall { .. } => {/* handled by registry */},
        LlmStreamEvent::Completed { .. } => break,
        LlmStreamEvent::Error { code, message, .. } => panic!("{code}: {message}"),
        _ => {}
    }
}
```

## Testing Expectations
- Unit tests for request validation, tool registry, schema checks.
- Integration tests with orchestrator stubs for enqueue/stream/cancel lifecycles.
- BDD coverage for tool call handling, retry policies, and proof capture once implemented.
