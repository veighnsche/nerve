Nerve — LLM Client Knowledge Bundle (for IDE AI)

Purpose: Give the IDE AI everything it needs to implement/modify code around nrv.llm without inventing behavior. This bundle distills the binding rules from the Constitution + ADRs and freezes an operable interface and examples.

0) Ground Truth (Citations)
	•	User-defined LLM only; no hidden policy; library-first; micro-CLI is non-runtime.
	•	Networking contract: capabilities, enqueue, stream, cancel. Structured errors, SSE-style stream, proof-friendly.
	•	UI primitives (step, prompt, diff, progress) are presentation only; must degrade to JSON for CI. No console hijack; defaults explicit.

⸻

1) Lifecycle — 3 Steps (Do Not Deviate)

(1) Discover → nrv.llm.capabilities() returns opaque model IDs and limits advertised by the orchestrator.
(2) Submit → nrv.llm.enqueue({ model, prompt, … }) with a user-authored prompt and optional response contract.
(3) Stream → for await (evt of nrv.llm.stream(job.id)) { … } until done. Optional cancel(job.id).
Constraints: No retries/fallbacks, no truncation heuristics, no narration synthesis in core.

⸻

2) Canonical JS/TS Surface (Contract)

// Types for the IDE AI to target. Keep surface minimal and explicit.
export interface LlmCapabilities {
  models: Array<{ id: string; family: string; context_tokens: number; features: string[] }>;
  server_version: string;
  limits: Record<string, number>;
}

export interface EnqueueOptions {
  /** Opaque model id exactly as advertised by capabilities() (e.g., "hf:org/model@quant") */
  model: string;
  /** Entirely user-authored; core MUST NOT rewrite or prepend/append hidden prompts */
  prompt: string;
  /** Optional explicit response schema/contract (JSON Schema or DSL) */
  response_contract?: object;
  /** Optional sidecars (kept opaque; suitable for Proof Bundle attachments) */
  attachments?: Record<string, Blob>;
  /** Opaque, userland-only */
  metadata?: Record<string, unknown>;
}

export interface Job { id: string; model: string; enqueued_at: string; }

export type StreamEvent =
  | { type: "token"; text: string }
  | { type: "log"; message: string }
  | { type: "narration"; text: string }   // passed through from orchestrator only
  | { type: "error"; code: string; message: string; data?: any }
  | { type: "done" };

export const nrv = {
  llm: {
    capabilities(): Promise<LlmCapabilities>,
    enqueue(opts: EnqueueOptions): Promise<Job>,
    stream(jobId: string): AsyncIterable<StreamEvent>,
    cancel(jobId: string): Promise<void>,
  }
};

Model identifiers are opaque text (hf:…, local:…, openai:…, vllm:…) and MUST NOT be parsed or normalized by core; they come from capabilities().

⸻

3) Canonical Rust Surface (Symmetry)

pub struct LlmCapabilities {
    pub models: Vec<ModelInfo>,
    pub server_version: String,
    pub limits: std::collections::HashMap<String, u64>,
}
pub struct ModelInfo { pub id: String, pub family: String, pub context_tokens: u32, pub features: Vec<String> }

pub struct EnqueueOptions {
    pub model: String, pub prompt: String,
    pub response_contract: Option<serde_json::Value>,
    pub attachments: Option<std::collections::HashMap<String, Vec<u8>>>,
    pub metadata: Option<serde_json::Value>,
}
pub struct Job { pub id: String, pub model: String, pub enqueued_at: chrono::DateTime<chrono::Utc> }

pub enum StreamEvent {
    Token { text: String },
    Log { message: String },
    Narration { text: String },
    Error { code: String, message: String, data: Option<serde_json::Value> },
    Done,
}

⸻

4) End-to-End Example (TS) — Fully Filled

// 1) Discover
const caps = await nrv.llm.capabilities();
const model = caps.models.find(m => m.id.startsWith("hf:"))?.id
  ?? caps.models[0]?.id; // no policy, just a dumb pick

// 2) Enqueue
const job = await nrv.llm.enqueue({
  model,
  prompt: [
    "You are an AI proof assistant. Summarize ADR-013 in <=80 words.\n",
    readFile(".adr/013_UI_interaction_primitives.md")
  ].join("\n"),
  response_contract: {
    type: "object",
    properties: { summary: { type: "string" } },
    required: ["summary"]
  },
  attachments: {
    "adr-013.md": new Blob([readFile(".adr/013_UI_interaction_primitives.md")], { type: "text/markdown" })
  },
  metadata: { request_id: "req-42", repo: "nerve" }
});

// 3) Stream
let out = "";
for await (const evt of nrv.llm.stream(job.id)) {
  if (evt.type === "token") out += evt.text;
  else if (evt.type === "narration") console.log("[narration]", evt.text);
  else if (evt.type === "log") console.log("[log]", evt.message);
  else if (evt.type === "error") throw new Error(`${evt.code}: ${evt.message}`);
  else if (evt.type === "done") break;
}

Proof Bundle guidance: log an llm.enqueue row with model, prompt, response_contract hash, attachment names, and metadata. Flush deterministically; redact secrets.

⸻

5) Error Semantics (Map to ADR-011)

Return structured errors only ({ code, message, data? }). No implicit retries/backoff; the caller owns policy. Examples:
	•	ERR_INVALID_MODEL — model id not present in capabilities().
	•	ERR_CONTEXT_BUDGET — request exceeds server/context limits (fail fast unless caller overrides).
	•	ERR_STREAM_DISCONNECTED — broken SSE/stream transport (no auto-retry).
	•	ERR_SERVER — pass-through structured server error.

⸻

6) UI Usage Around LLM (Presentation-Only)
	•	Prefer nrv.ui.step("…") for sectioned logs, not for gating. Use prompt.confirm({ label, default }) if you need a human decision.
	•	All UI must degrade to JSON for CI; no console hijack; defaults explicit if not awaited.

Anti-Insanity guardrails: no hidden behavior, no prompt pollution, minimal surfaces, explainable in < 90 seconds.

⸻

7) “Do / Don’t” for the IDE AI

Do
	•	Use only capabilities → enqueue → stream/cancel.
	•	Treat model as opaque.
	•	Keep prompts 100% user-authored.
	•	Emit Proof Bundle rows immediately and durably; redact secrets.
	•	Surface orchestrator narration verbatim (if present); never synthesize.

Don’t
	•	Don’t add retries, truncations, or routing heuristics.
	•	Don’t modify prompts or inject “system” policy.
	•	Don’t parse model strings to guess engines.
	•	Don’t make step a gate; use prompt.confirm for gates.

⸻

8) Quick BDD Hooks (for your test harness)

Feature: LLM job lifecycle
  Scenario: Discover -> Enqueue -> Stream to completion
    Given the orchestrator advertises a model "hf:*"
    When I enqueue a job with that model and a JSON schema contract
    Then I should receive a stream of token events and a final "done"
    And no "error" events MUST occur

Contract checks:
	•	capabilities.models[*].id is non-empty string.
	•	enqueue returns { id, model, enqueued_at }.
	•	Stream emits at least one { type: "token"|"done" } and ends with "done" (or explicit "error").

⸻

9) Implementation Notes (Non-Normative)
	•	The orchestrator client will likely be a thin HTTP/SSE wrapper. Keep it side-effect free.
	•	Attachments: send as multipart or pre-upload; record references (names/sha256) in Proofs.
	•	Timeouts/cancellation live at the caller; cancel(jobId) maps to server cancel endpoint.

⸻

If you want, I can also generate a sibling file .knowledge/ide_ai/adr_014a_llm_client_signature.md that freezes this surface as a formal extension to ADR-014.
