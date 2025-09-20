# nrv.orch::TaskRequest

## Structure
```
pub struct TaskRequest {
    pub model: String,
}
```

## Purpose
Submission payload for enqueuing a task with the orchestrator. Currently only captures the `model`
identifier; future expansions will carry prompt payloads, guardrail options, and context budgeting
options.

## Open Questions
- How should prompts, attachments, and guardrail overrides be represented?
- Do we need to version the request payload to manage orchestrator compatibility?
