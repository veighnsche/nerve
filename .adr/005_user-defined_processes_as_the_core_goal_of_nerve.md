# ADR 005: User-Defined Processes as the Core Goal of Nerve

## Status

- Accepted

## Context

The Nerve CLI is being designed as more than a project scaffolder or build tool.  
Its purpose is to provide a **minimal capability surface (`nrv`)** that allows users to author *their own processes* for creating, maintaining, and publishing projects.

Key observations:

- My own process happens to be oriented around **Rust multi-crate workspaces**, with specs and contracts per crate.
- Other users may have completely different processes:
  - **Python** projects using venv, pytest, and Sphinx.
  - **JavaScript/TypeScript** projects with monorepos, Nx/Turbo, and npm publishing.
  - **Markdown-only projects** (e.g. documentation, personal knowledge systems, even health journals).
  - Entirely non-code projects (structured documentation, narrative logs, chat transcripts).
- Therefore, the CLI cannot hard-code a single notion of "project."  
- Instead, the CLI must make **all process logic user-authored**, with Nerve scripts stored in the repo, versioned, and executed by the CLI.

We also recognize that users may want **interactive flows**, e.g. the CLI prompting them with questions (possibly LLM-generated). This enables use-cases like chatbot-style personal workflows, as long as a user script defines them.

## Decision

- The **core goal of Nerve is to empower users to define their own processes.**
- The CLI MUST inject `nrv` into a user-defined entrypoint script (`main()`) in the repo.
- That script defines **how** to transform an intent file into artifacts (or not).
- The CLI MUST accommodate **all project styles**, including but not limited to:
  - Multi-crate Rust workspaces
  - Python monorepos
  - JS/TS turborepos
  - Docs-only repos (Markdown, RST, etc.)
  - Personal recordkeeping (health logs, journals)
- If the user’s `main()` implements an interactive mode, the CLI MUST support reading input and writing output so the script can behave like a chatbot.
- The CLI itself MUST NOT assume what “a project” is. Its responsibility is only to:
  - Expose `nrv` primitives,
  - Run the user’s entrypoint script,
  - Mediate filesystem, LLM, process, and VCS actions.

### In Scope

- Executing user-defined processes from a repo entrypoint (e.g. `.nerve/index.nrv` or equivalent).
- Supporting non-code projects (e.g. Markdown-only repos).
- Supporting interactive chatbot-like flows if defined by the user script.
- Ensuring `nrv` primitives are broad enough to cover diverse ecosystems.

### Out of Scope

- Providing built-in project generators (these must be written by users).
- Defining a canonical “best process” for all programming projects.

## Consequences

### Pros

- Maximum flexibility: every user defines the workflow that makes sense for their project.
- Language-agnostic: accommodates Rust, Python, JS/TS, or even docs-only projects.
- Encourages experimentation and innovation in “readiness ladders.”
- Scales from trivial (health log in Markdown) to complex (multi-crate orchestrator).

### Cons

- Less out-of-the-box convenience: users must author or adopt processes themselves.
- Inconsistent project structures across users unless conventions emerge.
- More responsibility on the community to share, maintain, and improve process scripts.

### Neutral / Notes

- My process (Rust crates with specs/contracts) is just **one** example; others may look completely different.
- Over time, process libraries/templates may emerge, but adoption remains user choice.

## Alternatives Considered

- **Hard-coding Rust-centric flow**: rejected; too narrow and unscalable.
- **Bundling default generators per language**: rejected; introduces bias and hidden complexity.
- **Restricting to code projects**: rejected; excludes valid use-cases like docs/journals.

## References

- ADR 002: User-Defined LLM Interactions Only
- ADR 003: Single `nrv` Object Injection
- ADR 004: CLI Injection Model for `nrv`
- `.specs/00_nerve.md` (Minimal Core Spec)
