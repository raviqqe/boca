Feature: File
  Scenario: Create a file
    Given a file named "foo.txt" with:
      """
      foo
      """
    When I successfully run `test -r foo.txt`

  Scenario: Create a file with an escaped character
    Given a file named "foo.txt" with:
      """
      a\\b
      """
    When I successfully run `cat foo.txt`
    Then the stdout should contain exactly "a\\b"
