use cucumber::{World, given, then, when};
use std::error::Error;
use tokio::process::Command;

#[derive(Debug, Default, World)]
struct CommandWorld {
    exit_status: Option<i32>,
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
    use super::*;

    #[tokio::test]
    async fn test() {
        CommandWorld::run("tests/command.feature").await;
    }
}
