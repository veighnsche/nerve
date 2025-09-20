# nrv-ui (crates/nrv-ui)

## Purpose
- Hosts the Rust UI applet primitives described in ADR 013 and ADR 018.
- Provides the canonical `nrv.ui` implementation while keeping narration helpers isolated from
  the rest of `nrv-rs`.

## Public API (initial surface)
- `module_name(): "ui"` — module sentinel used by BDD scaffolding and proofs.
- `step(label: impl Into<Arc<str>>): Step` — constructs an immutable narration step.
- `Step::info(msg)` / `Step::ok(msg?)` / `Step::fail(msg?)` — record narration events without side
  effects.
- `Step::label()` / `Step::events()` — expose collected data for proof bundles or UI renderers.

## Behavioural Notes
- Applets MUST remain deterministic and side-effect free; they record events that callers can wire
  into custom logging, Proof Bundles, or UX surfaces.
- Narration content is caller-authored; the crate MUST NOT inject text, prompts, or summarisation.
- `nrv-rs` re-exports this crate as `nrv_rs::ui` to preserve the single `nrv` object surface.

## Next Steps
- Add JSON/TTY renderers that consume `StepEvent` without mutating the underlying data.
- Extend the crate with `prompt`, `diff`, and `progress` applets once their ADR requirements land in
  code.
- Share event types with proof bundle helpers so narration can be emitted into proofs verbatim.
