use std::fmt;
use std::io::{self};

#[derive(Debug)]
pub enum CommandError {
	Io(io::Error),
	ExecutionError(String),
}

impl From<io::Error> for CommandError {
	fn from(error: io::Error) -> Self {
		CommandError::Io(error)
	}
}

impl fmt::Display for CommandError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CommandError::Io(err) => write!(f, "IO error: {}", err),
			CommandError::ExecutionError(err) => write!(f, "Execution error: {}", err),
		}
	}
}

impl std::error::Error for CommandError {}
