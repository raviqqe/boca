package aruba

import (
	"bytes"
	"errors"
	"os/exec"

	"github.com/kballard/go-shellquote"
)

type command struct {
	Line   string
	Cmd    *exec.Cmd
	Stdout *bytes.Buffer
	Stderr *bytes.Buffer
}

func newCommand(line string) (*command, error) {
	ss, err := shellquote.Split(line)
	if err != nil {
		return nil, err
	} else if len(ss) == 0 {
		return nil, errors.New("empty command")
	}

	c := &command{
		Line:   line,
		Cmd:    exec.Command(ss[0], ss[1:]...),
		Stdout: bytes.NewBuffer(nil),
		Stderr: bytes.NewBuffer(nil),
	}
	c.Cmd.Stdout = c.Stdout
	c.Cmd.Stderr = c.Stderr

	return c, nil
}

func (c *command) Output() string {
	return c.Stdout.String() + c.Stderr.String()
}
