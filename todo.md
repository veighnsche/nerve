# TODO

Align the project plan with the current scaffolding and guidelines. Keep changes
small, implementation‑oriented, and consistent with `.specs/` conventions.

## Structure & Scaffolding
- Ensure `.specs/` and `.specs/language/` exist and remain the home for specs.
- Name spec files with zero‑padded, snake_case prefixes: `NN_title.md`.
- Split oversized specs (~500+ lines) into a folder; keep an index file.
- Maintain a short index in `.specs/` (or update an existing one) explaining
  ordering, anchors, and how to navigate sections.

## Language Specs
- Verify `.specs/language/00_nerve.md` exists as the entry point. If renumbering,
  update all intra‑repo links and anchors accordingly.
- Plan and add subsequent, focused specs in order (e.g., `01_syntax.md`,
  `02_interpreter.md`) only when needed; keep each file cohesive and <~500 lines.
- Provide minimal, runnable examples using fenced code blocks with languages set
  (e.g., `nerve`, `bash`). Mark non‑executable snippets with comments.

## Style, Lint, and Links
- Use `#`, `##`, `###` headings and wrap lines near ~100 chars.
- Before a PR, lint Markdown and fix warnings.
- Validate intra‑repo links (`[...](./relative/path.md)`) and anchors.
- Optional local checks:
  - `rg -n "flow\s\"" .specs` — quick scan for examples.
  - `npx markdownlint "**/*.md"` — lint Markdown formatting.
  - `npx markdown-link-check .specs/language/00_nerve.md` — validate links.

## Editing & Review Workflow
- Preserve numbering/order; when renumbering, adjust all references and anchors.
- Prefer small, atomic changes focused on one topic per commit.
- Use Conventional Commits (`feat:`, `fix:`, `docs:`, `refactor:`, `chore:`, `test:`).
- Keep tone instructive, precise, implementation‑oriented; avoid speculation.

## Acceptance Criteria
- Repo structure matches guidelines; specs live under `.specs/` with correct
  naming and ordering.
- Rust workspace builds (`cargo check --workspace`); CLI runs.
- TS packages build if opted in (`npm run -w @nrv/core build`).
- Cross‑references and anchors resolve where practical.
- Examples are present, minimal, and consistent with the spec.
