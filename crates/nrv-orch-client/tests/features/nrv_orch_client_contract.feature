Feature: nrv orchestrator client contract
  The orchestrator bindings must expose capabilities, queueing, streaming, cancellation, and
  structured errors. These scenarios pin the data-shape expectations before the transport lands.

  Background:
    Given an orchestrator binding contract draft

  @capabilities @todo
  Scenario: Capabilities handshake sketch
    When capabilities are requested
    Then the client tracks capability constraints for later assertions
    And capability metadata includes optional fields

  @queue @stream @cancel @todo
  Scenario: Queue and stream lifecycle sketch
    When a task is enqueued with model "foundation-model"
    And the orchestrator begins streaming events
    And task "task-123" is cancelled
    Then the BDD scaffold records pending stream and cancel flows
    And task cancellation remains a first-class contract

  @errors @todo
  Scenario: Structured error envelope sketch
    When the orchestrator reports error code "E429" with message "Too many requests"
    And the error is flagged retriable after 1000 ms
    Then the structured error is exposed to callers
