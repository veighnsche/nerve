# ADR 008: Anti-Insanity Clause

## Status

- Accepted

## Context

As Nerve evolves, the temptation grows to over-engineer primitives, hide intent in prompts, or pollute the API with knobs that belong in userland.  
Past iterations (e.g., early matcher designs) bundled too many responsibilities (LLM call + schema validation + routing + calibration), leading to confusion and scope creep.

This ADR introduces a formal **“Anti-Insanity Clause”**: explicit constraints that keep the CLI/core minimal, composable, and auditable.  

The clause exists to prevent:

- Hidden behavior in core primitives.
- Silent prompt pollution that obscures user intent.
- Accumulation of convenience features that increase complexity faster than value.

## Decision

- **Minimal API surface.**
  - Every primitive MUST have one clear responsibility.
  - If a feature can be written in <30 lines in userland, it MUST NOT be in core.

- **Explicit > implicit.**
  - Prompts MUST remain user-authored; the CLI MUST NOT insert policy or “confidence tricks.”
  - Core MAY append a deterministic response-format footer (e.g., JSON schema block), but MUST NOT alter instructions.

- **Userland > core.**
  - Heuristics, calibration, thresholds, and routing MUST remain userland responsibilities.
  - Core MAY offer pure helpers (e.g., `validateOneOf()`), but MUST NOT call LLMs on the user’s behalf.

- **Proof by deletion.**
  - If a proposed feature can be removed without breaking the project’s end-to-end goals, it SHOULD be removed.
  - Simpler alternatives MUST be preferred to layered complexity.

- **Snapshot principle.**
  - For any LLM interaction, the exact prompt MUST be inspectable (`compilePrompt()` style).
  - If a prompt cannot be snapshot-tested in <10 lines, the design is invalid.

- **Explainability.**
  - If a feature cannot be explained to a tired human in <90 seconds, it MUST be rejected or deferred.

- **Guardrails.**
  - Common-sense core guardrails remain (e.g., no parent folder escapes, no `rm -rf /` by default).
  - All guardrails MUST be overrideable with explicit `nrv.override.*` calls.

## Consequences

### Pros

- Keeps core primitives minimal, composable, and easy to reason about.
- Prevents “prompt pollution” that hides user intent.
- Forces repo authors to own their workflows instead of relying on hidden policy.
- Makes proof bundles auditable: inputs → outputs remain transparent.

### Cons

- Some users will want more “magic sugar” in core; they must implement it themselves.
- Repetition may increase in userland until patterns stabilize into small libs.
- Extra discipline required when reviewing new features.

### Neutral / Notes

- This ADR is normative for core; repos are free to ignore it in userland.
- Convenience wrappers may still be published as separate npm crates, but MUST NOT be added to `nrv` core.

## Alternatives Considered

- **Bake confidence/threshold into core** → rejected; pollutes prompts and hides intent.
- **Allow implicit heuristics** (e.g., auto-summarization, auto-routing) → rejected; violates explicit > implicit.
- **Do nothing** → rejected; history shows complexity creep without formal discipline.

## References

- ADR 002: User-Defined LLM Interactions Only  
- ADR 003: Single `nrv` Object Injection  
- ADR 006: How the CLI Writes Files  
- ADR 007: Context Size & Budgeting  
