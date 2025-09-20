# nrv.llm::response::text

## Purpose
Returns the raw text response from the model exactly as delivered by the transport.

## Behaviour Expectations
- No trimming, post-processing, or hidden summarisation.
- Optional helpers may strip known response contract footers when explicitly requested.
- Errors should surface when transports deliver malformed or truncated payloads.

## Open Questions
- Do we need additional helpers for extracting tool directives or citations from text responses?
- Should there be a convenience method to pair raw text with the originating prompt for logging?
