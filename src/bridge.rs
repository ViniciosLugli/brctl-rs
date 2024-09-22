use crate::{BridgeController, CommandError, CommandExecutor};
use lazy_regex::regex_captures;
use log::{debug, error, info};
use std::fmt;

#[derive(Debug)]
pub struct Bridge {
	name: String,
}

impl fmt::Display for Bridge {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl Bridge {
	pub fn new(name: &str) -> Bridge {
		info!("Creating new Bridge: {}", name);
		Bridge { name: name.to_string() }
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}

	pub fn delete(&self) -> Result<(), CommandError> {
		info!("Deleting Bridge: {}", self.name);
		BridgeController::delete_bridge(&self.name)
	}

	pub fn add_interface(&self, interface: &str) -> Result<(), CommandError> {
		info!("Adding interface {} to Bridge: {}", interface, self.name);
		CommandExecutor::run_command(vec!["brctl", "addif", &self.name, interface]).map(|_| ())
	}

	pub fn remove_interface(&self, interface: &str) -> Result<(), CommandError> {
		info!("Removing interface {} from Bridge: {}", interface, self.name);
		CommandExecutor::run_command(vec!["brctl", "delif", &self.name, interface]).map(|_| ())
	}

	pub fn set_hairpin(&self, port: &str, enable: bool) -> Result<(), CommandError> {
		let state = if enable { "on" } else { "off" };
		info!("Setting hairpin {} on port {} for Bridge: {}", state, port, self.name);
		CommandExecutor::run_command(vec!["brctl", "hairpin", &self.name, port, state]).map(|_| ())
	}

	pub fn set_stp(&self, enable: bool) -> Result<(), CommandError> {
		let state = if enable { "on" } else { "off" };
		info!("Setting STP {} for Bridge: {}", state, self.name);
		CommandExecutor::run_command(vec!["brctl", "stp", &self.name, state]).map(|_| ())
	}

	pub fn set_ageing_time(&self, time: u32) -> Result<(), CommandError> {
		info!("Setting ageing time to {} for Bridge: {}", time, self.name);
		CommandExecutor::run_command(vec!["brctl", "setageing", &self.name, &time.to_string()]).map(|_| ())
	}

	fn show(&self) -> Result<String, CommandError> {
		debug!("Fetching details for Bridge: {}", self.name);
		let output = CommandExecutor::run_command(vec!["brctl", "show", &self.name])?;
		let stdout = String::from_utf8_lossy(&output.stdout).to_string();
		Ok(stdout)
	}

	pub fn get_id(&self) -> Result<String, CommandError> {
		debug!("Getting ID for Bridge: {}", self.name);
		let details = self.show()?;

		if let Some((_, _, bridge_id)) = regex_captures!(r"^\s*(\S+)\s+(\S+)\s+", &details.lines().nth(1).unwrap_or(""))
		{
			return Ok(bridge_id.to_string());
		}

		error!("Failed to get bridge ID for {}", self.name);
		Err(CommandError::ExecutionError("Failed to get bridge ID".into()))
	}

	pub fn get_interfaces(&self) -> Result<Vec<String>, CommandError> {
		debug!("Getting interfaces for Bridge: {}", self.name);
		let details = self.show()?;
		let mut interfaces = Vec::new();

		for line in details.lines() {
			if let Some((_, interface)) = regex_captures!(r"^\s*\S+\s+\S+\s+\S+\s+(\S+)", line) {
				interfaces.push(interface.to_string());
			}
		}

		Ok(interfaces)
	}

	pub fn get_stp(&self) -> Result<bool, CommandError> {
		debug!("Checking STP state for Bridge: {}", self.name);
		let details = self.show()?;

		if let Some((_, stp_state)) = regex_captures!(r"^\s*\S+\s+\S+\s+(\S+)", details.lines().nth(1).unwrap_or("")) {
			return Ok(match stp_state {
				"yes" => true,
				"no" => false,
				_ => {
					error!("Unknown STP state for Bridge: {}", self.name);
					unimplemented!("Unknown STP state {}", stp_state)
				}
			});
		}

		error!("Failed to determine STP state for Bridge: {}", self.name);
		Err(CommandError::ExecutionError("Failed to determine STP state".into()))
	}
}
