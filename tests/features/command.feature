Feature: Command
  Scenario: Execute a command
    Given I have a command "echo Hello, World!"
    When I execute the command
    Then the stdout should be "Hello, World!"

  Scenario: Command with arguments
    Given I have a command "echo"
    And I have arguments "Hello, World!"
    When I execute the command with arguments
    Then the stdout should be "Hello, World!"

  Scenario: Command with environment variables
    Given I have a command "echo $GREETING"
    And I set environment variable GREETING to "Hello, World!"
    When I execute the command
    Then the stdout should be "Hello, World!"
