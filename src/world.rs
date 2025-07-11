use cucumber::World;
use std::path::Path;
use tempfile::{TempDir, tempdir};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct CommandWorld {
    directory: TempDir,
    exit_status: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl CommandWorld {
    pub fn new() -> Self {
        Self {
            directory: tempdir().expect("test directory"),
            exit_status: None,
            stdout: Default::default(),
            stderr: Default::default(),
        }
    }

    pub fn directory(&self) -> &Path {
        self.directory.path()
    }

    pub fn exit_status(&self) -> Option<i32> {
        self.exit_status
    }

    pub fn set_exit_status(&mut self, status: Option<i32>) {
        self.exit_status = status;
    }

    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    pub fn set_stdout(&mut self, output: Vec<u8>) {
        self.stdout = output;
    }

    pub fn stderr(&self) -> &[u8] {
        &self.stderr
    }

    pub fn set_stderr(&mut self, error: Vec<u8>) {
        self.stderr = error;
    }
}
