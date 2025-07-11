mod world;

use cucumber::{Parameter, World, gherkin::Step, given, then, when};
use derive_more::{Deref, FromStr};
use memchr::memmem;
use std::error::Error;
use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command};
use world::CommandWorld;

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"`.*`", name = "command")]
struct CommandString(String);

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"successfully |", name = "successfully")]
struct Successfully(String);

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"stdin|stdout|stderr", name = "stdio")]
struct Stdio(String);

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"exactly |", name = "exactly")]
struct Exactly(String);

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

#[when(expr = "I {successfully}run {command}")]
async fn run_command(
    world: &mut CommandWorld,
    _successfully: Successfully,
    command_string: CommandString,
) -> Result<(), Box<dyn Error>> {
    let command = command_string.0.split_whitespace().collect::<Vec<_>>();

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

#[then(expr = "the {stdio} should contain {exactly}{string}")]
async fn check_stdio(
    world: &mut CommandWorld,
    stdio: Stdio,
    exactly: Exactly,
    expected_output: String,
) -> Result<(), Box<dyn Error>> {
    let output = match stdio.0.as_str() {
        "stdout" => world.stdout(),
        "stderr" => world.stderr(),
        _ => return Err("invalid stdio type".into()),
    };
    let expected_output = expected_output.as_bytes();

    if &exactly.0 == "exactly" {
        assert_eq!(output, expected_output);
    } else {
        assert!(memmem::find(output, expected_output).is_some());
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
