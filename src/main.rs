mod world;

use cucumber::{Parameter, World, gherkin::Step, given, then, when};
use derive_more::{Deref, FromStr};
use memchr::memmem;
use std::error::Error;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command};
use world::CommandWorld;

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"stdin|stdout|stderr", name = "stdio")]
struct Stdio(String);

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
    world.set_stdout(output.stdout);
    world.set_stderr(output.stderr);

    Ok(())
}

#[then(expr = "the exit status should be {int}")]
async fn check_exit_status(world: &mut CommandWorld, status: i32) -> Result<(), Box<dyn Error>> {
    assert_eq!(world.exit_status(), Some(status));

    Ok(())
}

#[then(expr = "the {stdio} should contain {string}")]
async fn check_stdio(
    world: &mut CommandWorld,
    stdio: Stdio,
    output: String,
) -> Result<(), Box<dyn Error>> {
    match stdio.0.as_str() {
        "stdout" => assert!(memmem::find(world.stdout(), output.as_bytes()).is_some()),
        "stderr" => assert!(memmem::find(world.stderr(), output.as_bytes()).is_some()),
        _ => return Err("invalid stdio type".into()),
    }

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
