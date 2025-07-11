use cucumber::{World as _, given};
use tokio::process::Command;

#[derive(Debug, Default, cucumber::World)]
struct World {
    user: Option<String>,
    capacity: usize,
}

#[given(expr = "I run \"{command}\"")]
async fn run_command(world: &mut World, command: String) {
    Command::new(command).run().await;
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
