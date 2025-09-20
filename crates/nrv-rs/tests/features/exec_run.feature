Feature: nrv.exec scaffold
  Command execution helpers guard external tooling. The placeholder documents intent until the
  runtime lands.

  Background:
    Given the exec primitive scaffold is loaded

  @exec @todo
  Scenario: Run reports unimplemented
    When nrv.exec.run is invoked for program "echo"
    Then the exec operation reports an unimplemented placeholder
