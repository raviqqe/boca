Feature: File
  Scenario: Create a file
    Given a file named "foo.txt" with:
      """
      foo
      """
    When I successfully run `test -r foo.txt`

  Scenario: Create a file with a content type
    Given a file named "foo.txt" with:
      """foo
      foo
      """
    When I successfully run `cat foo.txt`
    Then the stdout should contain exactly "foo"
