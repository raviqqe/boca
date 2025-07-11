mod world;

use cucumber::{World, gherkin::Step, given, then, when};
use std::error::Error;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command};
use world::CommandWorld;

#[given(expr = "a file named {string} with:")]
async fn create_file(
    world: &mut CommandWorld,
    step: &Step,
    name: String,
) -> Result<(), Box<dyn Error>> {
    OpenOptions::default()
        .create(true)
        .write(true)
        .open(world.directory().join(name))
        .await?
        .write_all(step.docstring.as_ref().expect("file content").as_bytes())
        .await?;

    Ok(())
}

#[when(regex = "I run `(.*)`")]
async fn run_command(world: &mut CommandWorld, command: String) -> Result<(), Box<dyn Error>> {
    let command = command.split_whitespace().collect::<Vec<_>>();

    let output = Command::new(command[0])
        .args(&command[1..])
        .output()
        .await?;

    world.set_exit_status(output.status.code());

    Ok(())
}

#[then(expr = "the exit status should be {int}")]
async fn check_exit_status(world: &mut CommandWorld, status: i32) -> Result<(), Box<dyn Error>> {
    assert_eq!(world.exit_status(), Some(status));

    Ok(())
}

#[tokio::main]
async fn main() {
    CommandWorld::run("features").await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn run_features() {
        CommandWorld::run(Path::new("features")).await;
    }
}
