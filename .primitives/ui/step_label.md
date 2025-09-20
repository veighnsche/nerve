# Step::label

## Signature
`fn label(&self) -> &str`

## Purpose
Returns the immutable label assigned when the `Step` was created. Callers use this to surface
narration context in logs, proofs, or UI renderers.

## Behaviour
- Pure accessor; reflects exactly what was provided to `nrv.ui::step`.
- Does not mutate internal state.
