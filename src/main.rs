use cucumber::{World as _, given, then, when};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Default, cucumber::World)]
struct World {
    user: Option<String>,
    capacity: usize,
}

#[given(expr = "{word} is hungry")]
async fn someone_is_hungry(world: &mut World, user: String) {
    sleep(Duration::from_secs(2)).await;

    world.user = Some(user);
}

#[when(regex = r"^(?:he|she|they) eats? (\d+) cucumbers?$")]
async fn eat_cucumbers(world: &mut World, count: usize) {
    sleep(Duration::from_secs(2)).await;

    world.capacity += count;

    assert!(
        world.capacity < 4,
        "{} exploded!",
        world.user.as_ref().unwrap()
    );
}

#[then("she is full")]
async fn is_full(world: &mut World) {
    sleep(Duration::from_secs(2)).await;

    assert_eq!(
        world.capacity,
        3,
        "{} isn't full!",
        world.user.as_ref().unwrap()
    );
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
        World::run("tests/features/command.feature").await;
    }
}
