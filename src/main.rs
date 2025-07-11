mod parameter;
mod world;

use cucumber::{World, gherkin::Step, given, then, when};
use parameter::{CommandString, Exactly, Not, StdioName, StdioType, Successfully};
use std::{error::Error, str};
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

#[when(expr = "I {successfully}run {command}")]
async fn run_command(
    world: &mut CommandWorld,
    successfully: Successfully,
    command_string: CommandString,
) -> Result<(), Box<dyn Error>> {
    let command = command_string
        .command()
        .split_whitespace()
        .collect::<Vec<_>>();

    let output = Command::new(command[0])
        .args(&command[1..])
        .current_dir(world.directory())
        .output()
        .await?;

    if successfully.successfully() && !output.status.success() {
        return Err(format!(
            "invalid command status {}",
            output.status.code().unwrap_or(-1),
        )
        .into());
    }

    world.set_exit_status(output.status.code());
    world.set_stdout(output.stdout);
    world.set_stderr(output.stderr);

    Ok(())
}

#[then(expr = "the exit status should {not}be {int}")]
async fn check_exit_status(
    world: &mut CommandWorld,
    not: Not,
    status: i32,
) -> Result<(), Box<dyn Error>> {
    if not.not() {
        assert_ne!(world.exit_status(), Some(status))
    } else {
        assert_eq!(world.exit_status(), Some(status))
    }

    Ok(())
}

#[then(expr = "the {stdio} should contain {exactly}{string}")]
async fn check_stdio(
    world: &mut CommandWorld,
    stdio: StdioName,
    exactly: Exactly,
    expected_output: String,
) -> Result<(), Box<dyn Error>> {
    let output = str::from_utf8(match stdio.kind()? {
        StdioType::Stdout => world.stdout(),
        StdioType::Stderr => world.stderr(),
        StdioType::Stdin => return Err("invalid stdin for output".into()),
    })?;

    if exactly.exactly() {
        assert_eq!(output.trim(), expected_output.trim());
    } else {
        assert!(output.contains(&expected_output));
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
