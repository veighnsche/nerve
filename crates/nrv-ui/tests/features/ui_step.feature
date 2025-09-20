Feature: nrv-ui step applet behaviour
  The step applet records caller-authored narration without side effects so Proof Bundles
  and renderers can inspect the collected events.

  Background:
    Given the ui module sentinel is registered

  @step @todo
  Scenario: Record narration success path
    When a narration step is created with label "scaffold task"
    And the narration step records info "starting"
    And the narration step records ok "done"
    Then the module name reports "ui"
    And the narration events include info "starting"
    And the narration events include ok "done"

  @step @todo
  Scenario: Record narration failure path
    When a narration step is created with label "scaffold task"
    And the narration step records fail "blocked"
    Then the narration events include fail "blocked"
