Feature: Exit status
  Scenario: Execute a command
    When I run `echo hello world`
    Then the exit status should be 0
