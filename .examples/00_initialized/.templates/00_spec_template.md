# Specification: <Project / Component Name>

Status: Draft  
Owner: @<team/maintainer>  
Date: YYYY-MM-DD  
Version: v0.x  

---

## 0) Motivation & Goals

- Concise narrative of *why this responsibility exists*.  
- State intended scope and outcomes.  
- MUST clearly identify the problems this spec addresses.  

## 1) Non-Goals

- Explicitly list what this spec does **not** cover.  
- SHOULD include adjacent responsibilities for clarity.  

## 2) Current State (Brief)

- One-page overview of the system as it exists.  
- MAY reference discovery docs, prototypes, or test reports.  

## 3) Responsibilities

- **Primary Responsibility:** What this component MUST guarantee.  
- **Secondary Responsibilities:** Behaviors it SHOULD support.  
- **Out-of-Scope:** MAY be deferred or delegated elsewhere.  

## 4) Requirements

Use **RFC-2119 keywords** (MUST, SHOULD, MAY, MUST NOT, SHOULD NOT).  

Example format:

- The system MUST provide atomic backup/restore before destructive operations.  
- The scheduler SHOULD expose fairness metrics over an API.  
- Logs MAY include optional human-readable narration, but MUST preserve machine-readable fields.  

## 5) Interfaces & Contracts

- **CLI / API / Config Surface**: All external points of responsibility.  
- Each interface MUST be described with inputs, outputs, and error cases.  
- MAY provide schemas (OpenAPI, JSON Schema, etc.).  

## 6) Observability

- Which facts, metrics, and audit events MUST be emitted.  
- SHOULD define naming conventions and severity levels.  

## 7) Security & Policy

- Enumerate what MUST be enforced (authn, authz, sandboxing).  
- SHOULD describe trust boundaries.  

## 8) Testing & Proofs

- Define how compliance will be *proven*.  
- MUST list BDD scenarios, golden fixtures, or attestation bundles.  
- MAY specify determinism or chaos tests.  

## 9) Migration & Compatibility

- How this responsibility integrates into existing systems.  
- MUST note any breaking changes and mitigations.  

## 10) Open Questions

- Tracked issues and TBD decisions.  
- SHOULD be closed before spec is promoted from Draft → Accepted.  

---
