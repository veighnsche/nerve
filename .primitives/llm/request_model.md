# nrv.llm::request::model

## Purpose
Specifies which engine/model to invoke. Keeps model selection explicit, no hidden defaults.

## Behaviour Expectations
- Accepts a string identifier or typed enum representing the model.
- Must be supplied explicitly—no fallback to env vars without caller opting in.
- Should integrate with capabilities metadata (`nrv.orch::Capabilities`) to validate availability.

## Open Questions
- Should model selection include version pinning and fallback logic?
- How do we reconcile direct transport models vs orchestrator-managed aliases?
