# Repository Guidelines

## Project Structure & Module Organization

- Specs live under `.specs/`. Language docs in `.specs/language/` (e.g., `.specs/language/00_nerve.md`).
- Keep new specs in `.specs/` using zero‑padded ordering: `00_topic.md`, `01_subtopic.md`.
- Use folders for domains when a file exceeds ~500 lines.

## Build, Test, and Development Commands

- No build required; this repo currently hosts specifications and docs.
- Helpful local checks (optional):
  - `rg -n "flow\s\"" .specs` — quick scan for examples.
  - `npx markdownlint "**/*.md"` — lint Markdown.
  - `npx markdown-link-check .specs/language/00_nerve.md` — validate links.
- If you add scripts or a Makefile, document commands here.

## Coding Style & Naming Conventions

- Markdown: use `#`, `##`, `###` headings; wrap at ~100 chars where reasonable.
- Code fences: specify language (e.g., ```nerve,```bash). Prefer runnable, minimal examples.
- Filenames: snake_case with numeric prefix: `NN_title.md` (e.g., `02_interpreter.md`).
- Tone: instructive, precise, and implementation‑oriented; avoid speculative language.

## Testing Guidelines

- Lint Markdown and fix warnings before opening a PR.
- Validate intra‑repo links (`[...](./relative/path.md)`) and anchors.
- For Nerve examples, ensure syntax is consistent with the spec and compiles conceptually; mark non‑executable snippets with comments.
- Add small tests/examples rather than large, monolithic blocks.

## Commit & Pull Request Guidelines

- Use Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`, `test:`.
- Commits should be scoped and atomic; prefer multiple small commits over one large one.
- PRs must include: purpose/summary, notable changes, any follow‑ups, and links to issues.
- For spec changes, list impacted sections (e.g., “2) Syntax Overview → Step”).

## Agent‑Specific Instructions

- Follow this AGENTS.md across the repo; prefer minimal, surgical diffs.
- Do not invent tooling; only reference commands that exist or mark them optional.
- Preserve numbering/order in spec files; when renumbering, adjust all references and anchors.
- Avoid adding licenses/headers; keep changes narrowly scoped to the task.
