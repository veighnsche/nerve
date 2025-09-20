# nrv.llm tool calls

## Purpose
- Define how `nrv.llm` discovers and exposes deterministic tool calls to models.
- Ensure tool usage remains explicit, auditable, and caller-controlled per ADR-002/ADR-014.
- Align Rust and JS implementations so orchestrator snapshots (`ToolCapability`) map to runtime callables.

## Concepts

### Tool Descriptor
- `ToolDescriptor` captures metadata shared with the orchestrator:
  - `name: String` — stable identifier (lowercase, kebab-case).
  - `description: String` — human-readable summary surfaced to the model.
  - `input_schema: JsonSchema` — JSON Schema (Draft 07) describing request payload.
  - `returns: JsonSchema` — optional schema describing tool responses.
  - `timeout_ms: Option<u32>` — optional execution limit for host runtime.
- Descriptors MUST be serialisable to JSON (`@nrv/core` → orchestrator) and to Rust structs for proof capture.

### Tool Implementation
- Host provides an executable closure/function matching the descriptor signature:
  - Rust: `async fn(&ToolInvocation) -> ToolResult` (Send + Sync).
  - JS/TS: `async (invocation: ToolInvocation) => ToolResult`.
- Tool implementations MUST NOT capture hidden state; dependencies are passed explicitly via context.
- Tools MUST return structured results (`{ output, warnings?, attachments? }`) or typed errors.

### Tool Invocation Lifecycle
1. Caller assembles `ToolRegistry` with descriptors + implementation handles.
2. `nrv.llm.client({ tools })` registers the tool set and validates schemas.
3. When orchestrator streams a `ToolCall` event, the runtime:
   - Validates payload against the descriptor schema.
   - Executes the registered implementation under timeout/guardrails.
   - Emits `ToolResult` back to the model stream (structured JSON).
4. Proof bundle records invocation request, serialized payload, result, and duration.

## API Surface (Rust)
- `nrv_rs::llm::ToolDescriptor` — struct mirroring metadata fields.
- `nrv_rs::llm::ToolRegistry` — builder for registering tools:
  - `ToolRegistry::new()`
  - `.register(descriptor: ToolDescriptor, handler: ToolHandler)`
  - `.freeze()` → `RegisteredTools` used by `client()`.
- `ToolHandler = Arc<dyn Fn(ToolInvocation) -> ToolFuture + Send + Sync>`.
- `ToolInvocation` — includes `tool_name`, `input` (serde_json::Value), `context` (caller-provided).
- `ToolResult` — success `{ output, warnings?, attachments? }` or error `ToolError { code, message }`.
- `ToolCallStreamEvent` — variant in orchestrator stream mapping to `ToolInvocation`.

## API Surface (JS/TS)
- `type ToolDescriptor = { name: string; description: string; inputSchema: JsonSchema; returns?: JsonSchema; timeoutMs?: number }`.
- `type ToolHandler = (invocation: ToolInvocation) => Promise<ToolResult>`.
- `createToolRegistry()` builder with `.register()`, `.freeze()` mirroring Rust semantics.
- `ToolInvocation` & `ToolResult` types aligned with Rust (JSON-compatible).

## Orchestrator Contract Alignment
- `ToolCapability` (from `.specs/04_nrv_orch_client.md`) MUST match `ToolDescriptor` fields.
- CLI `sync-capabilities` writes tool metadata into generated snapshots (TS & Rust) to expose `ToolName` unions.
- Models advertising `supports_tool_calls: true` MUST declare which tool names are valid in `ToolCapability`.

## Behavioural Requirements
- Tool registration MUST be explicit; no ambient/global tool discovery.
- Duplicate tool names MUST be rejected during `.register()`.
- Schema validation occurs at registration time; invalid JSON Schema is an error.
- Invocation MUST reject payloads that fail JSON Schema validation.
- Tool errors MUST be surfaced to the model stream without retries unless caller supplies policy.
- Execution MUST observe timeouts; exceeding timeout results in `ToolError { code: "TIMEOUT" }`.
- Attachments (binary/data) are out of scope pending ADR for tool attachments.

## Proof & Telemetry
- Each invocation appends `ToolInvocationEvent` to proof bundles: `{ tool_name, input_hash, started_at, finished_at, status }`.
- Warnings (validation fallback, truncated output) MUST be returned to callers.

## Testing Strategy
- Unit tests: registry validation, duplicate detection, schema parsing, timeout handling.
- Integration tests: simulate orchestrator tool call stream event, ensure handler executed.
- End-to-end (future): orchestrator stub invoking tool through `nrv.llm.client` with captured proof artifacts.

## Next Steps
- Extend `.specs/10_nrv_llm.md` to reference tool registry.
- Implement registry stubs in Rust/TS with BDD coverage (tool registration + invocation happy path + error).
- Update CLI snapshots to include tool metadata and regenerate `.d.ts` unions.
