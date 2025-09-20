# 🧬 The Nerve Constitution (v1)

## 0. Purpose

This Constitution defines the **core principles and final architecture of Nerve**.
It supersedes earlier ADRs and unifies the decisions that remain binding.
All future evolution must respect these principles unless explicitly amended.

---

## 1. Core Principles

1. **Userland Control**

   * All LLM interactions, workflows, and policies are authored in the repo.
   * The CLI and libraries provide only raw, composable primitives.
   * Nothing hidden, nothing injected without user consent.

2. **Minimalism**

   * Every primitive has one clear responsibility.
   * If something can be written in <30 lines in userland, it does not belong in core.

3. **Explicitness**

   * Prompts remain authored by users.
   * Core may append deterministic response contracts, but never alters intent.

4. **Explainability**

   * Every feature must be explainable to a tired human in under 90 seconds.
   * If not, it is rejected or deferred.

---

## 2. Architecture

1. **Library-First**

   * Core surfaces are libraries:

     * `@nrv/core` (JS/TS)
     * `nrv-rs` (Rust)
   * Users import these explicitly; no global injections.

2. **Micro-CLI**

   * CLI exists only as a helper for:

     * Generating capability snapshots (`nrv sync-capabilities`)
     * Initial scaffolding (`nrv init`)
   * No runtime execution. Scripts run via standard tools (node, bun, cargo).

3. **Hardware-Aware Typing**

   * CLI snapshots real server/device capabilities into generated files (e.g. GPU IDs).
   * Keeps compile-time types aligned with runtime reality.

---

## 3. File and Process Boundaries

1. **File Operations**

   * Primitives (`nrv.file.write`, `nrv.apply.changeset`) act immediately and deterministically.
   * Minimal guardrails (no `/etc`, no wildcards, no >100MB writes).
   * All guardrails overrideable (`nrv.override.*`).
   * No hidden diffs, no silent prompts — user defines review loops.

2. **User-Defined Processes**

   * Every repo defines its own `main()` process (e.g. `.nrv/index.ts`).
   * Nerve makes no assumptions about what a “project” is.
   * Works equally for Rust workspaces, JS turborepos, Python, docs, or journals.

---

## 4. LLM Policies

1. **LLM Interactions**

   * Defined explicitly in userland, versioned in the repo.
   * CLI never provides hidden roles, scaffolds, or fixers.

2. **Context Budgeting**

   * Repos own policies for packing, truncation, and summarization.
   * CLI exposes only mechanics (`nrv.ctx.estimate`, `nrv.ctx.enforce`, `nrv.tokenizers.register`).
   * Default guardrail: reject over-limit requests unless explicitly overridden.

3. **Semantic Matchers**

   * Nerve provides pure helpers (`compileOneOf`, `validateOneOf`, `route`).
   * They never call LLMs; they only validate and route deterministic JSON outputs.

---

## 5. Guardrails & Proofs

1. **Anti-Insanity Clause**

   * No hidden behavior, no prompt pollution, no silent heuristics.
   * Explicit > implicit.
   * Simpler alternatives are always preferred.

2. **Proof by Deletion**

   * If a feature can be removed without breaking the end-to-end workflow, it must be removed.

3. **Proof Bundles**

   * CLI may emit structured audit trails (inputs, prompts, diffs, ops, results).
   * Responsibility for enforcing or publishing them lies in userland.

---

## 6. Final Consequences

* Nerve is **small, composable, explicit, and library-first**.
* The CLI is an auxiliary tool, not a runtime.
* Users bear full responsibility for defining workflows, policies, and safety nets.
* This discipline enables reproducibility, auditability, and long-term sanity.

---

📜 **Ratified by ADRs 002, 005, 006, 007, 008, 009, 010.**
Earlier ADRs (001, 003, 004) are considered superseded.
