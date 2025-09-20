# TODO

## nrv.llm implementation
- [x] Flesh out `nrv.rs::llm` client: transport wiring for enqueue/stream/cancel.
- [x] Implement `LlmRequest` builder with validation against capabilities.
- [ ] Handle streaming event pipeline (`Token`, `ToolCall`, `ToolResult`, metrics, completion).
- [ ] Integrate tool registry: dispatch tool calls, enforce JSON Schema, return results.
- [ ] Align TypeScript client with Rust surface.
- [ ] Add unit/integration tests and BDD coverage for request/stream/tool lifecycle.

## File/Dir/Apply primitives
- [ ] Implement `nrv.file` read/write/stat/remove/exists with guardrails + tests.
- [ ] Implement `nrv.dir` list/ensure/remove_empty/walk.
- [ ] Ship diff engine for `nrv.apply.diff` (parse hunks, apply, checksum handling).
- [ ] Provide JS bindings for file/dir/apply primitives.
- [ ] Add BDD scenarios covering diff apply success/failure.

## Command execution
- [ ] Implement `nrv.exec.run` with timeouts, env/cwd controls, captured output.
- [ ] Provide JS bindings and tests.

## CLI
- [ ] Implement `nrv sync-capabilities`: orchestrator call, schema validation, generated TS/Rust files.
- [ ] Add CLI integration tests verifying deterministic output.
- [ ] Consider CLI wrappers for diff apply when core primitive is ready.

## Proofs & telemetry
- [ ] Define proof bundle schema for LLM requests, tool invocations, file writes, exec runs.
- [ ] Implement proof sink pluggability in Rust/TS clients.

## Context & matcher
- [ ] Expand `nrv.ctx` spec + implementation for storing/retrieving scoped context.
- [ ] Deliver initial matcher helpers per ADR-009.

## Documentation & Examples
- [ ] Update specs with implementation progress, examples, and guardrail overrides.
- [ ] Publish guided examples for autonomous project scaffolding once primitives land.

## Follow-up planning
- [ ] Evaluate necessity of telemetry/metrics primitives.
- [ ] Schedule ADR for tool attachments/binary support.
- [ ] Review sandbox/guardrail policies after first implementation pass.
