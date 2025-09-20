# ADR 009: Semantic Pattern Matcher (OneOf / ManyOf)

## Status

- Accepted

## Context

Nerve workflows often need to branch: e.g., “next step is `spec_ok` vs. `needs_scaffold` vs. `revise_intent`.”  
Early attempts at designing a matcher bundled too many responsibilities (LLM call + schema validation + routing + calibration), which violated ADR 008’s Anti-Insanity Clause.  
We need a **minimal, pure, explicit matcher** that helps repos structure and validate classification decisions, without hiding intent or polluting prompts.

## Decision

- **Core matcher responsibilities**:
  - Produce a deterministic **response contract** snippet (footer) to append to prompts.
  - Validate JSON returned by the LLM against a closed set of labels (and optional per-label schemas).
  - Provide a simple helper to route on labels.

- **Out of scope**:
  - The matcher MUST NOT call LLMs.
  - The matcher MUST NOT invent “confidence” or thresholds.
  - The matcher MUST NOT modify user instructions.

### API

```ts
// Compile a JSON response contract footer
nrv.match.compileOneOf({
  labels: string[],                  // required, closed set
  abstainLabel?: string,             // optional “none of the above”
  dataSchemas?: Record<string, any>  // optional per-label JSON Schema
}) -> string

// Validate an LLM JSON result
nrv.match.validateOneOf(json: any, {
  labels: string[],
  abstainLabel?: string,
  dataSchemas?: Record<string, any>,
  strict?: boolean                   // default true: throw on invalid/out-of-set
}) -> { label: string, data?: any }

// Convenience router (pure function)
nrv.match.route(decision, handlers: Record<string, (arg:any)=>any|Promise<any>>)
```

### Example Usage

```ts
const labels = ["spec_ok","needs_scaffold","revise_intent","out_of_scope"];
const contract = nrv.match.compileOneOf({ labels, abstainLabel: "out_of_scope" });

const prompt = [
  "== INSTRUCTION ==\nPick the best next action.",
  "\n\n== TEXT ==\n", repoSummary,
  "\n\n== RESPONSE ==\n", contract
];

const out = await nrv.llm({ model, input: prompt }).run();
const decision = nrv.match.validateOneOf(out.asJson(), { labels, abstainLabel: "out_of_scope" });

await nrv.match.route(decision, {
  spec_ok:        () => proceed(),
  needs_scaffold: ({data}) => scaffold(data.files),
  revise_intent:  () => openEditor("Nerve.md"),
  out_of_scope:   () => nrv.ui.step("noop").ok("Skipping")
});
```

## Consequences

### Pros

- Keeps matcher pure and minimal (no hidden LLM calls).
- Prompts remain user-authored; contract footer is small and snapshot-testable.
- Validation enforces closed-set determinism, preventing spurious labels.
- Routing is explicit and auditable.

### Cons

- Users must run the LLM themselves and pass results back into `validateOneOf`.
- No built-in scoring/thresholds; repos must define these in userland if desired.
- Slightly more boilerplate than a “magic” matcher, but simpler to reason about.

### Neutral / Notes

- Future `nrv.match.compileManyOf/validateManyOf` MAY be added with the same philosophy (`{"labels":["…"]}`).
- Calibration, scoring, and hierarchical decision trees remain userland responsibilities.

## Alternatives Considered

- **LLM-calling matcher** (bindLLM/fromText): rejected, violated ADR 002 and ADR 008.
- **Confidence/threshold in core**: rejected, pollutes prompts and hides intent.
- **No matcher at all**: rejected, too much copy-paste boilerplate across repos.

## References

- ADR 002: User-Defined LLM Interactions Only
- ADR 003: Single `nrv` Object Injection
- ADR 008: Anti-Insanity Clause
