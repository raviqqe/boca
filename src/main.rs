use cucumber::{World as _, given, then, when};
use std::error::Error;
use tokio::process::Command;

#[derive(Debug, Default, cucumber::World)]
struct World {
    user: Option<String>,
    capacity: usize,
}

#[given(regex = "I run `(.*)`")]
#[when(regex = "I run `(.*)`")]
#[then(regex = "I run `(.*)`")]
async fn run_command(_world: &mut World, command: String) -> Result<(), Box<dyn Error>> {
    let command = command.split_whitespace().collect::<Vec<_>>();

    Command::new(&command[0])
        .args(&command[1..])
        .output()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    World::run("tests/features/readme").await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        World::run("tests/command.feature").await;
    }
}
