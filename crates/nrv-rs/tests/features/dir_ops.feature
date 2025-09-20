Feature: nrv.dir scaffold
  Directory helpers will eventually manage listings and creation. The scaffold keeps behaviour
  explicit until the full engine lands.

  Background:
    Given the directory primitive scaffold is loaded

  @dir @todo
  Scenario: List reports unimplemented
    When nrv.dir.list is invoked for "."
    Then the directory operation reports an unimplemented placeholder
