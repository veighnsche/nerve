# llama-orch toolkit (userland scaffolding helpers)

## Purpose
- Empower users to design their own local-LLM workflows that transform a single file into a production-ready project.
- Keep core minimal and explicit: no hidden prompts, retries, or policy. Userland owns orchestration.
- Provide just enough batteries to make common flows ergonomic, auditable, and deterministic.

## Boundaries (What This Is / Is Not)
- This IS a set of small, composable helpers for:
  - Capability discovery and validation (via `nrv.llm`).
  - Building explicit requests with typed expectations (JSON plans, diffs).
  - Applying patches deterministically to disk (`nrv.apply`).
  - Running post-steps (format/test/verify) via `nrv.exec` (future).
- This is NOT:
  - A high-level magic agent, router, or auto-solver.
  - A policy engine (retries, heuristics, confidence, or prompt injection).
  - A runtime. All orchestration remains user-authored scripts in-repo.

## Core Flow (Happy Path)
1) Capabilities
- Call `client.capabilities()` and select a concrete model + workload.

2) Plan Prompt
- User constructs a prompt that instructs the model to output a JSON plan with:
  - A list of textual unified diffs (single-file patches per diff).
  - Optional commands to run after applying diffs (format/test/build).

3) Stream & Parse
- Stream tokens; once complete, parse the final JSON into a user-defined plan type.
- Validate against user-provided schema (userland validator or JSON Schema, optional).

4) Apply
- Convert plan diffs into `ApplyOptions` and call `nrv.apply.diff()` for each.
- Use `DryRun` first when desired; then `Write` or `WriteBackup` for persistence.

5) Verify (Optional)
- Run commands via `nrv.exec` (formatters/tests/linters), capture outcomes.

All steps produce proof-friendly artifacts by construction (requests, streams, diffs, outcomes).

## Data Shapes (Recommended)
- Scaffold plans should remain small and serialisable:

```jsonc
{
  "diffs": [
    {
      "path": "src/main.rs",
      "checksum": "<sha256-hex-of-preimage>",
      "unified_diff": "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n-hello\n+world\n"
    }
  ],
  "post": {
    "commands": [
      { "program": "cargo", "args": ["fmt"], "timeout_ms": 60000 },
      { "program": "cargo", "args": ["check", "--workspace"], "timeout_ms": 300000 }
    ]
  }
}
```

- Diffs MUST be single-file unified patches (multi-file patches split by caller/model).
- Checksums are optional but strongly recommended to prevent stale patch application.

## Rust API (initial)
- Provided by `nrv-rs::llm::toolkit`:
  - `ScaffoldDiff { path: PathBuf, checksum: Option<String>, diff: String }`
  - `ApplyPlan { diffs: Vec<ScaffoldDiff> }`
  - `apply_plan(plan: &ApplyPlan, strategy: ApplyStrategy) -> Result<Vec<ApplyOutcome>, ApplyError>`

Future helpers (non-normative yet):
- `parse_plan<T: DeserializeOwned>(json: &str) -> Result<T, PlanError>` (behind optional `serde`).
- `verify(commands: &[CommandOptions]) -> Result<Vec<ExecOutcome>, ExecError>` (bridges `nrv.exec`).

## Prompt Authoring Guidance
- Keep instructions explicit and short. Append a deterministic response contract:

```text
You are a code transformation assistant.
Goal: Convert the provided single file into a minimal, buildable project scaffold.
Constraints:
- Output JSON ONLY matching the schema below.
- Emit single-file unified diffs only; do not include commentary.
- If unknown, emit an empty diff list.

Schema (TypeScript):
{
  diffs: Array<{
    path: string;            // relative to repo root
    checksum?: string;       // sha256 of preimage
    unified_diff: string;    // unified diff
  }>;
  post?: {
    commands?: Array<{ program: string; args?: string[]; timeout_ms?: number }>;
  };
}
```

- Provide the file contents (or a path + explicit file reading step in userland) as input.
- Consider a two-pass flow: (1) plan JSON, (2) apply + verify, (3) optional refinement round.

## Determinism & Guardrails
- No hidden retries or schema injection.
- Plan parsing/validation MUST be explicit in userland.
- `nrv.apply` MUST be used for file mutations; default to `DryRun` before writing.
- Use `WriteBackup` for destructive changes to preserve preimages.

## Testing Expectations
- Unit tests for `apply_plan` ensuring:
  - Multiple diffs apply in deterministic order.
  - Checksum mismatches fail fast.
  - Dry-run yields `Noop/Applied` status without touching disk.
- Integration tests (future):
  - End-to-end flow with a stub orchestrator emitting a plan → apply → verify.

## Example (Rust)
```rust
use nrv_rs::llm::{self, toolkit};
use nrv_rs::apply::ApplyStrategy;

// 1) Create client with your orchestrator impl (omitted)
let client = llm::client(llm::LlmClientConfig::new(my_orchestrator))?;
let caps = client.capabilities()?;

// 2) Build a request (prompt + schema authoring is userland; omitted here)
let req = llm::LlmRequest::builder(caps)
    .model("meta-llama/llama-3-70b")
    .max_tokens(1024)
    .build()?;

let handle = client.enqueue(req)?;
let events: Vec<llm::LlmStreamEvent> = client.stream(&handle)?.collect::<Result<_, _>>()?;

// 3) Parse final JSON into `ApplyPlan` (userland; placeholders here)
let plan = toolkit::ApplyPlan { diffs: vec![
    toolkit::ScaffoldDiff { path: "src/main.rs".into(), checksum: None, diff: "...".into() }
]};

// 4) Apply diffs deterministically
let outcomes = toolkit::apply_plan(&plan, ApplyStrategy::WriteBackup { backup_suffix: ".bak" })?;
```

## Future Extensions
- Optional serde-bound helpers for JSON plan parsing.
- Built-in JSON Schema validator integration (user-supplied schema string, no policy).
- Convenience adapters for popular orchestrators once HTTP/SSE endpoints stabilise.
