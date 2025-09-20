# Plan: <Project / Component Name>

Status: Draft  
Owner: @<team/maintainer>  
Date: YYYY-MM-DD  
Version: v0.x  

---

## 0) Purpose & Scope

- Why this plan exists.  
- MUST state what stage of the project this plan covers (MVP, v1, refactor, etc.).  
- SHOULD link to related specs, ADRs, or issues.  

## 1) Objectives

- High-level outcomes this plan MUST achieve.  
- SHOULD list measurable deliverables (CLI commands implemented, tests passing, docs published).  

## 2) Milestones

- **M0 — Bootstrap:** initial repo setup, CI, templates.  
- **M1 — Core Functionality:** first working implementation of primary responsibilities.  
- **M2 — Testing & Proofs:** required test coverage and proof bundles.  
- **M3 — Packaging & Release:** distribution targets (AUR, crates.io, docs.rs).  
- **M4 — Post-Launch Maintenance:** cadence, patch process.  

(Milestones MAY be adjusted or extended.)  

## 3) Work Breakdown

Organized by **responsibility area**:

- **CLI Surface:** subcommands, argument parsing, error codes.  
- **Detection & Enforcement:** `.desktop` scanning, toolkit heuristics, wrapper generation.  
- **Observability:** logging, reports, compliance categories.  
- **Testing:** unit, integration, property tests, golden fixtures.  
- **Docs & Specs:** user docs, `.specs/` maintenance, ADRs.  

Each responsibility MUST have tasks and MAY include stretch goals.  

## 4) Dependencies & Risks

- Libraries, crates, or external tooling this plan relies on.  
- Risks (technical, organizational, licensing) that MUST be tracked.  

## 5) Resource Budgets

- Token budgets, time budgets, or iteration limits (if AI-driven).  
- Build/test timeouts and CI constraints.  

## 6) Deliverables

- What artifacts MUST exist by the end of this plan (binaries, docs, tests, configs).  
- MAY include publishing checkpoints (tags, releases).  

## 7) Maintenance & Follow-up

- Patch cadence, backlog handling.  
- SHOULD specify who owns long-term upkeep.  

## 8) Open Questions

- Unresolved design or process choices.  
- Items that SHOULD be revisited before marking the plan as Accepted.  

---
