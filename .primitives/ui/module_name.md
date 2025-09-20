# nrv.ui::module_name

## Purpose
Exports the namespace sentinel for UI applets, allowing consumers to confirm the `nrv.ui` category is
available when composing the `nrv` object.

## Current Behaviour
Returns the literal `"ui"`. Used in BDD scaffolding to ensure the module remains wired correctly.

## Notes
- All concrete UI primitives live in dedicated files within this folder.
- The module is implemented in `crates/nrv-ui` and re-exported via `nrv-rs::ui`.
