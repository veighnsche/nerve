# nrv.llm::stream

## Purpose
Provides streaming access to model outputs (tokens, tool calls, telemetry). Bridges direct LLM
interactions and orchestrator streams under a consistent interface.

## Behaviour Expectations
- Exposes iterators/async streams over structured events (e.g., `Token`, `ToolCall`, `Metrics`).
- Supports cancellation and backpressure signals compatible with `nrv.orch`.
- Keeps narration optional—callers decide how to surface partial outputs.

## Open Questions
- Should events align 1:1 with `nrv.orch::StreamEvent`, or do we need richer variants for tools?
- How do we handle streaming JSON chunks vs raw text? Do we provide helpers to assemble them?
