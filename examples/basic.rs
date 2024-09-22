use brctl::{BridgeController, CommandError};

fn main() -> Result<(), CommandError> {
	env_logger::init();

	match BridgeController::check() {
		Ok((brctl_version, interfaces)) => {
			println!("brctl version: {}", brctl_version);
			println!("Interfaces: {:?}", interfaces);

			println!("The system is ready for bridge operations!")
		}
		Err(e) => eprintln!("The system is not ready for bridge operations: {}", e),
	}

	println!();

	if let Some(mybridge) = BridgeController::get_bridge("mybridge")? {
		mybridge.delete()?;
	}

	let mybridge = BridgeController::create_bridge("mybridge")?;

	mybridge.add_interface("enp9s0")?;

	let bridges = BridgeController::list_bridges()?;
	println!("Available bridges:");
	for b in bridges {
		println!("  - {}", b);
	}

	println!();

	println!("{}:", mybridge.get_name());
	println!("  Interfaces:");
	for i in mybridge.get_interfaces()? {
		println!("    - {}", i);
	}

	println!("  ID: {}", mybridge.get_id()?);
	println!("  STP: {}", mybridge.get_stp()?);

	println!();

	let mybridge = BridgeController::get_bridge("mybridge")?;
	assert!(mybridge.is_some(), "Bridge not found");
	mybridge.expect("Bridge not found").delete()?;

	let mybridge = BridgeController::get_bridge("mybridge")?;
	assert!(mybridge.is_none(), "Bridge not deleted");

	println!("Bridge deleted successfully");
	Ok(())
}
