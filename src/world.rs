use cucumber::{World, gherkin::Step, given, then, when};
use std::error::Error;
use tempfile::{TempDir, tempdir};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command};

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
}
