use cucumber::{World, gherkin::Step, given, then, when};
use std::error::Error;
use tempfile::{TempDir, tempdir};
use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command};

#[derive(Debug, World)]
#[world(init = Self::new)]
struct CommandWorld {
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

#[given(expr = "a file named {string}:")]
async fn create_file(
    world: &mut CommandWorld,
    step: &Step,
    name: String,
) -> Result<(), Box<dyn Error>> {
    OpenOptions::default()
        .create(true)
        .write(true)
        .open(world.directory.path().join(name))
        .await?
        .write_all(&step.docstring.as_ref().expect("file content").as_bytes())
        .await?;

    Ok(())
}

#[when(regex = "I run `(.*)`")]
async fn run_command(world: &mut CommandWorld, command: String) -> Result<(), Box<dyn Error>> {
    let command = command.split_whitespace().collect::<Vec<_>>();

    let output = Command::new(&command[0])
        .args(&command[1..])
        .output()
        .await?;

    world.exit_status = output.status.code();

    Ok(())
}

#[then(expr = "the exit status should be {int}")]
async fn check_exit_status(world: &mut CommandWorld, status: i32) -> Result<(), Box<dyn Error>> {
    assert_eq!(world.exit_status, Some(status));

    Ok(())
}

#[tokio::main]
async fn main() {
    CommandWorld::cucumber().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test() {
        for name in ["command", "exit_status", "file"] {
            CommandWorld::run(Path::new("features").join(format!("{}.feature", name))).await;
        }
    }
}
