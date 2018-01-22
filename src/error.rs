use serde_json;
use std::error::Error;
use std::fmt;
use std::io;

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Debug)]
pub struct CommandError {
    t: ErrorType,
    message: Option<String>,
}

#[derive(Debug)]
pub enum ErrorType {
    IO(io::Error),
    Serde(serde_json::Error),
}

impl Error for CommandError {
    fn description(&self) -> &str {
        &"Command Error"
    }

    fn cause(&self) -> Option<&Error> {
        match self.t {
            ErrorType::IO(ref e) => Some(e),
            ErrorType::Serde(ref e) => Some(e),
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        if let Some(ref m) = self.message {
            write!(f, "{} Cause:\n", m)?;
        }

        match self.t {
            ErrorType::IO(ref e) => write!(f, "{}", e),
            ErrorType::Serde(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(e: serde_json::Error) -> CommandError {
        CommandError {
            t: ErrorType::Serde(e),
            message: None,
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(e: io::Error) -> CommandError {
        CommandError {
            t: ErrorType::IO(e),
            message: None,
        }
    }
}
