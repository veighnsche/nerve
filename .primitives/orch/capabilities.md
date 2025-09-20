# nrv.orch::Capabilities

## Structure
```
pub struct Capabilities {
    pub ctx_max: u32,
    pub max_tokens_out: Option<u32>,
    pub supported_workloads: Vec<String>,
    pub engine: Option<String>,
    pub version: Option<String>,
}
```

## Purpose
Represents the orchestrator's advertised capability limits and metadata. Consumers use this to plan
requests, enforce context budgets, and display engine/version info.

## Behaviour
- All fields are data-only; no methods are defined yet.
- Optional fields allow the server to omit data when unknown.

## Open Questions
- Should workload identifiers have a dedicated enum or remain free-form strings?
- Do we need to capture additional limits (e.g., concurrent jobs, rate limits)?
