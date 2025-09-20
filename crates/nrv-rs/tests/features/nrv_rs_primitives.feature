Feature: nrv-rs primitive coverage plan
  The nrv-rs crate exposes primitives for every nrv category. This scaffold verifies that
  each module advertises its category through `module_name()` while we grow richer APIs.

  Background:
    Given the nrv-rs primitive scaffold is loaded

  @guardrails @todo
  Scenario: Map guardrail responsibilities
    When a contributor enumerates module guardrails
    Then each primitive category is tracked for future behavior specs
    And the orchestrator bindings remain a separate concern
