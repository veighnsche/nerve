# nrv.llm::request::input

## Purpose
Captures the prompt or structured input sent to the model. Keeps all instructions explicit and
version-controlled.

## Behaviour Expectations
- Accepts strings, structured prompts, or tokenized inputs depending on transport.
- MUST NOT modify or inject additional instructions; caller owns exact prompt text.
- Should support attaching response contracts (e.g., from `nrv.match.compileOneOf`).

## Open Questions
- How do we represent multi-part inputs (system/user/tool messages) in a deterministic structure?
- Should we provide helpers for chunking or stitching long prompts, or leave that to `nrv.ctx`?
