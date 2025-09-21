/*
Purpose (bigger picture):
This script orchestrates the journey from a single intent file (Nerve.md)
to a fully realized project, using AI to expand, scaffold, and align artifacts.
The vision is: from “one file” → to “entire project” → to “ready for build,
test, release, and publish.”

Current state: only handles early scaffolding (spec + plan + dirs).
Future state: should orchestrate *all stages of readiness* until the project
is production-grade and deployable.
*/
// -------------------------------------------------------------------------------------
// REVIEW (Cascade): This prototype encodes your goal: take a single intent file (Nerve.md)
// and iteratively materialize a full, production‑ready project via model roles and
// minimal, sandboxed tools. My reading of what you want from this script:
// 1) Keep orchestration in‑repo and explicit; no hidden runtime.
// 2) Let the model write specs/plans/templates, but keep file/process mutations
//    deterministic and auditable (idempotent runs; minimal diffs; reversible).
// 3) Grow a “readiness ladder” (build → test → package → release → publish) as opt‑in stages.
// 4) Use tool calls for bounded, high‑leverage actions (create dir/file), not for policy.
// 5) Over time, replace ad‑hoc checks (LLM .bool()) with deterministic validators,
//    reserving LLMs for generation and structured planning.
//
// Migration sketch (no code change here):
// - The injected llm()/dir/file should eventually be replaced by a tiny host that:
//   (a) calls your model via an orchestrator client, (b) exposes deterministic tools,
//   (c) applies textual changes via unified diffs (nrv.apply) rather than raw writes.
// - Directory creation via a tool is fine; file content changes are best expressed as diffs.
// - Where this script asks the model to answer boolean “is scaffolding satisfied?”, consider
//   swapping to a parser that compares the real tree to the plan to avoid non‑determinism.
/*
Purpose (evolving): Orchestrate a progressive “readiness ladder” for the WayLaunch repo
using LLM-assisted, idempotent actions. Today it bootstraps early artifacts; tomorrow it
should drive the repo toward: ready-to-build → ready-to-test → ready-to-package →
ready-to-release → ready-to-publish.

Scope (open-ended, study case):
- Consume human-authored intent (e.g., Nerve.md, templates, plans) and materialize/align
  repo structure, specs, and scaffolding accordingly.
- Use injected CLI primitives (dir, file, llm, etc.)—this file declares *intent*; the
  host provides the implementation.
- Remain non-destructive and re-runnable (idempotent). Prefer additive, minimal changes.

Design principles:
- Declarative first: infer actions from documents/plans rather than hard-coding steps.
- Tool-sandboxed: expose only the smallest set of safe operations (create, write, verify).
- Deterministic enough: stable prompts/inputs → stable outputs (seed/temperature tunable).
- Auditable: every mutation is explainable from inputs (intent docs, templates, plans).

Target capabilities (to be added iteratively):
- Build: ensure minimal specs/plans/scaffolding exist.
- Test: generate/maintain smoke tests, BDD stubs, and CI hooks.
- Package: lay down manifests/config for crates/npm/pypi/deb/rpm as relevant.
- Release: versioning, changelog stubs, release notes drafting.
- Publish: registry upload wiring, AUR/deb/pypi/npm helpers, provenance/artifact surfacing.

Collaboration notes:
- Treat this as a living orchestrator. Prefer small, composable stages and clear model roles.
- Keep prompts/templates first-class; do not bake policy into code if it can live in docs.
- If you add new readiness stages, wire them behind capability checks and keep them opt-in.
- Leave breadcrumbs (logs/notes) that explain *why* a change occurred, not just *what*.

Non-goals (for now):
- One-shot “scaffold everything.” This should be iterative and reversible.
- Destructive refactors. Avoid deletes/moves unless explicitly planned and documented.

TODO (high level):
- [ ] Introduce stage registry (build/test/package/release/publish) with feature flags.
- [ ] Add validators/linters for generated artifacts before applying changes.
- [ ] Emit changelog entries for any scaffolding/actions performed.
- [ ] Seed/temperature policy for repeatability + a dry-run mode with diff previews.
- [ ] Extensible publisher adapters (crates.io, AUR, npm, PyPI, GitHub Releases, etc.).

Implementation note:
This file assumes a host CLI will inject filesystem helpers and an llm() executor.
Replace/extend models and tools as the readiness ladder grows.
*/

const root = "~/Projects/WayLaunch";
const NerveFile = root + "/Nerve.md";
const specsDir = root + "/.specs";
const plansDir = root + "/.plans";
const templatesDir = root + "/.templates";

const defaultModel = "hf:bartowski/Meta-Llama-3.1-70B-Instruct-GGUF/Meta-Llama-3.1-70B-Instruct-Q4_K_M.gguf";

const models = {
    specWriter: defaultModel,
    planWriter: defaultModel,
    extractor: defaultModel,
    templateWriter: defaultModel,
    checker: defaultModel,
    scaffolder: defaultModel,
};
// REVIEW: Model roles are clear and helpful for prompts. In a non‑injected world, these
// names map to a concrete model id chosen from capabilities, plus per‑role prompt templates.
// Keep roles, but make the transport explicit and validated against a capability snapshot.

async function main() {
    if (!fileExists(NerveFile)) {
        console.error("Nerve.md does not exist");
        return;
    }

    await ensureFirstSpecFile();
    await ensureProjectStructurePlanFile();
    await ensureProjectStructureScaffolding();
}

/**
 * Ensure the first spec file exists; if not, create it.
 */
async function ensureFirstSpecFile() {
    if (!dir.exists(specsDir) || !file.exists(specsDir + `/00_WayLaunch.md`)) {
        dir.create(specsDir);

        await llm(models.specWriter)
            // REVIEW: Generation step — good fit for LLM. Keep prompts explicit and stable.
            // Consider appending a short, deterministic response contract (e.g., "write a
            // single Markdown file; no frontmatter; no trailing spaces").
            .prompt([
                "== Nerve.md ==\n\n",
                file.read(NerveFile),
                "\n\n== Template ==\n\n",
                file.read(templatesDir + '/00_spec_template.md'),
                "\n\n== Instructions ==\n\n",
                "Write a high level spec file for the WayLaunch project based on the Nerve file and template."
            ])
            .file.write(specsDir + `/00_WayLaunch.md`);
            // REVIEW: For textual content, prefer applying a unified diff over direct writes
            // so runs are auditable and idempotent. A future host can translate this into
            // a diff application (nrv.apply) under the hood.
    }
}

/**
 * Ensure the project structure plan file exists; if not, create it.
 */
async function ensureProjectStructurePlanFile() {
    if (!dir.exists(plansDir) || !file.exists(plansDir + '/00_project_structure.md')) {
        dir.create(plansDir);

        await llm(models.planWriter)
            .prompt([
                "== Nerve.md ==\n\n",
                file.read(NerveFile),
                "\n\n== Template ==\n\n",
                file.read(templatesDir + '/10_project_structure_template.md'),
                "\n\n== Instructions ==\n\n",
                "Write a high level project structure file for the WayLaunch project based on the Nerve file and template."
            ])
            .file.write(plansDir + '/00_project_structure.md');
            // REVIEW: Treat this as the single source of truth for structure. Later stages
            // should parse this deterministically (no LLM needed) to compute required dirs/files.
    }
}

/**
 * Ensure the project structure scaffolding exists; if not, create it.
 */
async function ensureProjectStructureScaffolding() {
    const hasScaffolding = await llm(models.checker)
        .prompt([
            "== Project Tree ==\n\n",
            dir.tree(root, { depth: 2 }),
            "\n\n== Plan ==\n\n",
            file.exists(plansDir + '/00_project_structure.md')
                ? file.read(plansDir + '/00_project_structure.md')
                : "(no plan yet)",
            "\n\n== Instructions ==\n\n",
            "Return true if the current project tree satisfies the directories/files required by the plan; otherwise return false."
        ])
        .bool();
    // REVIEW: Using an LLM to return a boolean introduces non‑determinism. Strongly consider
    // replacing this with a parser that compares the real tree to the plan and returns a
    // precise list of missing nodes. You can still ask the model to REWRITE the plan, but not
    // to validate it.

    if (!hasScaffolding) {
        await llm(models.scaffolder)
            .tools([
                {
                    name: "dirCreate",
                    description: "Create a directory",
                    parameters: {
                        type: "object",
                        properties: {
                            path: {
                                type: "string",
                                description: "Path to create the directory"
                            }
                        },
                        required: ["path"]
                    },
                    func: dir.create
                },
                {
                    name: "fileCreate",
                    description: "Create a file with optional content",
                    parameters: {
                        type: "object",
                        properties: {
                            path: {
                                type: "string",
                                description: "File path to create"
                            },
                            content: {
                                type: "string",
                                description: "File contents (optional)",
                                default: ""
                            }
                        },
                        required: ["path"]
                    },
                    func: file.write
                }
            ])
            .prompt(({ readFile }) => [
                "== Plan ==\n\n",
                file.read(plansDir + '/00_project_structure.md'),
                "\n\n== Instructions ==\n\n",
                "Create any missing directories/files to satisfy the plan. Use dirCreate for directories and fileCreate for files. Keep changes minimal and idempotent."
            ])
            .concurrent({
                maxCalls: 15,
                timeout: 60000,
            });
        // REVIEW: Tool calls are a great match here. To improve determinism:
        // - Keep dirCreate for directories, but for files prefer a tool that applies a unified
        //   diff (or writes only when the target content exactly matches a provided checksum).
        // - Replace the implicit "create any missing files" with explicit plan entries so that
        //   each mutation is traceable (who/what/why) and reproducible.
        // - The host can rate‑limit and audit tool invocations; .concurrent hints map to that.
    }
}

void main();
