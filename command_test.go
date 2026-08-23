package aruba

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func createCommand(t *testing.T, line string) *command {
	t.Helper()

	c, err := newCommand(line)
	require.NoError(t, err)

	return c
}

func TestNewCommandSplitsArguments(t *testing.T) {
	assert.Equal(t, []string{"echo", "foo", "bar"}, createCommand(t, "echo foo bar").Cmd.Args)
}

func TestNewCommandKeepsQuotedArgument(t *testing.T) {
	c := createCommand(t, "echo 'foo bar'")

	assert.Equal(t, "echo 'foo bar'", c.Line)
	assert.Equal(t, []string{"echo", "foo bar"}, c.Cmd.Args)
}

func TestNewCommandFailsWithEmptyLine(t *testing.T) {
	_, err := newCommand("")

	assert.Error(t, err)
}

func TestNewCommandFailsWithUnterminatedQuote(t *testing.T) {
	_, err := newCommand("echo 'foo")

	assert.Error(t, err)
}
