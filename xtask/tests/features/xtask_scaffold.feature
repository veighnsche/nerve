Feature: xtask automation scaffold
  The xtask binary will eventually orchestrate formatting, linting, and distribution.
  This file captures the placeholder behaviour we will refine with concrete automation.

  @todo
  Scenario: Invoke xtask without arguments
    Given the xtask binary is scaffolded
    When xtask runs with arguments ""
    Then xtask emits scaffolding guidance
    And xtask exits cleanly
