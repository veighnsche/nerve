# ADR 002: User-Defined LLM Interactions Only

## Status

- Accepted

## Context

- We are building an orchestrator that expands a project from a single intent file into a production-ready system.
- Large Language Models (LLMs) are central to this process: they generate specs, plans, contracts, scaffolding, and fixes.
- A design tension exists: should the CLI/framework predefine how LLMs are used, or should prompts and schemas be authored by the project itself?
- Hidden or implicit LLM interactions inside the CLI would make behavior non-transparent, non-reproducible, and harder to audit.
- We want **all LLM usage to be explicit, intentional, and stored in the repo**, so developers can version, diff, and refine them like any other artifact.

## Decision

- **All LLM interactions MUST be defined by the user (inside the repo, in JS or prompt files).**
- **The CLI MUST NOT inject hidden prompts, policies, or classification logic.**
- The CLI **MAY provide only raw primitives** such as:
  - `nrv.llm(model).prompt([...])` (no prompt text added automatically)
  - `nrv.file.read`, `nrv.file.write`, `nrv.dir.tree`, etc.
- The CLI **SHOULD expose no behavior that decides how an LLM is invoked.**
- Responsibility for defining prompt instructions, roles, schemas, and output expectations **MUST remain in the user’s script and be version-controlled.**

In scope:

- User-authored functions like `llmSuggestQuickEdits` or `llmSuggestBroadEdits` that embed full prompts.
- Explicit schemas, JSON edit formats, and normative instructions written in repo files.

Out of scope:

- Pre-baked “fixers,” “scaffolders,” or “test runners” inside the CLI.
- Hidden classification or policy logic provided by the CLI.

## Consequences

### Pros

- Full transparency of all LLM interactions.
- Easy to diff, review, and audit prompts as code.
- Reproducibility: re-running the same repo yields the same LLM instructions.
- Extensible: different projects can experiment with different prompt strategies.

### Cons

- Slightly more boilerplate for users (they must define every LLM role themselves).
- Less “out-of-the-box” experience; new users must supply prompts/templates.
- Risk of poorly designed prompts unless maintained carefully.

### Neutral / Notes

- This keeps the CLI extremely lightweight: it’s more of a *runtime harness* than a “smart agent.”
- Prompts can be shared, templated, or factored into libraries if desired, but inclusion is always explicit.

## Alternatives Considered

- **Predefined LLM roles in the CLI (Rejected):**
  - Would make it easy to start, but introduces hidden behavior, non-determinism, and lock-in.
- **Hybrid approach with default fallbacks (Rejected):**
  - Still risks hidden state. Explicit > implicit.
- **Separate “prompt registry” outside the repo (Deferred):**
  - Might allow shared community best practices, but only if imported intentionally.

## References

- [ADR 001: Standard Library in Global Scope] (if applicable)
- Related spec files in `.specs/`
- Industry discussions about “prompt injection” and reproducibility
