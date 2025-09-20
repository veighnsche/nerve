Understood — let’s move straight to **ADR-015: Versioning & Compatibility Policy**. Crisp, authoritative, no fluff.

---

# ADR 015: Versioning & Compatibility Policy

## Status

Accepted

## Context

Nerve is in **pre-alpha**. Stability and backwards compatibility slow down progress at this stage. To keep iteration fast and avoid “insanity,” we explicitly declare **zero backwards-compatibility guarantees** until we reach v2. This must be unambiguous so users know what to expect.

---

## Decision

### 1) Pre-v1 / 0.x

* **No compatibility guarantees.**
* APIs, file formats, and generated artifacts may be renamed, moved, or removed without notice.
* Users must pin exact versions or update code/scripts when upgrading.

### 2) v1

* **Artifact-stable, not API-stable.**
* Proof Bundle schema and core primitive semantics are stable enough for external tooling.
* CLI/library APIs may still change or be reorganized.

### 3) v2

* **First backwards-compatibility promise.**
* After v2, breaking changes require formal deprecation and migration notes.

### 4) Generated Files

* Contents of `.nrv/generated/*` may be invalidated by any release before v2.
* Users must re-generate instead of assuming stability.

### 5) Documentation

* Each release must explicitly state:

  * Current stability phase (0.x / v1 / v2).
  * Whether any Proof Bundle schema changes occurred.
  * Whether any core APIs were renamed or removed.

---

## Consequences

* Maximizes velocity in early phases.
* Users cannot assume stability until v2.
* Once at v2, we lock into a compatibility contract and carry migration overhead.

---

## Out of Scope

* Third-party crates/kits built on Nerve: they manage their own versioning.
* Orchestrator release cadence: tracked separately.

---

## References

* Constitution (explicitness, explainability)
* ADR-008: Anti-Insanity Clause (simplicity > convenience)
* ADR-012: Proof Bundles (artifact stability baseline)

---

👉 Ready to push on to **ADR-016: Testing Philosophy & Gates** next?
