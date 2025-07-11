Feature: Command
  Scenario: Execute a command
    Given I run `echo hello world`
    When I execute the command
    Then the stdout should be "Hello, World!"
