Feature: nrv CLI scaffolding
  The micro-CLI wraps library surfaces. These scenarios pin the behaviours we expect to
  exercise once the specs graduate from stubs.

  @version
  Scenario: Print CLI version
    Given the nrv CLI binary is available
    When the CLI is invoked with arguments "--version"
    Then the CLI responds without crashing
    And the exit code is 0
    And stdout prints the CLI version

  @usage
  Scenario: Show help when requested
    Given the nrv CLI binary is available
    When the CLI is invoked with arguments "--help"
    Then the CLI responds without crashing
    And the exit code is 0
    And stderr mentions "Usage"

  @unknown
  Scenario: Unknown subcommand reports usage
    Given the nrv CLI binary is available
    When the CLI is invoked with arguments "launch"
    Then the CLI responds without crashing
    And the exit code is 2
    And stderr mentions "Unknown command"

  @sync_capabilities @todo
  Scenario: Sync capabilities placeholder
    Given the nrv CLI binary is available
    When the CLI is invoked with arguments "sync-capabilities"
    Then the CLI responds without crashing
    And the exit code is 0
    And stdout contains "sync-capabilities: TODO"
