# llama-orch API Guidance for `nrv` CLI Snapshot Generation

## Purpose
- Direct the llama-orch IDE/AI team toward a stable HTTP surface that powers `nrv sync-capabilities`.
- Ensure the orchestrator response can be mirrored into both TypeScript (`.d.ts`) and Rust constants
  with deterministic builds.

## Required Endpoint

### `GET /capabilities`
- Returns `200 OK` with a JSON body conforming to the `CapabilitySnapshot` structure below.
- Must be side-effect free and suitable for frequent polling.
- Responses MUST be cache-safe (set `Cache-Control: no-store` if content varies rapidly).

## Response Schema (`CapabilitySnapshot`)

```jsonc
{
  "metadata": {
    "engine": "nrv-sse",
    "version": "2024.09",
    "build": "2024-09-18",
    "commit": "abcdef123456"
  },
  "limits": {
    "ctx_max": 32768,
    "max_tokens_out": 4096,
    "max_concurrent_requests": 8,
    "queue_depth_limit": 512
  },
  "workloads": [
    {
      "workload": "chat",
      "supported_models": ["meta-llama/llama-3-70b"],
      "default_model": "meta-llama/llama-3-70b",
      "supports_guardrails": true
    }
  ],
  "models": [
    {
      "id": "meta-llama/llama-3-70b",
      "display_name": "Llama 3 70B",
      "family": "meta-llama",
      "modality": "text",
      "ctx_max": 32768,
      "max_tokens_out": 4096,
      "supports_tool_calls": true,
      "supports_parallel_functions": false,
      "inference_units_per_ms": 0.42
    }
  ],
  "hardware": {
    "gpus": [
      {
        "id": "H100-PCIE-80GB-0",
        "vendor": "nvidia",
        "name": "NVIDIA H100 80GB",
        "memory_gb": 80,
        "driver": "550.54",
        "arch": "sm90"
      }
    ],
    "cpus": [
      {
        "model": "AMD EPYC",
        "cores": 64,
        "threads": 128
      }
    ]
  },
  "tools": [
    {
      "name": "vector-search",
      "description": "Fetches documents from the project index",
      "input_schema": "{\"type\":\"object\"}"
    }
  ],
  "captured_at": "2024-09-18T12:34:56Z"
}
```

### Field Requirements
- `metadata.engine` and `metadata.version` MUST be present; `build`/`commit` MAY be omitted when
  unknown.
- Array ordering MUST be handled server-side (`workloads` by `workload`, `models` by `id`, `hardware.gpus` by `id`) to keep generated `.d.ts` literal unions deterministic.
- `limits.ctx_max` is required; other limit fields MAY be `null`/absent.
- `workloads` MUST include every supported workload; keep array sorted by `workload` ascending.
- `models` MUST list all routable models; sort by `id` ascending for deterministic generation.
- `hardware.gpus` MUST present the physical IDs used in placement hints; sort by `id`.
- `captured_at` MUST be an ISO 8601 UTC timestamp.

## Error Semantics
- `4xx/5xx` errors MUST return JSON: `{ "code": "...", "message": "...", "retry_after_ms"?: number }`.
- Transport errors (timeouts, TLS) SHOULD map to `code = "E_TRANSPORT"` with actionable messaging.
- For maintenance windows, respond `503 Service Unavailable` with `retry_after_ms` hint.

## Versioning & Compatibility
- Bump `metadata.version` using `YYYY.MM` format when any field is added or semantics change.
- Additive fields MUST be optional to avoid breaking older CLI builds.
- Deprecations require a minimum one-version overlap and should be announced via release notes.

## Testing Expectations
- Provide a fixture JSON matching the schema and run it through contract tests to guarantee the
  snapshot remains serialisable.
- Ensure deterministic ordering before emitting the body (sort arrays server-side).
- Validate payload against an OpenAPI/JSON Schema definition checked into llama-orch source.

## Future Extensions
- Tool metadata MAY expand (`schema_digest`, `runtime_constraints`); keep additions optional.
- If model families gain tiering info, add fields under `ModelCapability` without renaming existing
  keys.

## Deliverables for the llama-orch Team
1. Update the API implementation to honour the schema and ordering rules.
2. Document the endpoint in llama-orch API references with example responses.
3. Supply automated regression tests to ensure new deployments cannot regress the contract.
4. Coordinate version bumps with the `nrv` CLI maintainers so code generation keeps pace.
