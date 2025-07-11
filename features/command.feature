Feature: Command
  Scenario: Execute a command
    When I run `echo hello world`

  Scenario: Execute a command
    When I successfully run `echo hello world`

  Scenario: Check an exit status
    When I run `true`
    Then the exit status should be 0

  Scenario: Check an exit status
    When I run `false`
    Then the exit status should be 1
