package aruba

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type worldKey struct{}

type world struct {
	commands         []command
	RootDirectory    string
	CurrentDirectory string
	Stdin            io.WriteCloser
	Environment      []string
	StartupWaitTime  time.Duration
}

func newWorld(d string) world {
	return world{
		RootDirectory:    d,
		CurrentDirectory: d,
		Environment:      os.Environ(),
	}
}

func (w world) AddCommand(c command) world {
	w.commands = append(w.commands, c)
	return w
}

func (w world) FindCommand(s string) *command {
	for _, c := range w.commands {
		if s == c.Line {
			return &c
		}
	}

	return nil
}

func (w world) LastCommand() command {
	return w.commands[len(w.commands)-1]
}

func (w world) EnvironmentVariable(name string) string {
	for i := len(w.Environment) - 1; i >= 0; i-- {
		if k, v, ok := strings.Cut(w.Environment[i], "="); ok && k == name {
			return v
		}
	}

	return ""
}

func (w world) Stop() {
	for _, c := range w.commands {
		if c.Cmd.Process != nil && c.Cmd.ProcessState == nil {
			_ = c.Cmd.Process.Kill()
			_ = c.Cmd.Wait()
		}
	}
}

func (w world) Stdout() string {
	return w.output(func(c command) *bytes.Buffer {
		return c.Stdout
	})
}

func (w world) Stderr() string {
	return w.output(func(c command) *bytes.Buffer {
		return c.Stderr
	})
}

func (w world) Output() string {
	bs := []byte(nil)

	for _, c := range w.commands {
		_ = c.Cmd.Wait()
		bs = append(bs, c.Output()...)
	}

	return string(bs)
}

func (w world) path(p string) (string, error) {
	if filepath.IsAbs(p) || filepath.VolumeName(p) != "" {
		return "", fmt.Errorf("path %q must be relative to the working directory", p)
	}

	q := filepath.Join(w.CurrentDirectory, p)

	if d, err := filepath.Rel(w.RootDirectory, q); err != nil {
		return "", err
	} else if strings.HasPrefix(d, "..") {
		return "", fmt.Errorf("path %q is outside the working directory", p)
	}

	return q, nil
}

func (w world) output(f func(command) *bytes.Buffer) string {
	bs := []byte(nil)

	for _, c := range w.commands {
		_ = c.Cmd.Wait()
		bs = append(bs, f(c).Bytes()...)
	}

	return string(bs)
}

func contextWorld(ctx context.Context) world {
	return ctx.Value(worldKey{}).(world)
}

func contextWithWorld(ctx context.Context, w world) context.Context {
	return context.WithValue(ctx, worldKey{}, w)
}
