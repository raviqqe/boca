Feature: File
  Scenario: Create a file
    Given a file named "foo.txt" with:
      """
      foo
      """
    When I successfully run `test -r foo.txt`
