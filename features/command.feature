Feature: Command
  Scenario: Execute a command
    When I run `echo hello world`

  Scenario: Check an exit status
    When I run `echo hello world`
    Then the exit status should be 0
