use log::debug;
use std::process::{Command, Output, Stdio};

use crate::CommandError;

pub struct CommandExecutor;

impl CommandExecutor {
	pub fn run_command(cmd: Vec<&str>) -> Result<Output, CommandError> {
		debug!("Running raw command: {:?}", cmd);
		let output = Command::new(cmd[0]).args(&cmd[1..]).stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;

		if !output.status.success() {
			let err_message = String::from_utf8_lossy(&output.stderr).to_string();
			return Err(CommandError::ExecutionError(err_message));
		}

		Ok(output)
	}
}
