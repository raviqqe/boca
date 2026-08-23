package aruba

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestEnvironmentVariableReturnsValue(t *testing.T) {
	w := world{Environment: []string{"foo=bar"}}

	assert.Equal(t, "bar", w.EnvironmentVariable("foo"))
}

func TestEnvironmentVariableReturnsLastValue(t *testing.T) {
	w := world{Environment: []string{"foo=bar", "foo=baz"}}

	assert.Equal(t, "baz", w.EnvironmentVariable("foo"))
}

func TestEnvironmentVariableReturnsNothingForUnknownName(t *testing.T) {
	w := world{Environment: []string{"foo=bar"}}

	assert.Empty(t, w.EnvironmentVariable("baz"))
}

func TestFindCommandByCommandLine(t *testing.T) {
	c := createCommand(t, "echo 'foo bar'")

	assert.Same(t, c, world{}.AddCommand(c).FindCommand("echo 'foo bar'"))
}

func TestFindCommandFindsLastCommand(t *testing.T) {
	c := createCommand(t, "echo foo")

	assert.Same(
		t,
		c,
		world{}.AddCommand(createCommand(t, "echo foo")).AddCommand(c).FindCommand("echo foo"),
	)
}

func TestFindCommandFindsNothing(t *testing.T) {
	assert.Nil(t, world{}.AddCommand(createCommand(t, "echo foo")).FindCommand("echo bar"))
}

func TestStopKillsRunningCommand(t *testing.T) {
	c := createCommand(t, "sleep 10")
	require.NoError(t, c.Cmd.Start())

	world{}.AddCommand(c).Stop()

	require.NotNil(t, c.Cmd.ProcessState)
	assert.False(t, c.Cmd.ProcessState.Success())
}

func TestStopKeepsFinishedCommand(t *testing.T) {
	c := createCommand(t, "true")
	require.NoError(t, c.Cmd.Start())
	require.NoError(t, c.Cmd.Wait())

	world{}.AddCommand(c).Stop()

	assert.True(t, c.Cmd.ProcessState.Success())
}

func TestStopIgnoresUnstartedCommand(t *testing.T) {
	c := createCommand(t, "true")

	world{}.AddCommand(c).Stop()

	assert.Nil(t, c.Cmd.ProcessState)
}
