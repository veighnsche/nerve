# Repository Guidelines

## Project Structure & Module Organization

- Specs live under `.specs/`, anchored by `.specs/00_nerve.md` as the entry point.
- Keep new specs in `.specs/` using zero‑padded ordering: `00_topic.md`, `01_subtopic.md`.
- Use folders for domains when a file exceeds ~500 lines.

## Build, Test, and Development Commands

- Rust workspace (primary):
  - `cargo check --workspace`
  - `cargo test --workspace` (as tests are added)
  - `cargo run -p nrv -- --version`
  - `cargo run -p nrv -- sync-capabilities`
- TypeScript packages (optional):
  - `npm install --workspaces --include-workspace-root --silent --no-fund`
  - `npm run -w @nrv/core build`
  - `npm run -w @nrv/ui-kit build`
- Helpful local checks (optional):
  - `rg -n "flow\s\"" .specs` — quick scan for examples.
  - Markdown linting is NOT required; skip unless you explicitly want it.
  - Link checks are optional during early code bring‑up.
- If you add scripts or a Makefile, document commands here.

## Coding Style & Naming Conventions

- Markdown (deprioritized): use `#`, `##`, `###` headings; wrap at ~100 chars where reasonable.
- Code fences: specify language (e.g., ```bash```, ```rust```). Prefer runnable, minimal examples.
- Filenames (specs): snake_case with numeric prefix: `NN_title.md` (e.g., `02_interpreter.md`).
- Rust/TS: prefer small, composable modules; keep public surface minimal and explicit.
- Tone: instructive, precise, and implementation‑oriented; avoid speculative language.

## Testing Guidelines

- Prioritize unit and integration tests for Rust and TS code.
- For specs, keep examples correct and minimal; clearly mark non‑executable snippets.
- Intra‑repo links and anchors are nice to have; fix opportunistically.
- Markdown lint is NOT required.

## Commit & Pull Request Guidelines

- Use Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`, `test:`.
- Commits should be scoped and atomic; prefer multiple small commits over one large one.
- PRs must include: purpose/summary, notable changes, any follow‑ups, and links to issues.
- For spec changes, list impacted sections (e.g., “2) Syntax Overview → Step”).

## Agent‑Specific Instructions

- Follow this AGENTS.md across the repo; prefer minimal, surgical diffs.
- `nrv.llm` is the heart of the library. When in doubt, ensure capabilities → enqueue → stream
  surfaces stay first-class and explicit before touching other namespaces.
- Do not invent tooling; only reference commands that exist or mark them optional.
- Preserve numbering/order in spec files; when renumbering, adjust all references and anchors.
- Avoid adding licenses/headers; keep changes narrowly scoped to the task.
- Backwards compatibility: do NOT preserve it. Early-stage policy is to break APIs/CLIs as needed. Update all callers and docs within the same change. No deprecation shims.
- This supersedes any ADR suggesting future compatibility guarantees until explicitly revised.
- Keep `todo.md` in the repo root current; when it reaches "done", update its content, then run `sceipts/archive_todo.sh` to file it into `.done/NNN_todo.md` so archives stay sequential.
