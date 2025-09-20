# TODO

The current scaffolding matches `.plans/00_project_structure.md` and the repo state.
Use this checklist to keep the alignment true.

## Completed Alignment
- [x] `.specs/00_nerve.md` anchors the spec index and mirrors the plan references.
- [x] Spec files stay zero-padded (`00_nerve.md`, `01_ui_applets.md`, `02_nrv_object.md`).
- [x] No `.specs/language/` hierarchy remains; guidance now targets the repo root.
- [x] `cargo check --workspace` succeeds (last run `cargo check --workspace`).
- [x] `cargo run -p nrv -- --version` reports the expected version (last run `nrv 0.1.0`).

## Ongoing Maintenance
- [ ] Add new specs in order when fresh surfaces emerge.
- [ ] Keep examples minimal with explicit code-fence languages (e.g., `bash`, `json`).
- [ ] Run optional markdown/link checks as needed before publishing.

## Acceptance Criteria
- Specs remain under `.specs/` with correct numbering and cross-links.
- Workspace builds stay green (`cargo check --workspace`; `cargo run -p nrv -- --version`).
- Any new instructions in `.plans/00_project_structure.md` are reflected here as they land.
