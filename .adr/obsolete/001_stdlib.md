# ADR 001: Standard Library in Global Scope

## Status

Accepted

## Context

Every language must decide how its *standard library (stdlib)* is made available:

* **Explicit imports** (like Rust `use`, Python `import`) give clarity but add boilerplate.
* **Implicit globals** (like JavaScript’s `console`, `Math`, `JSON`) maximize accessibility and reduce friction for beginners, but risk namespace collisions and “magical” behavior.

Nerve’s mission is to be an LLM-native, workflow-oriented process language.
The first user experience matters: a newcomer typing `print("hello")` should succeed immediately, without imports or ceremony. At the same time, we require determinism, structured events, and reproducibility.

Additionally, the **CLI must parse multiple surface syntaxes**. Nerve’s core execution model is independent of syntax, but users and LLMs will expect to author in familiar languages. Supporting **JavaScript, TypeScript, and Rust** as valid front-ends allows wide accessibility:

* **JS**: lightweight scripting, minimal ceremony, closest to the “global stdlib” philosophy.
* **TS**: typed superset of JS, better safety and editor support.
* **Rust**: systems-level syntax, explicit semantics, and compatibility with existing Rust tooling.

The stdlib must be **consistent across all supported syntaxes**.

## Decision

* Nerve will ship with a **standard library of core functions** that are **globally in scope** by default.
* No explicit `import` or `use` is required to access these functions.
* This applies regardless of the surface language chosen (JS, TS, Rust).
* The CLI will include parsers/transpilers that normalize JS/TS/Rust front-ends into the same Nerve IR (Intermediate Representation).

### Global stdlib (initial set)

* **Logging**

  * `print(msg: string)` → sugar for `say msg`
  * `say msg: string` → structured info log
  * `info { … }`, `warn { … }`, `error { … }` → structured logs with fields
* **Core values**

  * `now()` → current timestamp (UTC, deterministic in replay)
  * `uuid()` → seeded UUID (deterministic in replay mode)
* **Facts I/O**

  * `emit(fact: object)` → append structured fact
  * `last_error()` → retrieve last error fact
* **Utilities**

  * `json(path: string)` → load fact by path
  * `file(path: string)` → read file snapshot (FS broker enforced)

### Language Surface Notes

* **JavaScript**

  * Global functions are injected into runtime (similar to `console.log`).
  * Shadowing rules apply (user fn overrides with warning).

* **TypeScript**

  * Same globals, but with **.d.ts** definitions for type safety and editor hints.
  * Example:

    ```ts
    declare function print(msg: string): void;
    declare function now(): string;
    ```

* **Rust**

  * CLI parses Rust-like syntax and auto-injects the stdlib into scope.
  * Example:

    ```rust
    fn main() {
        print("hello"); // works without use/import
    }
    ```

### Rules

* Stdlib functions are **always available**, no imports needed.
* All stdlib calls must produce **deterministic, structured events** in the run stream.
* Collisions: if user defines a fn with same name as stdlib, the **user definition shadows** (with warning).
* Future stdlib expansion must be **versioned** and **documented in ADRs**.
* Surface language is **orthogonal to semantics** — all front-ends map into the same execution IR.

## Consequences

**Pros**

* Zero-ceremony “hello world” in JS, TS, or Rust (`print("hello")` just works).
* Lower barrier to entry (LLMs and humans don’t need to learn import semantics).
* Structured determinism preserved (even `print()` is structured under the hood).
* Consistent stdlib across different authoring syntaxes.

**Cons**

* Risk of namespace clutter and collisions.
* Harder to reason about what symbols are “in scope”.
* Tooling must track stdlib version to ensure reproducible runs.
* Supporting three syntaxes increases parser/maintenance complexity.

## Alternatives Considered

* **Explicit imports (`use nerve.std.print`)**: rejected for v0 because it slows onboarding and hinders LLM authorability.
* **Hybrid (globals for sugar, imports for advanced)**: possible future refinement if stdlib grows large.
* **Single surface language only**: rejected, as multi-language support aligns with LLM-native workflows and user familiarity.

## Notes

* This ADR sets the precedent: **Nerve has a global stdlib**.
* Future ADRs (e.g., `.adr/002_stdlib_expansion.md`) will add categories like math, string ops, and FS utilities.
* A future ADR will define the **IR layer** that normalizes JS/TS/Rust into a single deterministic execution core.
