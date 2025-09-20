# ADR 016: Testing Philosophy & Gates

## Status

Accepted

## Context

Nerve must meet the same strict testing bar we set for orchestrator. To avoid “insanity,” we define clear ownership, layers, and CI gates. Every test must be explainable, reproducible, and tied to explicit specs.

---

## Decision

### 1) Ownership

* **Each crate/library**: owns its **unit tests** and **property tests**.
* **Cross-crate behavior**: only tested in integration/BDD harnesses.
* No duplication of end-to-end logic in multiple layers.

### 2) Test Layers

* **Unit tests**: fast, deterministic, one responsibility.
* **Property tests**: cover invariants (idempotence, error codes).
* **Contract tests**: verify OpenAPI schemas, config schemas, metrics contracts.
* **BDD tests**: exercise full workflows (admission → stream → apply → proofs).
* **Determinism suite**: ensure repeatable results under controlled inputs.
* **Metrics lint**: names/labels must match spec.

### 3) CI Gates

* All test layers must pass on pull requests.
* GPU-bound tests may be flagged/segregated but must exist.
* Snapshot tests (prompts, bundles, fixtures) must match committed artifacts.
* Flaky tests are forbidden — failing determinism = failing build.

### 4) Proofs & Snapshots

* Proof Bundles (ADR-012) are first-class test artifacts.
* Prompts, diffs, and transcripts must be snapshot-testable.
* Every test scenario must be reproducible in <90 seconds by a human.

### 5) Anti-Insanity Compliance

* No hidden “confidence tests” or statistical thresholds in core.
* All checks must be explicit and explainable.
* If a test can’t be explained in <90s, it must be rejected or simplified.

---

## Consequences

* Clear separation of responsibility across test layers.
* Fast feedback from unit/props, deep coverage from BDD.
* CI enforces determinism and artifact stability.
* Proof Bundles become the audit trail for correctness.

---

## Out of Scope

* Orchestrator GPU benchmarking — tracked separately.
* UI theming tests — belong to `ui-kit`, not core.

---

## References

* ADR-008: Anti-Insanity Clause
* ADR-012: Proof Bundles & Auditing
* ADR-015: Versioning & Compatibility