use cucumber::Parameter;
use derive_more::{Deref, FromStr};
use std::error::Error;

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"`(.*)`", name = "command")]
pub struct CommandString(String);

impl CommandString {
    pub fn command(&self) -> &str {
        &self.0
    }
}

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"successfully |", name = "successfully")]
pub struct Successfully(String);

impl Successfully {
    pub fn successfully(&self) -> bool {
        self.0.trim() == "successfully"
    }
}

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"stdin|stdout|stderr", name = "stdio")]
pub struct StdioName(String);

impl StdioName {
    pub fn kind(&self) -> Result<StdioType, Box<dyn Error>> {
        Ok(match self.0.as_str() {
            "stdin" => StdioType::Stdin,
            "stdout" => StdioType::Stdout,
            "stderr" => StdioType::Stderr,
            name => return Err(format!("invalid stdio name: {name}").into()),
        })
    }
}

pub enum StdioType {
    Stdin,
    Stdout,
    Stderr,
}

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"exactly |", name = "exactly")]
pub struct Exactly(String);

impl Exactly {
    pub fn exactly(&self) -> bool {
        self.0.trim() == "exactly"
    }
}

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"not |", name = "not")]
pub struct Not(String);

impl Not {
    pub fn not(&self) -> bool {
        self.0.trim() == "not"
    }
}
