use cucumber::World;
use std::path::Path;
use tempfile::{TempDir, tempdir};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct CommandWorld {
    directory: TempDir,
    exit_status: Option<i32>,
}

impl CommandWorld {
    pub fn new() -> Self {
        Self {
            directory: tempdir().expect("test directory"),
            exit_status: None,
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
}
