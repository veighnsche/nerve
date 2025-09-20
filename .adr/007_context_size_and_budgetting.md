# ADR 007: Context Size & Budgeting (Policies + Minimal APIs)

## Status

- Accepted

## Context

LLM calls fail or degrade when prompt + response exceed the model’s context window.  
Today the CLI does not reason about this. The server may or may not expose max context or runtime usage.  
We need a way for repos to **define their own context policies** (pack, truncate, summarize, reserve output space) without bloating the CLI.

This ADR defines:

- Minimal APIs the CLI provides (`nrv.ctx`, `nrv.tokenizers`).
- Userland ownership of policies and tokenizers.
- What we require from the server.

## Decision (Accepted)

### 1. Userland owns policy

- The CLI core remains minimal.  
- Repos decide *how* to pack/truncate/summarize inputs.  
- Default guardrail: reject over-limit jobs pre-enqueue unless explicitly overridden.

### 2. Tokenization ownership

- **Userland must provide tokenizers.**  
  - CLI **MUST NOT** ship a zoo of tokenizers.  
  - Only two baseline families MAY be bundled (disabled by default):  
    - `llama3-bpe` (for common GGUF/LLaMA families)  
    - `gpt-bpe` (for GPT-style BPE)  
- Repos can:
  - `nrv.tokenizers.register(familyId, impl)` → provide exact implementation.  
  - Enable/disable baselines via `.nrv/config.json`.  
- If no tokenizer is found:
  - **Default:** fail fast with clear error and remediation steps.  
  - **Optional:** repo can opt into a heuristic fallback (e.g., chars/4 +15% headroom).  
- Token counts must be deterministic; proof bundles MUST record tokenizer name + version.

### 3. Minimal APIs (mechanics only)
>
> CLI provides mechanics, not policy.

```ts
// Estimate tokens
nrv.ctx.estimate({ model, chunks }): { tokens_in, per_chunk[] }

// Capabilities (from server if available)
nrv.ctx.capabilities(): {
  models: Array<{ id, max_ctx, max_tokens_out?, tokenizer_name? }>
}

// Pack helper (returns plan only, no LLM calls)
nrv.ctx.pack({ chunks, max_ctx, reserve_out, headroom, strategy }): { packed, dropped[], summaryOps? }

// Enforcement gate
nrv.ctx.enforce({ model, chunks, output_mode, max_tokens_request, override? }): { ok, why?, plan? }

// Tokenizer registry
nrv.tokenizers.register(familyId, impl)
nrv.tokenizers.resolve(model_ref): Tokenizer
nrv.tokenizers.enableBaseline("llama3-bpe" | "gpt-bpe")
nrv.tokenizers.disableAllBaselines()
````

### 4. Server expectations

The server MUST:

- **Expose model capabilities**: `max_ctx`, `max_tokens_out`, and optionally `tokenizer_name`.
- **Reject over-budget requests cleanly**: fail fast with a 400/429 style error, not silent truncation.

The server SHOULD:

- Report `tokens_used` and `tokens_remaining` for active sessions (e.g. in the `started` SSE frame).
- Optionally expose `/v1/tokenize` for exact counts by model.
- This is opt-in: the CLI never calls it automatically; repos must choose to use it.

### 5. Default CLI guardrail

- Before enqueue, if `tokens_in + reserve_out > max_ctx * (1 - headroom)` → deny with clear error.
- Escape hatch: `nrv.override.allow_over_context()`.

## Consequences

### Pros

- Prevents most over-context failures before enqueue.
- Keeps CLI minimal: helpers only, policy lives in repos.
- Clear boundary of responsibility (CLI vs userland vs server).
- Deterministic packing plans, no hidden LLM calls.

### Cons

- Users must manage tokenizers themselves (except two optional baselines).
- Estimation can be conservative when true tokenizer differs.
- Summarization is manual (intentionally).

### Neutral / Notes

- Repos can persist chosen `max_ctx`, `reserve_out`, and strategies in `.nrv/state.json`.
- Mixed engines/models are fine: policies vary per model id.

## Alternatives Considered

- **Hardcoding policy in CLI**: rejected, too opinionated.
- **Always call server for limits**: rejected, not all servers expose this.
- **Auto-summarize in CLI**: rejected, hides LLM calls; belongs in userland.

## References

- ADR 003: Single `nrv` Object Injection
- ADR 005: User-Defined Processes as the Core Goal
- ADR 006: How the CLI Writes Files
