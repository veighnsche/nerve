Feature: apply diff scaffold
  The apply primitive will eventually apply unified diffs to files. This scaffold ensures the
  placeholder advertises the contract while implementation is pending.

  Background:
    Given the apply diff scaffold is loaded

  @apply @todo
  Scenario: diff invocation reports unimplemented
    When apply.diff is invoked against "example.txt" with a unified diff
    Then the diff operation reports an unimplemented placeholder
