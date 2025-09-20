# Step::ok

## Signature
`fn ok(&self, msg: Option<impl Into<String>>) -> Step`

## Purpose
Appends a success marker (`StepEvent::Ok`) with an optional message, producing a new `Step` instance
so callers can continue chaining events immutably.

## Behaviour
- Converts the optional message into an owned `String` if provided; otherwise stores `None`.
- Leaves prior events intact and preserves ordering guarantees.

## Open Questions
- Should success events default to a standard message if `None` is provided, or remain empty?
- Will we need to attach structured payloads (e.g., metrics) to success markers later on?
