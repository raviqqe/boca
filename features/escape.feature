Feature: Character escape
  Scenario: Create a file with an escaped backslash
    Given a file named "foo.txt" with:
      """
      a\\b
      """
    When I successfully run `cat foo.txt`
    Then the stdout should contain exactly "a\\b"

  Scenario: Create a file with an escaped double quote
    Given a file named "foo.txt" with:
      """
      a\"b
      """
    When I successfully run `cat foo.txt`
    Then the stdout should contain exactly "a\"b"

  Scenario: Check stdout with a blank character
    When I successfully run `echo \\\\\\\\`
    Then the stdout should contain exactly "\\\\"
