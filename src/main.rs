use cucumber::{World, gherkin::Step, given, then, when};
use std::error::Error;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};

#[derive(Debug, Default, World)]
struct CommandWorld {
    exit_status: Option<i32>,
}

#[given(expr = "a file named {string}:")]
async fn create_file(
    _world: &mut CommandWorld,
    step: &Step,
    name: String,
) -> Result<(), Box<dyn Error>> {
    File::open(name)
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
    CommandWorld::run("tests/features/readme").await;
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[tokio::test]
    async fn test() {
        for name in ["command", "exit_status", "file"] {
            CommandWorld::run(Path::new("features").join(format!("{}.feature", name))).await;
        }
    }
}
