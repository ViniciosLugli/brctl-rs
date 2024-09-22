use crate::{Bridge, CommandError, CommandExecutor};
use lazy_regex::regex_captures;
use log::{debug, info};

pub struct BridgeController;

impl BridgeController {
	pub fn check() -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
		if !cfg!(target_os = "linux") {
			debug!("Only Linux is supported!, current OS: {}", std::env::consts::OS);
			return Err("Only Linux is supported!".into());
		} else {
			debug!("Current OS: {}", std::env::consts::OS);
		}

		info!("Checking dependencies for brctl and ip...");

		let brctl_version_output = CommandExecutor::run_command(vec!["brctl", "--version"])?;
		let brctl_version = String::from_utf8_lossy(&brctl_version_output.stdout);

		let version = if let Some((_, version)) = regex_captures!(r"bridge-utils, (\d+\.\d+)", &brctl_version) {
			version.to_string()
		} else {
			return Err("Failed to parse brctl version".into());
		};

		let ip_link_output = CommandExecutor::run_command(vec!["ip", "link"])?;
		let ip_link = String::from_utf8_lossy(&ip_link_output.stdout);

		let mut interfaces = Vec::new();
		for line in ip_link.lines() {
			if let Some((_, interface_name)) = regex_captures!(r"^\d+: ([^:]+):", line) {
				interfaces.push(interface_name.to_string());
			}
		}

		debug!("brctl version: {}", version);
		debug!("Interfaces: {:?}", interfaces);

		info!("Dependencies are satisfied! brctl and ip are available");

		Ok((version, interfaces))
	}

	pub fn create_bridge(name: &str) -> Result<Bridge, CommandError> {
		info!("Initializing bridge: {}", name);
		CommandExecutor::run_command(vec!["brctl", "addbr", name])?;
		CommandExecutor::run_command(vec!["ip", "link", "set", "dev", name, "up"])?;
		Ok(Bridge::new(name))
	}

	pub fn delete_bridge(name: &str) -> Result<(), CommandError> {
		info!("Deleting bridge: {}", name);
		CommandExecutor::run_command(vec!["ip", "link", "set", "dev", name, "down"])?;
		CommandExecutor::run_command(vec!["brctl", "delbr", name]).map(|_| ())
	}

	pub fn list_bridges() -> Result<Vec<Bridge>, CommandError> {
		debug!("Listing all bridges");
		let output = CommandExecutor::run_command(vec!["brctl", "show"])?;
		let stdout = String::from_utf8_lossy(&output.stdout);

		let bridges: Vec<Bridge> = stdout
			.lines()
			.filter_map(|line| {
				if let Some((_, bridge_name)) = regex_captures!(r"^\s*(\S+)", line) {
					Some(Bridge::new(bridge_name))
				} else {
					None
				}
			})
			.collect();

		info!("Found {} bridges", bridges.len());
		Ok(bridges)
	}

	pub fn get_bridge(name: &str) -> Result<Option<Bridge>, CommandError> {
		debug!("Getting bridge: {}", name);
		let bridges = Self::list_bridges()?;
		Ok(bridges.into_iter().find(|b| b.get_name() == name))
	}

	pub fn get_bridge_by_id(id: u32) -> Result<Option<Bridge>, CommandError> {
		debug!("Getting bridge by ID: {}", id);
		let bridges = Self::list_bridges()?;
		Ok(bridges.into_iter().find(|b| b.get_id().unwrap() == id.to_string()))
	}

	pub fn check_bridge_exists(name: &str) -> Result<bool, CommandError> {
		debug!("Checking if bridge exists: {}", name);
		let bridges = Self::list_bridges()?;
		Ok(bridges.into_iter().any(|b| b.get_name() == name))
	}
}
