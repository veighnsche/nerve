# ADR 006: How the CLI Writes Files

## Status

- Accepted

## Context

The Nerve CLI exposes primitives (`nrv.file`, `nrv.dir`, `nrv.apply`, etc.) that allow user scripts to generate and modify code.  
A central question is: **what guardrails, if any, should the CLI enforce when writing files?**

Other frameworks (e.g., LangChain) often hide complexity behind abstractions, adding layers of “memory” or “safety” that make it hard to know what is happening.  
Nerve’s philosophy is different: **users define the process**. The CLI should remain minimal, deterministic, and explicit.

This ADR proposes conventions for file-writing behavior, with a clear boundary between what belongs in **CLI core** and what belongs in **userland**.  

## Decision (Accepted)

### CLI Responsibilities

- **Expose primitives only.**

  - `nrv.file.write(path, content)` → writes immediately.
  - `nrv.dir.create(path)` → creates directory.
  - `nrv.apply.changeset(ops)` → applies atomic ChangeOps.

- **Be deterministic.**

  - Primitives do exactly what they claim, with no hidden prompts or approvals.

- **Minimal guardrails (common sense, overrideable).**

  - No parent escape: block `../` paths that leave the repo root.
  - No absolute path writes: block `/`, `/etc`, `/usr`, etc. by default.
  - No destructive wildcards: block unscoped `*` deletes.
  - Size sanity: block file writes over a large threshold (e.g. >100 MB).
  - Text sanity: block writes of high-entropy binary junk into text-suffixed files.
  - Patch sanity: block `patch` ops where `find` matches 0 or >1 times.
  - **All of the above MAY be explicitly overridden** by the user (e.g. `nrv.override.allow_parent_escape()`).

- **Never silently block.**

  - If an operation is denied, surface a clear error message with instructions for override.

### Userland Responsibilities

- **Define process.** All orchestration logic lives in user scripts (readiness ladder, specs, contracts, tests, release steps).  
- **Define UX.** If a user wants diff previews or approvals, they must program them.  
  - Example:  
    - `nrv.file.write(...)` → no approval.  
    - `nrv.file.make_diff(...)` → produces diff.  
    - `wait_for_approval(diff)` (user-implemented via `nrv.ui`) → pauses until confirmed.  
- **Define policies.**  
  - Allowed/denied paths (`.nrv/policy.json`).  
  - Max operations, max diff size, protected paths.  
- **Own system safety.** If a user directly calls `nrv.file.write`, the CLI executes.  
- **Own auditing.** Proof bundles, approvals, and history are configured at repo level, not by the CLI.

### Proof Bundles

- The CLI SHOULD provide primitives to emit proof bundles (inputs, prompts, ops, diffs, test results).  
- Whether and how these are enforced is up to userland processes.

## Consequences

### Pros

- Clear boundary: CLI is minimal and deterministic, userland is expressive and flexible.  
- Prevents accidental bricking (with override escape hatch).  
- Encourages explicit, auditable user processes.  
- Keeps CLI philosophy intact: **no hidden guardrails, no hidden prompts**.

### Cons

- Users must take responsibility for their own policies.  
- Less “out-of-the-box” safety compared to frameworks that enforce guardrails internally.  
- Risk of misuse if overrides are applied carelessly.

### Neutral / Notes

- “Interactive diffs” or “approval loops” are not built-in features. They are *patterns* the user can build on top of `nrv` primitives.  
- Overnight automation (no prompts) vs. interactive mode (approval gates) can both be implemented in userland by switching policies.

## Alternatives Considered

- **CLI-enforced global policies** (e.g., forbid editing `/etc`): rejected. Would violate Nerve’s principle that the user defines the process.  
- **Always interactive diffs**: rejected. Too much human-in-the-loop, prevents unattended runs.  
- **Free-form file writes without ChangeOps**: still allowed (`nrv.file.write`), but discouraged for complex edits. ChangeOps remain the recommended abstraction for auditability.

## References

- `.specs/00_nerve.md` (Minimal Core Spec)  
- ADR 002: User-Defined LLM Interactions Only  
- ADR 003: Single `nrv` Object Injection  
- ADR 005: User-Defined Processes as the Core Goal of Nerve
