Feature: nrv.llm scaffold
  The llm client coordinates requests, streaming, and tool calls. Until implemented it should
  surface a clear placeholder so callers know the API contract is pending.

  Background:
    Given the llm client scaffold is loaded

  @llm @todo
  Scenario: Client construction reports unimplemented
    When nrv.llm.client is requested with default config
    Then the llm client operation reports an unimplemented placeholder
