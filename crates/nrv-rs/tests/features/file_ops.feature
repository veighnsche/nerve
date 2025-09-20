Feature: nrv.file scaffold
  The file primitives will eventually support read/write/stat operations. Until implemented, they
  should surface clear placeholders so callers know to supply their own wiring.

  Background:
    Given the file primitive scaffold is loaded

  @file @todo
  Scenario: Read reports unimplemented
    When nrv.file.read is invoked for "example.txt"
    Then the file operation reports an unimplemented placeholder
