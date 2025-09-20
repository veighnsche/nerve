# Nerve Specification — Constitution and ADRs (Normative Summary)

 This document normalizes the full set of Nerve architectural decisions into RFC 2119
 language. The keywords MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY are to be interpreted as
 described in RFC 2119.

 All entries below summarize the original ADRs and Constitutions housed under `.adr/`. Each
 section states the binding requirements and provides a reference link to the source ADR.

 ---

## Reference Surfaces

- Exhaustive `nrv` object categories and applets: [02_nrv_object.md](./02_nrv_object.md)
- Core LLM lifecycle (capabilities → enqueue → stream): [10_nrv_llm.md](./10_nrv_llm.md)
- Core UI applets only (subset of the above): [01_ui_applets.md](./01_ui_applets.md)

## Constitution (Authoritative Principles)

### Constitution v1 — Core Principles and Architecture

 Source: [ADR 900: The Nerve Constitution (v1)](../.adr/900_consitution.md)

- Userland control
  - All LLM interactions, workflows, and policies MUST be authored inside the repository.
  - The CLI and libraries MUST expose only raw, composable primitives.
  - The CLI and libraries MUST NOT inject hidden prompts, policies, or behaviors without explicit
     user consent.

- Minimalism
  - Every primitive MUST have one clear responsibility.
  - Features that can be written in fewer than ~30 lines in userland MUST NOT be in core.

- Explicitness
  - Prompts MUST remain user-authored.
  - Core MAY append deterministic response-contract footers (e.g., JSON schema blocks) but MUST
     NOT alter user instructions.

- Explainability
  - Every feature MUST be explainable to a tired human in under 90 seconds; otherwise it MUST be
     rejected or deferred.

- Architecture
  - Nerve MUST be library-first: `@nrv/core` (JS/TS) and `nrv-rs` (Rust) are the primary surfaces.
  - The `nrv.llm` namespace MUST exist in core and provide the canonical
    capabilities → enqueue → stream/cancel lifecycle for model calls.
  - The micro-CLI MAY provide code generation and initialization, and MUST NOT serve as a runtime.
  - Scripts MUST run via standard tools (node, bun, deno, cargo), not the CLI.
  - The CLI MAY snapshot real server/device capabilities into generated, typed files.

- File and process boundaries
  - File operations MUST be immediate and deterministic (e.g., `nrv.file.write`, `nrv.apply`).
  - Minimal guardrails (no parent escapes, no absolute sensitive paths, no destructive wildcards,
     size/text sanity) MUST be enforced by default and MUST be explicitly overrideable.
  - Hidden diffs or implicit approvals MUST NOT occur; review loops MUST be userland.
  - Nerve MUST NOT assume what a “project” is; processes MUST be user-defined and portable across
     ecosystems (Rust, Python, JS/TS, docs-only, journals, etc.).

- LLM policies
  - LLM interactions MUST be explicit and version-controlled in-repo; hidden roles or fixers MUST
     NOT exist in core.
  - Core primitives MUST surface structured errors, response parsing, and streaming hooks for LLM
     jobs without injecting retries or policy.
  - Context budgeting policies (pack/truncate/summarize) MUST be userland; the CLI MAY provide
     mechanics and MUST provide a default over-context guardrail with an explicit override.
  - Semantic matchers MUST be pure helpers (compile/validate/route) and MUST NOT call LLMs.

- Guardrails and proofs
  - The Anti-Insanity Clause MUST hold: no hidden behavior, no prompt pollution, explicit > implicit.
  - Features removable without breaking end-to-end MUST be removed (“proof by deletion”).
  - Proof Bundles MAY be emitted as structured audit trails; enforcement and publishing MUST remain
     in userland.

### Constitution v2 — Ratified Extensions (ADR 011–018)

 Source: [ADR 901: The Nerve Constitution (v2)](../.adr/901_constitution.md)

- Core principles reaffirmed
  - Explicit > Implicit and Anti-Insanity MUST continue to govern all surfaces.
  - Proofs are first-class artifacts; retries/policies/UI assembly MUST remain in userland.

- Error handling (ADR 011)
  - All primitives and server bindings MUST return structured errors (stable `code`, `message`,
     hints as data). Panics MUST be reserved for programmer invariants. Retries/fallbacks MUST NOT
     be in core.

- Proof Bundles (ADR 012)
  - JSONL schema with attachments MUST be supported; appends MUST be immediate and durable; human
     narration MUST be recorded as data; core MUST NOT synthesize narration.

- UI/Interaction (ADR 013)
  - Core MUST offer minimal applets (`step`, `prompt`, `diff`, `progress`) and MUST degrade to a
     JSON mode for CI. A richer UI Kit MAY exist as an opt-in package.

- Networking & Orchestration (ADR 014)
  - Orchestrator binding MUST provide `capabilities`, `enqueue`, `stream`, and `cancel` with SSE
     error semantics and enriched capabilities.

- Versioning (ADR 015)
  - Pre-v2 releases MUST NOT promise compatibility. v1 MUST be artifact-stable (not API-stable).
     From v2 onward, breaking changes MUST follow formal deprecation/migration.

- Testing (ADR 016)
  - Multi-layer testing and CI gates MUST enforce determinism and prohibit flakiness; Proof Bundles
     MUST be test artifacts.

- Narration & Logging (ADR 018)
  - Narration from the orchestrator MUST be surfaced; core MUST NOT generate narration; Proof
     Bundles MUST carry narration as plain data only.

- Security Boundaries (ADR 017)
  - Core MUST NOT manage secrets; users MUST rely on host-language conventions (env/dotenv/config).
  - Logs and proofs MUST redact sensitive values.

 ---

## ADRs — Normative Summaries

 Each entry states status, binding requirements, and references. Obsolete ADRs remain for historical
 context and MUST NOT be implemented where superseded.

### ADR 000 — Template (Non‑normative authoring rules)

 Source: [ADR 000: Template](../.adr/000_template.md)

- ADR documents MUST include: Status, Context, Decision, Consequences, Alternatives, References.
- Decision sections SHOULD use RFC 2119 terms (MUST/SHOULD/MAY/MUST NOT) to express normative
   requirements.
- ADRs SHOULD cross-link to related ADRs and specs; examples MAY be included when they clarify
   normative impact.

### ADR 001 — Standard Library in Global Scope (Obsolete)

 Source: [ADR 001 (obsolete)](../.adr/obsolete/001_stdlib.md)

- This ADR is obsolete. Implementations MUST NOT rely on global stdlib injection.
- Library-first architecture from ADR 010 MUST be followed instead.

### ADR 002 — User‑Defined LLM Interactions Only

 Source: [ADR 002](../.adr/002_user_defined_llm_interactions_only.md)

- All LLM interactions MUST be defined by the user inside the repo (code or prompt files).
- The CLI MUST NOT inject hidden prompts, policies, or classification logic.
- The CLI MAY provide raw primitives only (e.g., `nrv.llm().prompt`, `nrv.file.*`, `nrv.dir.*`).
- The CLI SHOULD NOT expose behavior that decides how an LLM is invoked.
- Responsibility for prompt instructions, schemas, and outputs MUST remain in user scripts under
   version control.

### ADR 003 — Single `nrv` Object Injection (Obsolete)

 Source: [ADR 003 (obsolete)](../.adr/obsolete/003_single_nvr_object_injection.md)

- Superseded by ADR 010. Implementations MUST NOT depend on global `nrv` injection. Scripts MUST
   import libraries explicitly per ADR 010.

### ADR 004 — CLI Injection Model for `nrv` (Obsolete)

 Source: [ADR 004 (obsolete)](../.adr/obsolete/004_cli_injection_model_for_nrv.md)

- Superseded by ADR 010. The CLI MUST NOT act as a runtime that injects `nrv`. Micro-CLI duties
   are limited to code generation and init; scripts MUST run via standard tools.

### ADR 005 — User‑Defined Processes as the Core Goal of Nerve

 Source: [ADR 005](../.adr/005_user-defined_processes_as_the_core_goal_of_nerve.md)

- Nerve’s primary goal MUST be to empower user-defined processes.
- Nerve MUST accommodate diverse project styles (Rust, Python, JS/TS, docs-only, journals, etc.).
- Nerve MUST support interactive flows if user scripts request them (I/O primitives in core).
- Nerve MUST NOT assume a canonical “project” shape; processes MUST be user-authored and versioned
   in-repo.
- NOTE: Any earlier notion of `nrv` injection is superseded by ADR 010’s library-first design.

### ADR 006 — How the CLI Writes Files

 Source: [ADR 006](../.adr/006_how_the_cli_writes_files.md)

- The CLI MUST expose primitives only and be deterministic (no hidden prompts or approvals).
- Common-sense guardrails (no parent escapes, no absolute sensitive paths, no destructive
   wildcards, size/text sanity, patch sanity) MUST be enabled by default and MUST be overrideable.
- The CLI MUST NOT silently block operations; denials MUST include clear, actionable errors.
- Users MUST own orchestration, UX, and policies; approvals/diff reviews SHOULD be implemented in
   userland using `nrv.ui`.
- The CLI SHOULD provide primitives to emit Proof Bundles; enforcement MAY be chosen by userland.

### ADR 007 — Context Size & Budgeting (Policies + Minimal APIs)

 Source: [ADR 007](../.adr/007_context_size_and_budgetting.md)

- Context policies MUST live in userland; the CLI core MUST remain minimal.
- Tokenizers MUST be provided by userland; the CLI MUST NOT ship a large zoo of tokenizers.
- The CLI MAY provide two optional baseline tokenizer families (disabled by default) and MUST fail
   fast with clear errors when tokenizers are missing.
- The CLI MUST provide minimal mechanics (`nrv.ctx.*`, `nrv.tokenizers.*`) without policy.
- Over-context requests MUST be denied by default before enqueue, with an explicit override escape
   hatch.

### ADR 008 — Anti‑Insanity Clause

 Source: [ADR 008](../.adr/008_anti_insanity_clause.md)

- Core API surface MUST remain minimal; each primitive MUST have a single clear responsibility.
- Features implementable in <30 lines in userland MUST NOT be added to core.
- Prompts MUST remain user-authored; core MUST NOT insert policy or confidence tricks.
- Core MAY append deterministic response-format footers but MUST NOT alter instructions.
- Heuristics, calibration, thresholds, and routing MUST remain userland responsibilities.
- Any feature not necessary for end-to-end MUST be removed (proof by deletion).
- Designs MUST be snapshot-testable and explainable in <90 seconds.
- Guardrails MUST be overrideable via explicit `nrv.override.*` calls.

### ADR 009 — Semantic Pattern Matcher (OneOf / ManyOf)

 Source: [ADR 009](../.adr/009_semantic_pattern_matcher.md)

- Matchers MUST produce deterministic response contracts for prompts and MUST validate LLM JSON
   against closed label sets (with optional per-label schemas).
- Matchers MUST NOT call LLMs, invent confidence/thresholds, or modify user instructions.
- A simple, pure router MAY be provided to dispatch on labels.

### ADR 010 — Library‑First Architecture with Micro‑CLI

 Source: [ADR 010](../.adr/010_library_first_architecture_with_micro_cli.md)

- `@nrv/core` and `nrv-rs` MUST be the primary interfaces; scripts MUST import explicitly.
- The CLI MUST be a micro-CLI for code generation and initialization only; it MUST NOT be a runtime.
- Type safety MAY be derived from real hardware/server capability snapshots generated by the CLI.
- Global `nrv` injection MUST NOT be used (ADR 003 obsolete). No full runtime CLI.

### ADR 011 — Error Handling & Recovery

 Source: [ADR 011](../.adr/011_error_handling_and_recovery.md)

- All primitives and bindings MUST return structured error objects with stable `code`s.
- Core MUST NOT perform implicit retries; hints MAY be surfaced as data.
- Server errors, SSE issues, and disconnects MUST be mapped to explicit error codes.
- Proof Bundles MUST record errors exactly; secrets MUST be redacted.

### ADR 012 — Proof Bundles & Auditing

 Source: [ADR 012](../.adr/012_proof_bundles_and_auditing.md)

- Proof Bundles MUST use JSONL with optional sidecar attachments in a deterministic layout.
- Core MUST provide minimal capture helpers and MUST NOT auto-capture without explicit calls.
- Appends MUST be durable; `proof.end()` MUST flush synchronously.
- Narration MAY be included as data; redaction MUST protect secrets.

### ADR 013 — UI / Interaction Primitives & UI Kit

 Source: [ADR 013](../.adr/013_UI_interaction_primitives.md)

- Core MUST provide tiny, deterministic UI applets and MUST degrade to JSON mode for CI.
- A separate `@nrv/ui-kit` MAY provide higher-level flows; it MUST be opt-in and built on core.
- Prompts without `await` MUST resolve to defaults to enable non-interactive flows.
- Core MUST NOT hijack the console; narration is displayed only if provided by userland/orch.

### ADR 014 — Networking & Orchestrator Binding

 Source: [ADR 014](../.adr/014_networking_and_orchestration_binding.md)

- The client surface MUST provide `capabilities`, `enqueue`, `stream`, `cancel` with specified
   semantics.
- Errors MUST map to structured server error types (see ADR 011).
- Authentication MUST be explicit when configured; no guessing.
- Proof Bundles MUST be able to record full streams deterministically.

### ADR 015 — Versioning & Compatibility Policy

 Source: [ADR 015](../.adr/015_versioning_and_backwards_compatibility.md)

- Pre‑v1/0.x releases MUST offer no compatibility guarantees.
- v1 MUST be artifact-stable (Proof Bundle schema and primitive semantics) but MAY change APIs.
- v2 MUST begin the backwards-compatibility contract; breaking changes MUST follow deprecation.
- Generated files under `.nrv/generated/*` MAY be invalidated before v2; users MUST re‑generate.
- Releases MUST state stability phase and any schema/API changes.

### ADR 016 — Testing Philosophy & Gates

 Source: [ADR 016](../.adr/016_testing_philosophy_and_gates.md)

- Ownership MUST be clear: unit/prop tests per crate/lib; cross-crate behavior in integration/BDD.
- Test layers MUST include unit, property, contract, BDD, determinism, and metrics lint.
- CI MUST enforce all layers; flakiness MUST fail the build; snapshots MUST match committed
   artifacts.
- Proof Bundles MUST be first-class test artifacts; scenarios MUST be reproducible quickly.

### ADR 017 — Security Boundaries & Secrets

 Source: [ADR 017](../.adr/017_security_boundries_and_secrets.ms) (content currently empty)
         and [ADR 901 §Security Boundaries](../.adr/901_constitution.md)

- Nerve core MUST NOT manage or store secrets.
- Users MUST rely on host-language conventions for secret management (environment variables,
   dotenv, config crates).
- Logs and Proof Bundles MUST redact sensitive values.
- Large sensitive blobs MUST NOT be inlined in JSON; attachments SHOULD be used where necessary.

### ADR 018 — Human Narration & Story‑Style Logging

 Source: [ADR 018](../.adr/018_human_narration_and_story_style_logging.md)

- Nerve core MUST NOT hijack the console; it MUST provide logging applets only.
- Narration strings from the orchestrator MUST be surfaced as structured events; core MUST NOT
   synthesize narration.
- Proof Bundles MUST record narration as plain data; presentation concerns MUST remain in userland.

### ADR 900 — The Nerve Constitution (v1)

 Source: [ADR 900](../.adr/900_consitution.md)

- See “Constitution v1” above; those requirements MUST be treated as binding.

### ADR 901 — The Nerve Constitution (v2)

 Source: [ADR 901](../.adr/901_constitution.md)

- See “Constitution v2” above; those extensions MUST be treated as binding.

 ---

## Conformance

- Implementations of Nerve libraries and the micro-CLI MUST conform to all MUST/MUST NOT
   statements above.
- Repositories using Nerve SHOULD reference these sections to derive their policies and tests.
- Changes to any binding behavior MUST be introduced via new or amended ADRs and reflected here.
